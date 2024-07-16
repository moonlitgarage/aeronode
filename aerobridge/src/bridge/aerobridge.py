from backends.webots import Webots
from bridge.drone import AbstractDrone
from bridge.controller import Channel, ChannelId, ControlInput
from enum import Enum
from xmlrpc.server import SimpleXMLRPCServer
from xmlrpc.server import SimpleXMLRPCRequestHandler
import json
import threading
import time
from typing import List

class SupportedBackends(Enum):
    Webots = 1
    Xplane = 2

def initBackend(backend: SupportedBackends) -> AbstractDrone:
    if backend == SupportedBackends.Webots:
        return Webots()
    else:
        raise NotImplementedError()

class RpcServer:
    def __init__(self):
        self.drone = Webots()

    def start(self):
        # self.drone = initBackend(SupportedBackends.Webots)
        self.drone_thread = threading.Thread(target=self.drone.run)
        self.drone_thread.start()
        return "Ok"

    def get_sensor_data(self):
        sensor_data = self.drone.get_sensor_data()
        return sensor_data
        # return json.dumps({
        #     'imu': {
        #         'roll': sensor_data.imu.roll,
        #         'pitch': sensor_data.imu.pitch,
        #         'yaw': sensor_data.imu.yaw
        #     },
        #     'altitude': sensor_data.altitude
        # })

    def handle_control_input(self, ci_json):
        # control_input_dict = json.loads(control_input_json)
        # if control_input_dict is None:
        #     print("Received None control input")
        #     return
        # channels = [
        #     Channel(ChannelId.LEFT_X, control_input_dict['channels'][0]),
        #     Channel(ChannelId.LEFT_Y, control_input_dict['channels'][1]),
        #     Channel(ChannelId.RIGHT_X, control_input_dict['channels'][2]),
        #     Channel(ChannelId.RIGHT_Y, control_input_dict['channels'][3])
        # ]
        # control_input = ControlInput(
        #     channels,
        #     switch_1=control_input_dict['switch_1'],
        #     switch_2=control_input_dict['switch_2']
        # )

        ci = ControlInput.from_json(ci_json)
        print("CI---->", ci)
        self.drone.handleControlInput(ci)
        return "HANDLING"

    def stop(self):
        self.drone.stop()
        self.drone_thread.join()
        print("Drone backend stopped")

class RequestHandler(SimpleXMLRPCRequestHandler):
    rpc_paths = ('/RPC2',)
