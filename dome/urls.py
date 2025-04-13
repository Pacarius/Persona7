from channels.routing import URLRouter
from django.urls import path
from .settings import settings_view
from .client import client_view
from .test import test_view
from django.views.generic import RedirectView  # Import RedirectView

urlpatterns = [
    path('', RedirectView.as_view(url='client/', permanent=True)),  # Redirect root of 'dome/' to 'client/'
    path('client/', client_view.client_view, name='client_view'),
    path('settings/', settings_view.settings_view, name='settings_view'),
    path('test/', test_view.test_view, name='test_view'),
]
router = URLRouter(
    [
        path('ws/main', test.testConsumer.as_asgi())
    ]
)