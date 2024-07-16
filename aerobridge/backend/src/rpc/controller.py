from dataclasses import dataclass, field
from enum import Enum, auto
import json
from typing import List

class ChannelId(Enum):
    LeftY = auto()
    LeftX = auto()
    RightY = auto()
    RightX = auto()

@dataclass
class Channel:
    channel_id: ChannelId
    channel_val: int
    min: int
    max: int

    def to_json(self):
        return {
            "channel_id": self.channel_id.name,
            "channel_val": self.channel_val,
            "min": self.min,
            "max": self.max
        }

    @classmethod
    def from_json(cls, data):
        return cls(
            channel_id=ChannelId[data["channel_id"]],
            channel_val=data["channel_val"],
            min=data["min"],
            max=data["max"]
        )

@dataclass
class ControlInput:
    channels: List[Channel] = field(default_factory=list)
    switch_1: bool = False
    switch_2: bool = False

    def to_json(self):
        return {
            "channels": [channel.to_json() for channel in self.channels],
            "switch_1": self.switch_1,
            "switch_2": self.switch_2
        }

    @classmethod
    def from_json(cls, data):
        return cls(
            channels=[Channel.from_json(channel) for channel in data["channels"]],
            switch_1=data["switch_1"],
            switch_2=data["switch_2"]
        )


def create_control_input(channel_values: List[float], switch_1: bool, switch_2: bool) -> ControlInput:
    if len(channel_values) != 4:
        raise ValueError("There must be exactly 4 channel values")
    
    channels = [
        Channel(ChannelId.LeftY, channel_values[0], 0, 100),
        Channel(ChannelId.LeftX, channel_values[1], 0, 100),
        Channel(ChannelId.RightY, channel_values[2], 0, 100),
        Channel(ChannelId.RightX, channel_values[3], 0, 100)
    ]
    
    return ControlInput(channels=channels, switch_1=switch_1, switch_2=switch_2)
