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
            print(f"Client already initialized in this process. Returning existing instance {id(self)}")
            return
            
        self.initialized = True
        self.ip = ip
        self.port = int(port)
        self.client_socket = None
        self.channel_layer = get_channel_layer()
        self.group_name = "relay"
        self.message_queue = asyncio.Queue()  # Queue for WebSocket messages
        self.running = False
        self.init_message = None
        self.is_primary_client = False
        
        # Detailed debugging information
        process_id = os.getpid()
        thread_id = threading.get_ident()
        print(f"**** NEW CLIENT INITIALIZED ****")
        print(f"Process ID: {process_id}")
        print(f"Thread ID: {thread_id}")
        print(f"Instance ID: {id(self)}")
        print(f"Working directory: {os.getcwd()}")
        print("**** END CLIENT INITIALIZATION ****")

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
                print(f"Process {os.getpid()} acquired primary client lock")
                return True
            except IOError:
                # Another process has the lock
                print(f"Process {os.getpid()} failed to acquire lock - another process is primary")
                return False
                
        except Exception as e:
            print(f"Error acquiring lock: {e}")
            return False

    async def connect(self):
        """
        Connect to the server using TCP only if this is the primary client
        """
        # Check if we're already connected
        if self.running and self.client_socket:
            print(f"Already connected to {self.ip}:{self.port} (Instance ID: {id(self)})")
            return
        
        # Only the primary client should connect to TCP server
        if not self.is_primary_client and not self.acquire_primary_client_lock():
            print(f"This is not the primary client (PID: {os.getpid()}). Not connecting to TCP server.")
            return
            
        try:
            print(f"Primary client (PID: {os.getpid()}) connecting to TCP server")
            self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.client_socket.setblocking(False)
            await asyncio.get_event_loop().sock_connect(self.client_socket, (self.ip, self.port))
            self.running = True
            print(f"Connected to {self.ip}:{self.port} (PID: {os.getpid()})")
        except Exception as e:
            print(f"Failed to connect to {self.ip}:{self.port} - {e} (PID: {os.getpid()})")

    async def send_message(self, message):
        """
        Send a message to the server.
        """
        if self.client_socket:
            try:
                await asyncio.get_event_loop().sock_sendall(self.client_socket, (message + "\n").encode('utf-8'))
                print("Message sent successfully.")
            except Exception as e:
                print(f"Failed to send message - {e}")
        else:
            print("Client is not connected to the server.")

    async def receive_message(self, buffer_size=8192):
        """
        Continuously receive messages from the server.
        """
        while self.running:
            if not self.client_socket:
                print("Client is not connected to the server.")
                await asyncio.sleep(5)  # Wait longer before retrying
                continue
                
            try:
                data = await asyncio.get_event_loop().sock_recv(self.client_socket, buffer_size)
                if not data:  # Connection closed by server
                    print("Connection closed by server")
                    self.running = False
                    break
                    
                if self.init_message == None:
                    self.init_message = data
                message = data.decode('utf-8')
                print(f"Message received from TCP server: {message}")
                
                # Relay the message to WebSocket clients via the channel layer
                if self.channel_layer is not None:
                    await self.channel_layer.group_send(
                        self.group_name,
                        {
                            "type": "outbound",  # Mark the message as outbound
                            "message": message,
                        }
                    )
                else:
                    print("Channel layer is not configured. Unable to relay message.")
            except ConnectionError:
                print("Connection lost to the server")
                self.running = False
                break
            except Exception as e:
                print(f"Failed to receive message - {e}")
                await asyncio.sleep(1)  # Wait before retrying

    async def relay_websocket_to_tcp(self):
        """
        Continuously listen for messages from the queue and send them to the TCP server.
        """
        while self.running:
            try:
                # Use timeout to prevent blocking forever
                message = await asyncio.wait_for(self.message_queue.get(), timeout=1.0)
                print(f"Relaying WebSocket message to TCP server: {message}")
                await self.send_message(message)
            except asyncio.TimeoutError:
                # Just continue the loop
                pass
            except Exception as e:
                print(f"Error in relay_websocket_to_tcp: {e}")
                await asyncio.sleep(1)

    async def run(self):
        """
        Run the client to handle both sending and receiving messages.
        """
        # Only try to connect if not already running
        if not self.running:
            await self.connect()
            
            # Only start the async tasks if we successfully connected
            if self.client_socket and self.is_primary_client:
                try:
                    print(f"Starting client tasks in process {os.getpid()}")
                    await asyncio.gather(
                        self.receive_message(),
                        self.relay_websocket_to_tcp(),
                    )
                except Exception as e:
                    print(f"Error in client run: {e}")
                finally:
                    self.running = False
                    self.close_connection()
            else:
                print(f"Process {os.getpid()} is not running TCP client tasks")

    def close_connection(self):
        """
        Close the connection to the server.
        """
        if self.client_socket:
            try:
                self.client_socket.close()
                print("Connection closed successfully.")
            except Exception as e:
                print(f"Failed to close connection - {e}")
        else:
            print("Client is not connected to the server.")

# Create a shared instance of the Client
client_instance = Client()

# For debugging purposes
def get_client():
    """Returns the singleton client instance"""
    return client_instance
