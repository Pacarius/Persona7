from django.apps import AppConfig
import asyncio
import os
from threading import Thread
from .client.client import get_client

class DomeConfig(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = 'dome'
    
    # Track if we've already started the client in this process
    client_started = False

    def ready(self):
        """
        Start the TCP client when the Django app is ready.
        """
        # Skip starting client in auto-reload process
        if os.environ.get('RUN_MAIN') == 'true':
            print("Skipping client start in Django auto-reloader process")
            return
            
        # Skip if client is already started in this process
        if DomeConfig.client_started:
            print("Client already started in this process")
            return
            
        DomeConfig.client_started = True
        print(f"Starting client in process {os.getpid()}...")
        
        def start_client():
            try:
                loop = asyncio.new_event_loop()
                asyncio.set_event_loop(loop)
                loop.run_until_complete(get_client().run())
            except Exception as e:
                print(f"Client thread error: {e}")
                traceback.print_exc()
        
        # Run the client in a separate thread
        client_thread = Thread(target=start_client, daemon=True)
        client_thread.name = "TCPClientThread"
        client_thread.start()
        print(f"Client thread started: {client_thread.name} in process {os.getpid()}")
