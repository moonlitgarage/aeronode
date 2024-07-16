from simulations.webots import Webots
from rpc.controller import ControlInput
from xmlrpc.server import SimpleXMLRPCServer
from xmlrpc.server import SimpleXMLRPCRequestHandler
import threading
from typing import List

class RpcServer:
    def __init__(self):
        self.drone = Webots()

    def start(self):
        self.drone_thread = threading.Thread(target=self.drone.run)
        self.drone_thread.start()
        return "STARTED"

    def get_sensor_data(self):
        sensor_data = self.drone.get_sensor_data()
        sensor_data_json = sensor_data.to_json()
        return sensor_data_json

    def handle_control_input(self, ci_json):
        ci = ControlInput.from_json(ci_json)
        self.drone.handleControlInput(ci)
        return "HANDLING"

    def stop(self):
        self.drone.stop()
        self.drone_thread.join()
        print("Drone backend stopped")

class RequestHandler(SimpleXMLRPCRequestHandler):
    rpc_paths = ('/RPC2',)


def start_rpc_server():
    drone_backend = RpcServer()
    server = SimpleXMLRPCServer(("localhost", 8000), requestHandler=RequestHandler)
    server.register_instance(drone_backend)
    print("RPC server running on port 8000...")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("Server interrupted")
    finally:
        drone_backend.stop()
        print("Server stopped")
