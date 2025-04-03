import socket

class Client:
    def __init__(self, ip, port):
        """
        Initialize the client with the server's IP and port.
        """
        self.ip = ip
        self.port = int(port)
        self.client_socket = None

    def connect(self):
        """
        Connect to the server using TCP.
        """
        try:
            self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.client_socket.connect((self.ip, self.port))
            print(f"Connected to {self.ip}:{self.port}")
        except Exception as e:
            print(f"Failed to connect to {self.ip}:{self.port} - {e}")

    def send_message(self, message):
        """
        Send a message to the server.
        """
        if self.client_socket:
            try:
                self.client_socket.sendall(message.encode('utf-8'))
                print("Message sent successfully.")
            except Exception as e:
                print(f"Failed to send message - {e}")
        else:
            print("Client is not connected to the server.")

    def receive_message(self, buffer_size=1024):
        """
        Receive a message from the server.
        """
        if self.client_socket:
            try:
                response = self.client_socket.recv(buffer_size).decode('utf-8')
                print("Message received successfully.")
                return response
            except Exception as e:
                print(f"Failed to receive message - {e}")
        else:
            print("Client is not connected to the server.")
        return None

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