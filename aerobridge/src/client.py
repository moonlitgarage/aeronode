from bridge.node import Node
import time
import threading
import xmlrpc.client
import json
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class AeroBridge:
    def __init__(self):
        self.server = None
        self.aeronode = Node()
        self.telemetry_thread = None
        self.running = False
        self.connected = False

    def connect(self):
        try:
            self.server = xmlrpc.client.ServerProxy('http://localhost:8000', allow_none=True)
            self.server.get_sensor_data()  # Test the connection
            self.connected = True
            logger.info("Connected to RPC server successfully")
        except Exception as e:
            logger.error(f"Cannot establish connection: {e}")
            self.connected = False

    def print_telemetry(self):
        while self.running:
            # sensor_data_json = self.server.get_sensor_data()
            print("SENSOR DATA TELEM")
            # sensor_data = json.loads(sensor_data_json)
            # logger.info(f"Telemetry: Roll: {sensor_data['imu']['roll']}, Pitch: {sensor_data['imu']['pitch']}, Yaw: {sensor_data['imu']['yaw']}")
            # logger.info(f"           Altitude: {sensor_data['altitude']}")
            time.sleep(0.5)

    def run(self):
        self.running = True
        self.connect()

        x = self.server.start()
        print(x)

        self.telemetry_thread = threading.Thread(target=self.print_telemetry)
        self.telemetry_thread.start()

        try:
            while self.running:
                
                data = self.server.get_sensor_data()
                print("SENSOR DATA", data)
                self.aeronode.send_data(data)
                ci = self.aeronode.receive_control_input().to_json()
                print("CI", ci)
                self.server.handle_control_input(ci)
                    
                time.sleep(0.5)

        except KeyboardInterrupt:
            logger.info("Program interrupted")

        finally:
            self.running = False
            if self.telemetry_thread:
                self.telemetry_thread.join()
            logger.info("Program finished")

if __name__ == "__main__":
    aero_bridge = AeroBridge()
    aero_bridge.run()