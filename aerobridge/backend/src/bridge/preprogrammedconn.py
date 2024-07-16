from bridge.abstractconn import AbstractConn
from rpc.controller import ControlInput, create_control_input
from simulations.abstractdrone import SensorData

class PreProgrammed(AbstractConn):
    def __init__(self):
        self.current = 0
        self.inputs = [
            # forward
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),

            # yaw right
            create_control_input([50, 100, 50, 50], False, False),
            create_control_input([50, 100, 50, 50], False, False),
            create_control_input([50, 100, 50, 50], False, False),
            create_control_input([50, 100, 50, 50], False, False),
            create_control_input([50, 100, 50, 50], False, False),

            # forward
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),
            create_control_input([50, 50, 100, 50], False, False),

            # yaw left
            create_control_input([50, 0, 50, 50], False, False),
            create_control_input([50, 0, 50, 50], False, False),
            create_control_input([50, 0, 50, 50], False, False),
            create_control_input([50, 0, 50, 50], False, False),
            create_control_input([50, 0, 50, 50], False, False),

            # back
            create_control_input([50, 50, 0, 50], False, False),
            create_control_input([50, 50, 0, 50], False, False),
            create_control_input([50, 50, 0, 50], False, False),
            create_control_input([50, 50, 0, 50], False, False),
            create_control_input([50, 50, 0, 50], False, False),
            create_control_input([50, 50, 0, 50], False, False),
        ]

    def send(self, data: SensorData) -> ControlInput:
        self.current = (self.current + 1) % len(self.inputs)

    def read(self) -> ControlInput:
        return self.inputs[self.current]
    