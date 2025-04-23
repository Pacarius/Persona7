import json
import os
from PIL import Image, ImageDraw, ImageFont

def parse_map_json(file_path):
    """
    Parse the map.json file into a structured format.
    """
    with open(file_path, "r") as f:
        data = json.load(f)
    return data

def generate_image(parsed_data, tile_size=64, output_file="world_debug.png", assets_dir="assets/"):
    """
    Generate a 2D image using tiles from the assets directory, annotate map objects,
    and draw walls with holes around rooms.
    """
    regions = parsed_data["regions"]
    objects = parsed_data["objects"]

    # Determine the overall size of the image
    max_width = max(region["position"]["x"] + region["size"]["width"] for region in regions)
    max_height = max(region["position"]["y"] + region["size"]["height"] for region in regions)
    image_width = max_width * tile_size
    image_height = max_height * tile_size

    # Create a blank image
    img = Image.new("RGBA", (image_width, image_height), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Load tile images from the assets directory
    tile_images = {}
    for filename in os.listdir(assets_dir):
        if filename.endswith(".png"):
            tile_name = os.path.splitext(filename)[0].lower()
            tile_images[tile_name] = Image.open(os.path.join(assets_dir, filename)).resize((tile_size, tile_size))

    # Collect all holes from all rooms into a global set
    global_holes = set()
    for region in regions:
        for room in region["rooms"]:
            global_holes.update((hole["x"], hole["y"]) for hole in room.get("holes", []))

    # Draw each region and its rooms
    for region in regions:
        region_name = region["name"].lower()
        tile_image = tile_images.get(region_name, None)
        for room in region["rooms"]:
            room_x, room_y = room["position"]["x"], room["position"]["y"]
            room_w, room_h = room["size"]["width"], room["size"]["height"]

            # Draw the room tiles
            for i in range(room_w):
                for j in range(room_h):
                    tile_x = (room_x + i) * tile_size
                    tile_y = (room_y + j) * tile_size
                    if tile_image:
                        img.paste(tile_image, (tile_x, tile_y))
                    else:
                        draw.rectangle(
                            [tile_x, tile_y, tile_x + tile_size, tile_y + tile_size],
                            fill="grey",
                            outline="black"
                        )

                       # Draw the room tiles
            # Draw walls around the room as full tiles
            if region_name != "street":
                for i in range(room_w):
                    for j in range(room_h):
                        # Only process perimeter tiles
                        if i == 0 or i == room_w - 1 or j == 0 or j == room_h - 1:
                            tile_x = (room_x + i) * tile_size
                            tile_y = (room_y + j) * tile_size

                            # Skip drawing walls for holed tiles
                            if (room_x + i, room_y + j) in global_holes:
                                continue

                            # Draw the wall
                            draw.rectangle(
                                [tile_x, tile_y, tile_x + tile_size, tile_y + tile_size],
                                fill="black"
                            )
    # Draw map objects as annotated grey rectangles
    # font = ImageFont.load_default()  # Use a default font for annotations
    # font = ImageFont.truetype("Arial.ttf", size=16)  # Load a TrueType font with a larger size
    font = ImageFont.truetype("/usr/share/fonts/TTF/Hack-Bold.ttf", size=16)
    for obj in objects:
        obj_x, obj_y = obj["pos"]["x"], obj["pos"]["y"]
        obj_w, obj_h = obj["size"]["height"], obj["size"]["width"]
        draw.rectangle(
            [
                obj_x * tile_size,
                obj_y * tile_size,
                (obj_x + obj_w) * tile_size,
                (obj_y + obj_h) * tile_size
            ],
            fill="black",
            outline="grey",
            width=16
        )
        draw.text(
            (obj_x * tile_size, obj_y * tile_size),
            obj["name"],
            fill="white",
            font=font,
            anchor="lt"  # Adjust text alignment if needed
        )
            # font_size=16
    # Save the image
    img.save(output_file)
    print(f"World image saved to {output_file}")

# Example usage
map_file = "map.json"  # Path to your map.json file
parsed_data = parse_map_json(map_file)
generate_image(parsed_data, assets_dir="assets/")
