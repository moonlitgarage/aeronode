import logging
from bridge.abstractconn import AbstractConn
from bridge.preprogrammedconn import PreProgrammed
from bridge.serialwrapper import SerialWrapper
from rpc.controller import ControlInput
from simulations.abstractdrone import SensorData

class Node:
    def __init__(self):
        self.conn: AbstractConn = None
        try:
            self.conn = SerialWrapper()
        except:
            self.conn = PreProgrammed()
            logging.error("cannot establish connection. will run preprogrammed code")


    def send_data(self, data: SensorData):
        self.conn.send(data)

    def receive_control_input(self) -> ControlInput:
        ci = self.conn.read()
        return ci