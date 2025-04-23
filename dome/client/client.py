import asyncio
import json
import socket
from channels.layers import get_channel_layer
from asgiref.sync import async_to_sync

class Client:
    def __init__(self, ip, port):
        """
        Initialize the client with the server's IP and port.
        """
        self.ip = ip
        self.port = int(port)
        self.client_socket = None
        self.channel_layer = get_channel_layer()
        self.group_name = "relay"
        self.message_queue = asyncio.Queue()  # Queue for WebSocket messages
        self.running = False
        self.init_message = None

    async def connect(self):
        """
        Connect to the server using TCP.
        """
        try:
            self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.client_socket.setblocking(False)  # Make the socket non-blocking
            await asyncio.get_event_loop().sock_connect(self.client_socket, (self.ip, self.port))
            self.running = True
            print(f"Connected to {self.ip}:{self.port}")
        except Exception as e:
            print(f"Failed to connect to {self.ip}:{self.port} - {e}")

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
        while True:
            if self.client_socket:
                try:
                    data = await asyncio.get_event_loop().sock_recv(self.client_socket, buffer_size)
                    if data:
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
                except Exception as e:
                    print(f"Failed to receive message - {e}")
            else:
                print("Client is not connected to the server.")
                await asyncio.sleep(1)  # Wait before retrying

    async def relay_websocket_to_tcp(self):
        """
        Continuously listen for messages from the queue and send them to the TCP server.
        """
        while True:
            message = await self.message_queue.get()
            print(f"Relaying WebSocket message to TCP server: {message}")
            await self.send_message(message)

    async def run(self):
        """
        Run the client to handle both sending and receiving messages.
        """
        if not self.running:
            await self.connect()
            if self.client_socket:
                await asyncio.gather(
                    self.receive_message(),  # Continuously receive messages from the TCP server
                    self.relay_websocket_to_tcp(),  # Continuously relay WebSocket messages to the TCP server
                )

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
client_instance = Client("127.0.0.1", 1234)
