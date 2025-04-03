from django.db import models

class Setting(models.Model):
    key = models.CharField(max_length=255, unique=True)  # Unique key for the setting
    value = models.TextField()  # Value of the setting

    def __str__(self):
        return f"{self.key}: {self.value}"