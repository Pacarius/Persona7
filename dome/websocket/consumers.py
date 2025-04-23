import json
import logging
from channels.generic.websocket import AsyncWebsocketConsumer
from dome.client.client import get_client

logger = logging.getLogger('websocket_consumer')

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
        logger.info(f"New WebSocket client connected: {self.channel_name}")
        
        # Send initial message (if available)
        init_message = get_client().init_message
        if (init_message):
            if isinstance(init_message, bytes):
                init_message = init_message.decode('utf-8')
            
            await self.send(text_data=json.dumps({
                "message": init_message,
                "type": "status"
            }))
        
        logger.info(f"WebSocket connected and subscribed to group: {self.group_name}")

    async def disconnect(self, close_code):
        """
        Handle WebSocket disconnection.
        """
        await self.channel_layer.group_discard(
            self.group_name,
            self.channel_name
        )
        logger.info(f"WebSocket disconnected with code: {close_code}")

    async def receive(self, text_data):
        """
        Handle messages received from the WebSocket.
        """
        try:
            data = json.loads(text_data)
            message = data.get('message')

            if message:
                logger.info(f"WebSocket received message: {message}")
                await get_client().message_queue.put(message)
                
                # Echo confirmation is handled by the client after sending to TCP

            # Test direct sending
            await self.send(text_data=json.dumps({
                "message": "Direct test response",
                "type": "test"
            }))
            logger.info("Direct test message sent")
        except json.JSONDecodeError:
            logger.error(f"Invalid JSON received: {text_data}")
        except Exception as e:
            logger.error(f"Error processing WebSocket message: {e}")

    async def outbound(self, event):
        """
        Handle messages from TCP server via channel layer.
        """
        message = event["message"]
        message_type = event.get("message_type", "from_server")
        
        # Convert bytes to string if needed
        if isinstance(message, bytes):
            message = message.decode('utf-8')
        
        logger.info(f"Sending message to WebSocket: {message} (type: {message_type})")
        
        try:
            await self.send(text_data=json.dumps({
                "message": message,
                "type": message_type
            }))
            logger.info("Message sent to WebSocket successfully")
        except Exception as e:
            logger.error(f"Failed to send message to WebSocket: {e}")

    # This is just an alias for outbound to maintain backward compatibility
    async def chat_message(self, event):
        await self.outbound(event)

    async def chat_message(self, event):
        """Simplified version for testing"""
        logger.info(f"SIMPLIFIED chat_message called with: {event}")
        
        # Just try to send the raw message
        try:
            await self.send(text_data=json.dumps({
                "message": str(event.get("message", "")),
                "type": "test"
            }))
            logger.info("Simplified message sent successfully")
        except Exception as e:
            logger.error(f"Simplified send failed: {e}")