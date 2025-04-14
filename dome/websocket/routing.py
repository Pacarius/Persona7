from django.urls import path

from .consumers import ClientConsumer
# from . import consumers

router = [
    path('ws/home/', ClientConsumer.as_asgi()),
]