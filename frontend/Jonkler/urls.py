"""
URL configuration for Jonkler project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/5.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.urls import include, path
from django.views.generic import RedirectView
# from django.urls import include, path 
from debug_toolbar.toolbar import debug_toolbar_urls

# from .websockets import temp_view# Import RedirectView
# from .websockets import temp_view
urlpatterns = [
    path('', RedirectView.as_view(url='dome/', permanent=True)),  # Redirect root URL to 'dome/'
    path('dome/', include('dome.urls')),
    path('admin/', admin.site.urls),
    # path('phaser/', temp_view.TempView)
] + debug_toolbar_urls()
