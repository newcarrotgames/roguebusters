import requests
from PIL import Image
from io import BytesIO

# ASCII characters from darkest to lightest
ASCII_CHARS = "@%#*+=-:. "

def resize_image(image, new_width=100):
    """Resizes an image while maintaining aspect ratio."""
    width, height = image.size
    aspect_ratio = height / width
    new_height = int(new_width * aspect_ratio * 0.55)  # Adjusted for terminal aspect ratio
    return image.resize((new_width, new_height))

def grayscale_image(image):
    """Converts image to grayscale."""
    return image.convert("L")

def pixels_to_ascii(image):
    """Maps grayscale pixels to ASCII characters safely."""
    pixels = image.getdata()
    ascii_str = "".join(ASCII_CHARS[min(len(ASCII_CHARS) - 1, pixel // (256 // len(ASCII_CHARS)))] for pixel in pixels)
    return ascii_str

def image_to_ascii(image_path, output_width=100):
    """Downloads an image from URL and converts it to ASCII representation."""
    try:
        image = Image.open(image_path)
        image = resize_image(image, output_width)
        image = grayscale_image(image)
        ascii_str = pixels_to_ascii(image)

        # Format ASCII string into lines
        ascii_str_len = len(ascii_str)
        img_width = image.width
        ascii_image = "\n".join(ascii_str[i:(i+img_width)] for i in range(0, ascii_str_len, img_width))

        return ascii_image
    except Exception as e:
        return f"Error: {e}"

# Usage example
if __name__ == "__main__":
    ascii_art = image_to_ascii("images/usa.jpg", output_width=100)
    
    # Print ASCII Art to console
    print(ascii_art)

    # Save to a text file
    with open("ascii_art.txt", "w") as f:
        f.write(ascii_art)
