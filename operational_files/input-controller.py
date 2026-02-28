#
#
# sudo vim /etc/systemd/system/zenpi_input.service
#
#


import requests
from gpiozero import Button
from signal import pause

# Initialize the switch on GPIO 17
switch = Button(17)

# Using localhost (127.0.0.1) since the API is on the same Pi
BASE_URL = "http://127.0.0.1:4000"

def switch_turned_on():
    print("Switch is ON! Sending /start/ request...")
    try:
        # Send the POST request
        response = requests.post(f"{BASE_URL}/start/")
        print(f"Success! API replied with status code: {response.status_code}")
    except requests.exceptions.RequestException as e:
        print(f"Failed to connect to API: {e}")

def switch_turned_off():
    print("Switch is OFF! Sending /stop/ request...")
    try:
        # Send the POST request
        response = requests.post(f"{BASE_URL}/stop/")
        print(f"Success! API replied with status code: {response.status_code}")
    except requests.exceptions.RequestException as e:
        print(f"Failed to connect to API: {e}")

# Assign the functions to the switch events
switch.when_pressed = switch_turned_on
switch.when_released = switch_turned_off

print("Listening to GPIO 17... Toggle your switch!")
print("Press CTRL+C to exit.")

# Keep the script running
pause()