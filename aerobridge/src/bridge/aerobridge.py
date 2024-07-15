from backends.webots import Webots
from bridge.drone import AbstractDrone
from bridge.node import Node
import time
import threading
from enum import Enum

class SupportedBackends(Enum):
    Webots = 1
    Xplane = 2

def initBackend(backend: SupportedBackends) -> AbstractDrone:
    if backend == SupportedBackends.Webots:
        return Webots()
    else:
        raise NotImplementedError()

class AeroBridge:
    def __init__(self):
        self.drone = initBackend(SupportedBackends.Webots)
        self.drone_thread = threading.Thread(target=self.drone.run)
        self.telemetry_thread = threading.Thread(target=self.print_telemetry)
        self.aeronode = Node()
    
    def print_telemetry(self):
        while self.drone.running:
            sensor_data = self.drone.get_sensor_data()
            print(f"Telemetry: Roll: {sensor_data.imu.roll}, Pitch: {sensor_data.imu.pitch}, Yaw: {sensor_data.imu.yaw}")
            print(f"           Altitude: {sensor_data.altitude}")
            time.sleep(0.5)
    
    def run(self):
        self.drone_thread.start()
        self.telemetry_thread.start()

        try:
            while True:
                data = self.drone.get_sensor_data()
                self.aeronode.send_data(data)
                ci = self.aeronode.receive_control_input()
                self.drone.handleControlInput(ci)
                # self.print_telemetry()
                time.sleep(0.5)

        except KeyboardInterrupt:
            print("Program interrupted")

        finally:
            self.drone.stop()
            self.drone_thread.join()
            self.telemetry_thread.join()
            print("Program finished")
