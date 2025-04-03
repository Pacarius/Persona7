from django.urls import path

from .settings import settings_view
from .client import client_view

urlpatterns = [
    path('client/', client_view.client_view, name='client_view'),
    path('settings/', settings_view.settings_view, name='settings_view'),
]