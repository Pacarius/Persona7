from django.db import models

class AppSetting(models.Model):
    key = models.CharField(max_length=255, unique=True)  # Setting name
    value = models.TextField()  # Setting value (stored as text)

    def __str__(self):
        return self.key