import abc
from typing import Dict, Any
from dataclasses import dataclass
from rpc.controller import ControlInput

from dataclasses import dataclass, field
from enum import Enum, auto
import json
from typing import List

@dataclass
class ImuData:
    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def to_json(self):
        return {
            "x": self.x,
            "y": self.y,
            "z": self.z
        }

    @classmethod
    def from_json(cls, data):
        return cls(
            x=data["x"],
            y=data["y"],
            z=data["z"]
        )

@dataclass
class SensorData:
    imu: ImuData = field(default_factory=ImuData)
    altitude: float = 0.0

    def to_json(self):
        return {
            "imu": self.imu.to_json(),
            "altitude": self.altitude
        }

    @classmethod
    def from_json(cls, data):
        return cls(
            imu=ImuData.from_json(data["imu"]),
            altitude=data["altitude"]
        )

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