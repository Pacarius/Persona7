from django.apps import AppConfig
import asyncio
from threading import Thread
from .client.client import client_instance

class DomeConfig(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = 'dome'

    def ready(self):
        """
        Start the TCP client when the Django app is ready.
        """
        # client = Client("127.0.0.1", 1234)

        def start_client():
            asyncio.run(client_instance.run())

        # Run the client in a separate thread to avoid blocking Django
        Thread(target=start_client, daemon=True).start()
