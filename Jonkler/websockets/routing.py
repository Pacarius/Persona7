from django.urls import path

from .consumers import TestConsumer
# from . import consumers

router = [
    path('ws/main/', TestConsumer.as_asgi()),
]