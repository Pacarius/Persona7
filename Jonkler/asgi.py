"""
ASGI config for Jonkler project.

It exposes the ASGI callable as a module-level variable named ``application``.

For more information on this file, see
https://docs.djangoproject.com/en/5.1/howto/deployment/asgi/
"""

import os
from websockets.routing import router
from channels.routing import ProtocolTypeRouter
from channels.auth import AuthMiddlewareStack
from channels.routing import ProtocolTypeRouter, URLRouter
from django.core.asgi import get_asgi_application
# from .dome import 

os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'Jonkler.settings')

raw = get_asgi_application()
application = ProtocolTypeRouter({
    "http": raw,
    "websocket" : AuthMiddlewareStack(
        # dome.client.urls.router
        URLRouter(router)
    )
})