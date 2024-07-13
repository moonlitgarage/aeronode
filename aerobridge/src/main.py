from backends.webots import Webots
import serial
import time

# Replace '/dev/ttyUSB0' with the appropriate port for your operating system
# For Windows, it might be something like 'COM3' or 'COM4'
port = '/dev/tty.usbmodem11'
baud_rate = 115200  # Make sure this matches the Pico's baud rate

# Initialize serial connection
# ser = serial.Serial(port, baud_rate, timeout=1)

# def send_data(data):
#     ser.write(data.encode())  # Send data to Pico

# def receive_data():
#     data = ser.readline().decode().strip()  # Read data from Pico
#     return data
#       # Close the serial connection when done


if __name__ == "__main__":
    tokens = ["U1", "U1", "U1", "L1", "U1", "U1", "R1", "U1"]
    drone = Webots()
    try:
        while True:
            recvd_tokens = []
            for token in tokens:
                # send_data(token)
            #     print(f"Sent to Pico: {token}")
            # # time.sleep(1)  # Wait for a response

            #     received = receive_data()
            #     print(f"Received from Pico: {received}")
                recvd_tokens.append(token)
            
            drone.run(recvd_tokens)

    except KeyboardInterrupt:
        print("Program interrupted")

    finally:
        # ser.close()
        pass
