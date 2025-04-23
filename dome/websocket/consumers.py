import json
from channels.generic.websocket import AsyncWebsocketConsumer
from dome.client.client import client_instance  # Import the shared Client instance

class ClientConsumer(AsyncWebsocketConsumer):
    async def connect(self):
        """
        Handle WebSocket connection.
        """
        self.group_name = "relay"

        # Add the WebSocket connection to the group
        await self.channel_layer.group_add(
            self.group_name,
            self.channel_name
        )

        await self.accept()
        await self.chat_message({"message": client_instance.init_message})
        # await self.outbound(client_instance.init_message)
        print(f"WebSocket connected and subscribed to group: {self.group_name}")

    async def disconnect(self, close_code):
        """
        Handle WebSocket disconnection.
        """
        await self.channel_layer.group_discard(
            self.group_name,
            self.channel_name
        )
        print(f"WebSocket disconnected and unsubscribed from group: {self.group_name}")

    async def receive(self, text_data):
        """
        Handle messages received from the WebSocket and add them to the Client's queue.
        """
        data = json.loads(text_data)
        message = data.get('message')

        if message:
            print(f"Adding message to Client queue: {message}")
            await client_instance.message_queue.put(message)  # Add the message to the queue

    async def chat_message(self, event):
        """
        Handle outbound messages and send them to the WebSocket.
        """
        message = event["message"]

        # Decode the message if it's in bytes
        if isinstance(message, bytes):
            message = message.decode('utf-8')  # Decode bytes to string

        print(f"Sending outbound message to WebSocket: {message}")
        await self.send(text_data=json.dumps({
            "message": message,
        }))

    async def outbound(self, event):
        """
        Handle outbound messages and send them to the WebSocket.
        """
        message = event["message"]
        print(f"Sending outbound message to WebSocket: {message}")
        await self.send(text_data=json.dumps({
            "message": message,
        }))