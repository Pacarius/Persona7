from PIL import Image, ImageDraw
def generate_image(output_file="world.png"):
      image_width = 1000
      image_height = 1000 
      img = Image.new("RGB", (image_width, image_height), "white")
      draw = ImageDraw.Draw(img)
      # Define some colors for regions
      colors = ["orange", "blue", "green", "yellow", "purple", "orange"]
      # Draw each region and its rooms
    #   for region_index, region in enumerate(self.regions):
        #   print(region_color)
        #   region_color = colors[region_index % len(colors)]
        #   print(f"Using color {region_color} for region {region['name']}")  # Debugging output
        #   for room in region["rooms"]:
            #   room_x, room_y = room["position"]
            #   room_w, room_h = room["size"]
            #   Draw the room tiles
            #   for i in range(room_w):
                #   for j in range(room_h):
                    #   tile_x = (region["position"][0] + room_x + i) * tile_size
                    #   tile_y = (region["position"][1] + room_y + j) * tile_size
                    #   if (room_x + i, room_y + j) not in room["holes"]:  # Skip holes
                        #   print(f"Drawing tile at ({tile_x}, {tile_y}) with color {region_color}")  # Debugging output
                    #   draw.rectangle(
                        #   [tile_x, tile_y, tile_x + tile_size, tile_y + tile_size],
                        #   fill=region_color,
                        #   outline="black"
                    #   )
          # region_color = None
      # Save the image
      x, y = 0, 0
      for color in colors:
        draw.rectangle(
            (x, y, x + 100, y + 100),
            fill=color,
            outline="black"
        )
        x += 100
        y += 100

      img.save(output_file)
      print(f"World image saved to {output_file}")

generate_image(output_file="test.png")