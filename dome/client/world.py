import re
from PIL import Image, ImageDraw
'''
{'messages': ['Coordinates(200, 200)', '[Region { name: "Apartment", position: Coordinates(0, 0), size: Coordinates(32, 32), rooms: [Room { name: "Rm_001", position: Coordinates(0, 7), size: Coordinates(7, 14), holes: [Coordinates(6, 10), Coordinates(6, 11), Coordinates(6, 12)], region_name: Some("Apartment"), walled: true }, Room { name: "Rm_002", position: Coordinates(6, 0), size: Coordinates(10, 8), holes: [Coordinates(7, 7), Coordinates(8, 7)], region_name: Some("Apartment"), walled: true }, Room { name: "Living Room", position: Coordinates(6, 7), size: Coordinates(20, 14), holes: [Coordinates(7, 20), Coordinates(8, 20), Coordinates(11, 20), Coordinates(12, 20), Coordinates(16, 20), Coordinates(17, 20), Coordinates(18, 20), Coordinates(24, 20)], region_name: Some("Apartment"), walled: true }, Room { name: "Exit", position: Coordinates(6, 20), size: Coordinates(5, 12), holes: [Coordinates(7, 31), Coordinates(8, 31), Coordinates(9, 31)], region_name: Some("Apartment"), walled: true }, Room { name: "Storage_Closet", position: Coordinates(10, 20), size: Coordinates(4, 12), holes: [], region_name: Some("Apartment"), walled: true }, Room { name: "Library", position: Coordinates(13, 20), size: Coordinates(9, 12), holes: [], region_name: Some("Apartment"), walled: true }, Room { name: "Toilet", position: Coordinates(21, 20), size: Coordinates(5, 12), holes: [], region_name: Some("Apartment"), walled: true }] }, Region { name: "Street", position: Coordinates(6, 32), size: Coordinates(4, 19), rooms: [Room { name: "Street", position: Coordinates(6, 32), size: Coordinates(4, 19), holes: [], region_name: Some("Street"), walled: false }] }, Region { name: "Gym", position: Coordinates(10, 32), size: Coordinates(16, 8), rooms: [Room { name: "Boxing Ring", position: Coordinates(10, 31), size: Coordinates(16, 10), holes: [Coordinates(10, 38), Coordinates(10, 39)], region_name: Some("Gym"), walled: true }] }, Region { name: "Office", position: Coordinates(10, 40), size: Coordinates(16, 11), rooms: [Room { name: "Reception Area", position: Coordinates(10, 40), size: Coordinates(6, 11), holes: [Coordinates(10, 48), Coordinates(10, 49)], region_name: Some("Office"), walled: true }, Room { name: "Main Area", position: Coordinates(15, 40), size: Coordinates(6, 11), holes: [Coordinates(15, 41), Coordinates(15, 42)], region_name: Some("Office"), walled: true }, Room { name: "Office Area", position: Coordinates(20, 40), size: Coordinates(6, 6), holes: [Coordinates(20, 41), Coordinates(20, 42)], region_name: Some("Office"), walled: true }, Room { name: "Director\'s Office", position: Coordinates(20, 45), size: Coordinates(6, 6), holes: [Coordinates(23, 45), Coordinates(24, 45)], region_name: Some("Office"), walled: true }] }]', '[MapObject { name: "Bed", pos: Coordinates(13, 1), size: (1, 2) }, MapObject { name: "Yoga_Mat", pos: Coordinates(7, 1), size: (1, 2) }, MapObject { name: "Plant", pos: Coordinates(10, 1), size: (1, 1) }, MapObject { name: "Candles", pos: Coordinates(7, 3), size: (1, 1) }, MapObject { name: "Photo", pos: Coordinates(10, 6), size: (1, 1) }, MapObject { name: "Desk", pos: Coordinates(14, 3), size: (1, 1) }, MapObject { name: "Bookshelf", pos: Coordinates(12, 6), size: (1, 3) }, MapObject { name: "Couch", pos: Coordinates(13, 8), size: (1, 3) }, MapObject { name: "Bean Bags", pos: Coordinates(17, 8), size: (1, 3) }, MapObject { name: "TV", pos: Coordinates(14, 11), size: (1, 5) }, MapObject { name: "Philosophy Bookshelf", pos: Coordinates(14, 21), size: (1, 2) }, MapObject { name: "Science Bookshelf", pos: Coordinates(19, 21), size: (1, 2) }, MapObject { name: "Interstellar Bookshelf", pos: Coordinates(14, 24), size: (5, 1) }, MapObject { name: "Second Interstellar Bookshelf", pos: Coordinates(14, 30), size: (1, 4) }, MapObject { name: "Lamp", pos: Coordinates(20, 30), size: (1, 1) }, MapObject { name: "Chair 1", pos: Coordinates(20, 24), size: (1, 1) }, MapObject { name: "Chair 2", pos: Coordinates(20, 28), size: (1, 1) }, MapObject { name: "Coffee Table", pos: Coordinates(20, 26), size: (1, 1) }, MapObject { name: "Broom", pos: Coordinates(12, 30), size: (1, 1) }, MapObject { name: "Closet", pos: Coordinates(11, 26), size: (2, 1) }, MapObject { name: "Werid Stack of Boxes", pos: Coordinates(11, 23), size: (1, 1) }, MapObject { name: "Sink", pos: Coordinates(22, 21), size: (1, 1) }, MapObject { name: "Toilet", pos: Coordinates(22, 26), size: (1, 1) }, MapObject { name: "Shower", pos: Coordinates(22, 28), size: (3, 2) }, MapObject { name: "Reception Desk", pos: Coordinates(13, 44), size: (4, 1) }, MapObject { name: "Printer_1", pos: Coordinates(16, 44), size: (2, 1) }, MapObject { name: "Printer_2", pos: Coordinates(19, 44), size: (2, 1) }, MapObject { name: "Office Desk", pos: Coordinates(16, 48), size: (1, 2) }, MapObject { name: "Office Desk 1", pos: Coordinates(23, 41), size: (2, 1) }, MapObject { name: "Office Desk 2", pos: Coordinates(22, 44), size: (1, 1) }, MapObject { name: "DESK", pos: Coordinates(21, 48), size: (1, 3) }]']}
'''
class World:
    def __init__(self):
        self.regions = []  # Store parsed regions as an instance variable

    def parse_debug_string(self, debug_string):
        """
        Parse the Rust-like debug string into a structured format.
        """
        # Match each region, accounting for the outer brackets
        region_pattern = re.compile(
            r'Region \{ name: "(.*?)", position: Coordinates\((\d+), (\d+)\), size: Coordinates\((\d+), (\d+)\), rooms: (\[.*?\]) \}'
        )
        # Match each room within a region
        room_pattern = re.compile(
            r'Room \{ name: "(.*?)", position: Coordinates\((\d+), (\d+)\), size: Coordinates\((\d+), (\d+)\), holes: \[(.*?)\], region_name: Some\("(.*?)"\), walled: (true|false) \}'
        )
        # Match each hole within a room
        hole_pattern = re.compile(r'Coordinates\((\d+), (\d+)\)')

        # Remove the outer brackets if present
        debug_string = debug_string.strip("[]")

        for region_match in region_pattern.finditer(debug_string):
            region_name, rx, ry, rw, rh, rooms_str = region_match.groups()
            rooms = []
            for room_match in room_pattern.finditer(rooms_str):
                room_name, x, y, w, h, holes_str, region_name, walled = room_match.groups()
                holes = [
                    (int(hx), int(hy)) for hx, hy in hole_pattern.findall(holes_str)
                ]
                rooms.append({
                    "name": room_name,
                    "position": (int(x), int(y)),
                    "size": (int(w), int(h)),
                    "holes": holes,
                    "walled": walled == "true"
                })
            self.regions.append({
                "name": region_name,
                "position": (int(rx), int(ry)),
                "size": (int(rw), int(rh)),
                "rooms": rooms
            })
        print(f"Parsed regions: {self.regions}")  # Debugging output

    def generate_image(self, tile_size=16, output_file="world.png"):
        """
        Generate a 2D image based on the parsed regions and rooms.
        """
        # Determine the overall size of the image
        max_width = max(region["position"][0] + region["size"][0] for region in self.regions)
        max_height = max(region["position"][1] + region["size"][1] for region in self.regions)
        image_width = max_width * tile_size
        image_height = max_height * tile_size

        # Create a blank image
        img = Image.new("RGB", (image_width, image_height), "white")
        draw = ImageDraw.Draw(img)

        # Define some colors for regions
        colors = ["red", "blue", "green", "yellow", "purple", "orange"]

        # Draw each region and its rooms
        for region_index, region in enumerate(self.regions):
            # print(region_color)
            region_color = colors[region_index % len(colors)]
            print(f"Using color {region_color} for region {region['name']}")  # Debugging output

            for room in region["rooms"]:
                room_x, room_y = room["position"]
                room_w, room_h = room["size"]
                print(f"Drawing room: {room} at {room_x}, {room_y}. Size: {room_w}, {room_h}")

                # Draw the room tiles
                for i in range(room_w):
                    for j in range(room_h):
                        tile_x = (region["position"][0] + room_x + i) * tile_size
                        tile_y = (region["position"][1] + room_y + j) * tile_size
                        # if (room_x + i, room_y + j) not in room["holes"]:  # Skip holes
                            # print(f"Drawing tile at ({tile_x}, {tile_y}) with color {region_color}")  # Debugging output
                        draw.rectangle(
                            [tile_x, tile_y, tile_x + tile_size, tile_y + tile_size],
                            fill=region_color,
                            outline="black"
                        )
            # region_color = None

        # Save the image
        img.save(output_file)
        print(f"World image saved to {output_file}")

    def test(self):
        """
        Test the parsing and image generation functionality.
        """
        debug = '''"Apartment", position: Coordinates(0, 0), size: Coordinates(32, 32), rooms: [Room { name: "Rm_001", position: Coordinates(0, 7), size: Coordinates(7, 14), holes: [Coordinates(6, 10), Coordinates(6, 11), Coordinates(6, 12)], region_name: Some("Apartment"), walled: true }, Room { name: "Rm_002", position: Coordinates(6, 0), size: Coordinates(10, 8), holes: [Coordinates(7, 7), Coordinates(8, 7)], region_name: Some("Apartment"), walled: true }, Room { name: "Living Room", position: Coordinates(6, 7), size: Coordinates(20, 14), holes: [Coordinates(7, 20), Coordinates(8, 20), Coordinates(11, 20), Coordinates(12, 20), Coordinates(16, 20), Coordinates(17, 20), Coordinates(18, 20), Coordinates(24, 20)], region_name: Some("Apartment"), walled: true }, Room { name: "Exit", position: Coordinates(6, 20), size: Coordinates(5, 12), holes: [Coordinates(7, 31), Coordinates(8, 31), Coordinates(9, 31)], region_name: Some("Apartment"), walled: true }, Room { name: "Storage_Closet", position: Coordinates(10, 20), size: Coordinates(4, 12), holes: [], region_name: Some("Apartment"), walled: true }, Room { name: "Library", position: Coordinates(13, 20), size: Coordinates(9, 12), holes: [], region_name: Some("Apartment"), walled: true }, Room { name: "Toilet", position: Coordinates(21, 20), size: Coordinates(5, 12), holes: [], region_name: Some("Apartment"), walled: true }] }, Region { name: "Street", position: Coordinates(6, 32), size: Coordinates(4, 19), rooms: [Room { name: "Street", position: Coordinates(6, 32), size: Coordinates(4, 19), holes: [], region_name: Some("Street"), walled: false }] }, Region { name: "Gym", position: Coordinates(10, 32), size: Coordinates(16, 8), rooms: [Room { name: "Boxing Ring", position: Coordinates(10, 31), size: Coordinates(16, 10), holes: [Coordinates(10, 38), Coordinates(10, 39)], region_name: Some("Gym"), walled: true }] }, Region { name: "Office", position: Coordinates(10, 40), size: Coordinates(16, 11), rooms: [Room { name: "Reception Area", position: Coordinates(10, 40), size: Coordinates(6, 11), holes: [Coordinates(10, 48), Coordinates(10, 49)], region_name: Some("Office"), walled: true }, Room { name: "Main Area", position: Coordinates(15, 40), size: Coordinates(6, 11), holes: [Coordinates(15, 41), Coordinates(15, 42)], region_name: Some("Office"), walled: true }, Room { name: "Office Area", position: Coordinates(20, 40), size: Coordinates(6, 6), holes: [Coordinates(20, 41), Coordinates(20, 42)], region_name: Some("Office"), walled: true }, Room { name: "Director's Office", position: Coordinates(20, 45), size: Coordinates(6, 6), holes: [Coordinates(23, 45), Coordinates(24, 45)], region_name: Some("Office"), walled: true }] }]"'''
        self.parse_debug_string(debug)
        self.generate_image()


# Example usage
# world = World()
# world.test()
