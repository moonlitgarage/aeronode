import serial
from simulations.abstractdrone import SensorData
from rpc.controller import ControlInput
from bridge.abstractconn import AbstractConn

PORT = '/dev/tty.usbmodem11'
BAUD = 115200

class SerialWrapper(AbstractConn):
    def __init__(self):
        self.conn = serial.Serial(PORT, BAUD, timeout=1)

    def send(self, data: SensorData):
        self.conn.write(data.encode())

    def read(self) -> ControlInput:
        data = self.conn.readline().decode().strip()
        return data
