import asyncio
import json
import socket
from channels.layers import get_channel_layer
import os
import sys
import traceback
import threading
import tempfile
import fcntl
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger('tcp_client')

class SingletonMeta(type):
    """Metaclass for implementing the Singleton pattern within a process"""
    _instances = {}

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            instance = super().__call__(*args, **kwargs)
            cls._instances[cls] = instance
        return cls._instances[cls]

class Client(metaclass=SingletonMeta):
    def __init__(self, ip="127.0.0.1", port=1234):
        """
        Initialize the client with the server's IP and port.
        Only initializes once due to singleton pattern.
        """
        # Check if this instance has been initialized before
        if hasattr(self, 'initialized'):
            logger.info(f"Client already initialized in this process. Returning existing instance {id(self)}")
            return
            
        self.initialized = True
        self.ip = ip
        self.port = int(port)
        self.client_socket = None
        self.channel_layer = get_channel_layer()
        self.group_name = "relay"
        self.message_queue = asyncio.Queue()  # Queue for WebSocket messages
        self.running = False
        self.init_message = "Initializing connection..."
        self.is_primary_client = False
        self.message_buffer = []  # Buffer for messages when TCP is disconnected
        
        # Detailed debugging information
        process_id = os.getpid()
        thread_id = threading.get_ident()
        logger.info(f"**** NEW CLIENT INITIALIZED ****")
        logger.info(f"Process ID: {process_id}")
        logger.info(f"Thread ID: {thread_id}")
        logger.info(f"Instance ID: {id(self)}")
        logger.info(f"Working directory: {os.getcwd()}")
        
        # Check channel layer
        if self.channel_layer:
            logger.info(f"Channel layer initialized: {type(self.channel_layer)}")
        else:
            logger.error("Channel layer is NOT available! Check your Django settings.")
            
        logger.info("**** END CLIENT INITIALIZATION ****")

    def acquire_primary_client_lock(self):
        """
        Try to acquire a lock file to determine if this client should be the primary one
        that connects to the TCP server.
        """
        try:
            # Create a lock file in the temp directory
            lock_file = os.path.join(tempfile.gettempdir(), 'persona6_client.lock')
            self.lock_fd = open(lock_file, 'w')
            
            # Try to acquire an exclusive lock (non-blocking)
            try:
                fcntl.flock(self.lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                self.is_primary_client = True
                logger.info(f"Process {os.getpid()} acquired primary client lock")
                return True
            except IOError:
                # Another process has the lock
                logger.info(f"Process {os.getpid()} failed to acquire lock - another process is primary")
                return False
                
        except Exception as e:
            logger.error(f"Error acquiring lock: {e}")
            return False

    async def connect(self):
        """
        Connect to the server using TCP only if this is the primary client
        """
        # Check if we're already connected
        if self.running and self.client_socket:
            logger.info(f"Already connected to {self.ip}:{self.port}")
            return True

        # Only the primary client should connect to TCP server
        if not self.is_primary_client and not self.acquire_primary_client_lock():
            logger.info(f"This is not the primary client. Not connecting to TCP server.")
            return False

        try:
            logger.info(f"Primary client (PID: {os.getpid()}) connecting to TCP server")
            self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.client_socket.setblocking(False)
            await asyncio.get_event_loop().sock_connect(self.client_socket, (self.ip, self.port))
            self.running = True
            logger.info(f"Connected to {self.ip}:{self.port} (PID: {os.getpid()})")
            
            # Update init message
            self.init_message = f"Connected to {self.ip}:{self.port}"
            await self.broadcast_status()
            
            return True
        except Exception as e:
            logger.error(f"Failed to connect to {self.ip}:{self.port} - {e} (PID: {os.getpid()})")
            
            # Update init message
            self.init_message = f"Failed to connect: {e}"
            await self.broadcast_status()
            
            return False

    async def broadcast_status(self):
        """
        Broadcast client status to all WebSocket clients
        """
        if self.channel_layer:
            try:
                logger.info(f"Broadcasting status: {self.init_message}")
                await self.channel_layer.group_send(
                    self.group_name,
                    {
                        "type": "outbound",
                        "message": self.init_message,
                        "message_type": "status"
                    }
                )
                logger.info("Status broadcast sent to channel layer")
            except Exception as e:
                logger.error(f"Failed to broadcast status: {e}")
                logger.error(traceback.format_exc())

    async def relay_websocket_to_tcp(self):
        """
        Process messages from WebSocket and send them to TCP server
        """
        logger.info(f"Starting WebSocket to TCP relay (PID: {os.getpid()})")
        
        while self.running:
            try:
                # Get message from queue (with timeout)
                try:
                    message = await asyncio.wait_for(self.message_queue.get(), timeout=0.5)
                    logger.info(f"Processing message from queue: {message}")
                    
                    # Check if we need to reconnect
                    if not self.client_socket or not self.running:
                        logger.warning("TCP connection down, buffering message for later")
                        self.message_buffer.append(message)
                        self.message_queue.task_done()
                        continue
                    
                    # Send the message to the TCP server
                    message_with_newline = message + '\n'
                    await asyncio.get_event_loop().sock_sendall(
                        self.client_socket, 
                        message_with_newline.encode('utf-8')
                    )
                    logger.info(f"Message sent to TCP server: {message}")
                    
                    # Echo confirmation back to WebSocket
                    await self.channel_layer.group_send(
                        self.group_name,
                        {
                            "type": "outbound",
                            "message": f"Sent: {message}",
                            "message_type": "echo"
                        }
                    )
                    
                    # Mark task as done
                    self.message_queue.task_done()
                
                except asyncio.TimeoutError:
                    # No message in the queue, continue
                    await asyncio.sleep(0.1)
                    
            except Exception as e:
                logger.error(f"Error in WebSocket to TCP relay: {e}")
                logger.error(traceback.format_exc())
                # Wait a bit before retrying
                await asyncio.sleep(1)
        
        logger.info(f"WebSocket to TCP relay stopped (PID: {os.getpid()})")

    async def relay_tcp_to_websocket(self):
        """
        Receive messages from TCP server and relay them to WebSockets
        """
        logger.info(f"Starting TCP to WebSocket relay (PID: {os.getpid()})")
        
        while self.running:
            try:
                if not self.client_socket:
                    logger.warning("No active socket connection")
                    await asyncio.sleep(1)
                    continue
                    
                # Try to receive data from the socket
                try:
                    data = await asyncio.wait_for(
                        asyncio.get_event_loop().sock_recv(self.client_socket, 4096),
                        timeout=0.5
                    )
                    
                    if not data:  # Connection closed by server
                        logger.warning("Connection closed by server")
                        self.running = False
                        break
                        
                    # Process the received data
                    message = data.decode('utf-8').strip()
                    logger.info(f"Message from TCP server: {message}")
                    
                    # Send to WebSocket clients via channel layer
                    if self.channel_layer:
                        logger.info(f"Sending message to group {self.group_name} via channel layer")
                        
                        # Debug the channel layer object
                        logger.info(f"Channel layer type: {type(self.channel_layer)}")
                        
                        await self.channel_layer.group_send(
                            self.group_name,
                            {
                                "type": "outbound",
                                "message": message,
                                "message_type": "from_server"
                            }
                        )
                        logger.info("Message sent to channel layer")
                    else:
                        logger.error("Channel layer is not available")
                
                except asyncio.TimeoutError:
                    # No data received, continue
                    await asyncio.sleep(0.1)
                    
            except Exception as e:
                logger.error(f"Error in TCP to WebSocket relay: {e}")
                logger.error(traceback.format_exc())
                await asyncio.sleep(1)
        
        logger.info(f"TCP to WebSocket relay stopped (PID: {os.getpid()})")

    async def run(self):
        """
        Main entry point for the client. Connects to the server and starts processing messages.
        """
        connected = await self.connect()
        if not connected:
            logger.error("Failed to connect to TCP server")
            self.init_message = "Failed to connect to TCP server"
            await self.broadcast_status()
            return
        
        logger.info(f"Starting client tasks in process {os.getpid()}")
        
        try:
            # Create tasks for both relays
            ws_to_tcp_task = asyncio.create_task(self.relay_websocket_to_tcp())
            tcp_to_ws_task = asyncio.create_task(self.relay_tcp_to_websocket())
            
            # Wait for tasks to complete or any error
            done, pending = await asyncio.wait(
                [ws_to_tcp_task, tcp_to_ws_task], 
                return_when=asyncio.FIRST_COMPLETED
            )
            
            # Cancel any pending tasks
            for task in pending:
                task.cancel()
                
            # Raise any exceptions
            for task in done:
                # This will re-raise any exceptions from the task
                task.result()
                
        except Exception as e:
            logger.error(f"Error in client run: {e}")
            logger.error(traceback.format_exc())
        finally:
            self.close_connection()
            self.init_message = "TCP connection closed"
            await self.broadcast_status()

    def close_connection(self):
        """
        Close the connection to the server.
        """
        if self.client_socket:
            try:
                self.client_socket.close()
                self.client_socket = None
                self.running = False
                logger.info("Connection closed successfully.")
            except Exception as e:
                logger.error(f"Failed to close connection - {e}")
        else:
            logger.info("Client is not connected to the server.")

# Create a shared instance of the Client
client_instance = Client()

# For debugging purposes
def get_client():
    """Returns the singleton client instance"""
    return client_instance
