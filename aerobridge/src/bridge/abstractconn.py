import abc
from bridge.drone import SensorData
from bridge.controller import ControlInput

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