from django.shortcuts import render
from .client import Client
from dome.settings.models import Setting  # Import the Setting model
from .world import World

def client_view(request):
    # Fetch default IP and port from the database
    default_ip = Setting.objects.filter(key="ip").first()
    default_port = Setting.objects.filter(key="port").first()

    # Use default values if they exist, otherwise fallback to hardcoded defaults
    default_ip = default_ip.value if default_ip else "127.0.0.1"
    default_port = int(default_port.value) if default_port else 1234  # Ensure port is an integer

    message = None  # Initialize the message variable

    if request.method == "POST":
        try:
            client = Client(default_ip, default_port)
            client.connect()
            # Receive a message from the server
            messages = [client.receive_message() for i in range(3)]
            message = {"messages": messages}
            # print(messages[0])
            world = World()
            world.parse_debug_string(messages[1])
            world.generate_image()
            # World.test()
            client.close_connection()
        except Exception as e:
            message = f"Failed to connect: {e}"

    return render(request, "client_view.html", {"ip": default_ip, "port": default_port, "message": message})
