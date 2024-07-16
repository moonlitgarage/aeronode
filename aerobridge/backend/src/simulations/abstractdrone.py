import abc
from typing import Dict, Any
from dataclasses import dataclass
from rpc.controller import ControlInput

@dataclass
class ImuData:
    roll: float = 0
    pitch: float = 0
    yaw: float = 0

@dataclass
class SensorData:
    imu: ImuData
    altitude: float = 0

class AbstractDrone(abc.ABC):
    @abc.abstractmethod
    def __init__(self):
        pass

    @abc.abstractmethod
    def run(self):
        pass

    @abc.abstractmethod
    def get_sensor_data(self) -> SensorData:
        pass

    @abc.abstractmethod
    def handleControlInput(self, ci: ControlInput):
        pass