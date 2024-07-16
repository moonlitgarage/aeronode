from enum import Enum
from simulations.webots import Webots
from simulations.abstractdrone import AbstractDrone

class SupportedBackends(Enum):
    Webots = 1
    Xplane = 2

def initBackend(backend: SupportedBackends) -> AbstractDrone:
    if backend == SupportedBackends.Webots:
        return Webots()
    else:
        raise NotImplementedError()