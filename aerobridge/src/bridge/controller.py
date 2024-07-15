from enum import Enum
from dataclasses import dataclass, field
from typing import List

class ChannelId(Enum):
    LEFT_Y = 1
    LEFT_X = 2
    RIGHT_Y = 3
    RIGHT_X = 4

@dataclass
class Channel:
    channel_id: ChannelId
    _channel_val: float = 0.0
    _min: int = 0
    _max: int = 100

    @property
    def channel_val(self) -> float:
        return self._channel_val
    
    @channel_val.setter
    def channel_val(self, val: float):
        self._channel_val = max(min(val, self._max), self._min)

    def getScaled(self):
        return (self._channel_val / (0.5 * (self._max - self._min))) - 1.0

@dataclass
class ControlInput:
    channels: List[Channel] = field(default_factory=list)
    switch_1: bool = False
    switch_2: bool = False

    def __post_init__(self):
        if len(self.channels) != 4:
            raise ValueError("Controller must have exactly 4 channels")
        
    def get_channel(self, channel_id: ChannelId) -> Channel:
        for channel in self.channels:
            if channel.channel_id == channel_id:
                return channel
        raise ValueError(f"Channel with id {channel_id} not found")
    
    def get_channel_scaled(self, channel_id: ChannelId) -> float:
        return self.get_channel(channel_id).getScaled()

def create_control_input(channel_values: List[float], switch_1: bool, switch_2: bool) -> ControlInput:
    if len(channel_values) != 4:
        raise ValueError("There must be exactly 4 channel values")
    
    channels = [
        Channel(ChannelId.LEFT_Y, channel_values[0]),
        Channel(ChannelId.LEFT_X, channel_values[1]),
        Channel(ChannelId.RIGHT_Y, channel_values[2]),
        Channel(ChannelId.RIGHT_X, channel_values[3])
    ]
    
    return ControlInput(channels=channels, switch_1=switch_1, switch_2=switch_2)
