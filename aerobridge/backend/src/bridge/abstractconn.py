import abc
from simulations.abstractdrone import SensorData
from rpc.controller import ControlInput

class AbstractConn(abc.ABC):
    @abc.abstractmethod
    def __init__(self):
        pass

    @abc.abstractmethod
    def send(self, data: SensorData):
        pass

    @abc.abstractmethod
    def read(self) -> ControlInput:
        pass