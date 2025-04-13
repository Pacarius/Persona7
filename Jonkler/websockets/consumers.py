import json
from channels.generic.websocket import AsyncWebsocketConsumer

class TestConsumer(AsyncWebsocketConsumer):
    async def connect(self):
        await self.accept()

    async def disconnect(self, close_code):
        pass

    async def receive(self, text_data):
        data = json.loads(text_data)
        # Handle incoming messages and broadcast updates
        await self.send(text_data=json.dumps({
            'message': 'Update from server',
            # Add game state updates here
        }))