from django.shortcuts import render
from django.views.decorators.csrf import csrf_exempt
from .models import Setting

@csrf_exempt
def settings_view(request):
    message = None  # Initialize the message variable
    if request.method == "POST":
        # Update settings in the database
        for key in request.POST:
            value = request.POST[key]
            Setting.objects.update_or_create(key=key, defaults={"value": value})
        message = "Settings saved successfully!"  # Set the success message

    # Fetch all settings from the database
    settings = {setting.key: setting.value for setting in Setting.objects.all()}
    return render(request, "settings_form.html", {"settings": settings, "message": message})