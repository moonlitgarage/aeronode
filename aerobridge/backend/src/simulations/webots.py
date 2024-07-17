import sys
import os
import threading
import queue
from dataclasses import dataclass
from rpc.controller import create_control_input, ControlInput, ChannelId
from simulations.abstractdrone import AbstractDrone, SensorData, ImuData

webots_path = '/Applications/Webots.app/Contents/lib/controller/python'
os.environ['WEBOTS_HOME'] = '/Applications/Webots.app'
sys.path.append(webots_path)
from controller import Robot, Motor, Camera, Compass, GPS, Gyro, InertialUnit, Keyboard, LED

# Constants, empirically found
k_vertical_thrust = 68.5
k_vertical_offset = 0.6
k_vertical_p = 3.0
k_roll_p = 50.0
k_pitch_p = 30.0

def get_channel_scaled(ci: ControlInput, channel_id: ChannelId) -> float:
        for channel in ci.channels:
            if channel.channel_id == channel_id:
                return (channel.channel_val - channel.min) / (channel.max - channel.min) * 2 - 1
        raise ValueError(f"Channel {channel_id} not found")

@dataclass
class Disturbances:
    pitch: float = 0
    yaw: float = 0
    roll: float = 0

    @staticmethod
    def mapControlInput(ci: ControlInput) -> 'Disturbances':
        dist = Disturbances()
        dist.pitch = -2.0 * get_channel_scaled(ci, ChannelId.RightY)
        dist.yaw = -1.3 * get_channel_scaled(ci, ChannelId.LeftX)
        return dist

def clamp(value, low, high):
    return max(min(value, high), low)

class Webots(AbstractDrone):
    def __init__(self):
        super().__init__()
        # Initialize the Robot instance
        self.robot = Robot()
        self.timestep = int(self.robot.getBasicTimeStep())

        # Get and enable devices
        self.camera = self.robot.getDevice("camera")
        self.camera.enable(self.timestep)
        self.front_left_led = self.robot.getDevice("front left led")
        self.front_right_led = self.robot.getDevice("front right led")
        self.imu = self.robot.getDevice("inertial unit")
        self.imu.enable(self.timestep)
        self.gps = self.robot.getDevice("gps")
        self.gps.enable(self.timestep)
        self.compass = self.robot.getDevice("compass")
        self.compass.enable(self.timestep)
        self.gyro = self.robot.getDevice("gyro")
        self.gyro.enable(self.timestep)

        self.camera_roll_motor = self.robot.getDevice("camera roll")
        self.camera_pitch_motor = self.robot.getDevice("camera pitch")

        self.front_left_motor = self.robot.getDevice("front left propeller")
        self.front_right_motor = self.robot.getDevice("front right propeller")
        self.rear_left_motor = self.robot.getDevice("rear left propeller")
        self.rear_right_motor = self.robot.getDevice("rear right propeller")
        self.motors = [self.front_left_motor, self.front_right_motor, self.rear_left_motor, self.rear_right_motor]

        for motor in self.motors:
            motor.setPosition(float('inf'))
            motor.setVelocity(1.0)

        while self.robot.step(self.timestep) != -1:
            if self.robot.getTime() > 1.0:
                break

        self.target_altitude = 1.0
        self.ci = create_control_input([50, 50, 50, 50], False, False)
        self.ci_queue = queue.Queue()
        self.running = True

        self.sensor_data = SensorData(ImuData())        
        self.sensor_data_lock = threading.Lock()

    def get_sensor_data(self) -> SensorData:
        with self.sensor_data_lock:
            return self.sensor_data

    def handleControlInput(self, ci: ControlInput):
        self.ci_queue.put(ci)

    def compute_inputs(self):
        time = self.robot.getTime()

        with self.sensor_data_lock:
            roll, pitch, yaw = self.imu.getRollPitchYaw()
            altitude = self.gps.getValues()[2]
            self.sensor_data = SensorData(ImuData(roll, pitch, yaw), altitude)

        roll_velocity, pitch_velocity, _ = self.gyro.getValues()

        led_state = int(time) % 2
        self.front_left_led.set(led_state)
        self.front_right_led.set(not led_state)

        self.camera_roll_motor.setPosition(-0.115 * roll_velocity)
        self.camera_pitch_motor.setPosition(-0.1 * pitch_velocity)

        roll_disturbance = 0.0
        pitch_disturbance = 0.0
        yaw_disturbance = 0.0

        dist = Disturbances.mapControlInput(self.ci)
        pitch_disturbance = dist.pitch
        yaw_disturbance = dist.yaw

        roll_input = k_roll_p * clamp(self.sensor_data.imu.x, -1.0, 1.0) + roll_velocity + roll_disturbance
        pitch_input = k_pitch_p * clamp(self.sensor_data.imu.y, -1.0, 1.0) + pitch_velocity + pitch_disturbance
        yaw_input = yaw_disturbance
        clamped_difference_altitude = clamp(self.target_altitude - self.sensor_data.altitude + k_vertical_offset, -1.0, 1.0)
        vertical_input = k_vertical_p * (clamped_difference_altitude ** 3)

        front_left_motor_input = k_vertical_thrust + vertical_input - roll_input + pitch_input - yaw_input
        front_right_motor_input = k_vertical_thrust + vertical_input + roll_input + pitch_input + yaw_input
        rear_left_motor_input = k_vertical_thrust + vertical_input - roll_input - pitch_input + yaw_input
        rear_right_motor_input = k_vertical_thrust + vertical_input + roll_input - pitch_input - yaw_input

        self.front_left_motor.setVelocity(front_left_motor_input)
        self.front_right_motor.setVelocity(-front_right_motor_input)
        self.rear_left_motor.setVelocity(-rear_left_motor_input)
        self.rear_right_motor.setVelocity(rear_right_motor_input)

    def token_listener(self):
        while self.running:
            try:
                ci = self.ci_queue.get(timeout=1)
                self.ci = ci
            except queue.Empty:
                pass

    def run(self):
        token_thread = threading.Thread(target=self.token_listener)
        token_thread.start()

        while self.robot.step(self.timestep) != -1 and self.running:
            self.compute_inputs()

        self.running = False
        token_thread.join()
