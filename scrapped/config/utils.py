# filepath: /home/pacific/Documents/Persona6/config/utils.py
from .models import AppSetting
from django.core.cache import cache

def get_setting(key, default=None):
    # Check if the setting is cached
    cached_value = cache.get(f"app_setting_{key}")
    if cached_value is not None:
        return cached_value

    # Fetch from the database
    try:
        setting = AppSetting.objects.get(key=key)
        cache.set(f"app_setting_{key}", setting.value, timeout=3600)  # Cache for 1 hour
        return setting.value
    except AppSetting.DoesNotExist:
        return default