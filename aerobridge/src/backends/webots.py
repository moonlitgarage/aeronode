import sys
import math
import os

# webots path
webots_path = '/Applications/Webots.app/Contents/lib/controller/python'
os.environ['WEBOTS_HOME'] = '/Applications/Webots.app'
sys.path.append(webots_path)
from controller import Robot, Motor, Camera, Compass, GPS, Gyro, InertialUnit, Keyboard, LED


# Constants, empirically found
k_vertical_thrust = 68.5  # with this thrust, the drone lifts
k_vertical_offset = 0.6  # Vertical offset where the robot actually targets to stabilize itself
k_vertical_p = 3.0  # P constant of the vertical PID
k_roll_p = 50.0  # P constant of the roll PID
k_pitch_p = 30.0  # P constant of the pitch PID

# Utility functions
def sign(x):
    return (x > 0) - (x < 0)

def clamp(value, low, high):
    return max(min(value, high), low)

class Webots:
    def __init__(self):
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

        # Display the welcome message
        print("Start the drone...")

        # Wait one second
        while self.robot.step(self.timestep) != -1:
            if self.robot.getTime() > 1.0:
                break

        # Variables
        self.target_altitude = 1.0  # The target altitude. Can be changed by the user.

    def compute_inputs(self, tokens):
        time = self.robot.getTime()  # in seconds

        # Retrieve robot position using the sensors
        roll = self.imu.getRollPitchYaw()[0]
        pitch = self.imu.getRollPitchYaw()[1]
        altitude = self.gps.getValues()[2]
        roll_velocity = self.gyro.getValues()[0]
        pitch_velocity = self.gyro.getValues()[1]

        # Blink the front LEDs alternatively with a 1 second rate
        led_state = int(time) % 2
        self.front_left_led.set(led_state)
        self.front_right_led.set(not led_state)

        # Stabilize the Camera by actuating the camera motors according to the gyro feedback
        self.camera_roll_motor.setPosition(-0.115 * roll_velocity)
        self.camera_pitch_motor.setPosition(-0.1 * pitch_velocity)

        # Transform the tokens to disturbances on the stabilization algorithm
        roll_disturbance = 0.0
        pitch_disturbance = 0.0
        yaw_disturbance = 0.0

        for token in tokens:
            if token == "U1":
                print("up")
                pitch_disturbance = -2.0
            elif token == "D1":
                print("down")
                pitch_disturbance = 2.0
            elif token == "R1":
                print("right")
                yaw_disturbance = -1.3
            elif token == "L1":
                print("left")
                yaw_disturbance = 1.3
            elif token == "SR":
                print("shift_right")
                roll_disturbance = -1.0
            elif token == "SL":
                print("shift_left")
                roll_disturbance = 1.0
            elif token == "SU":
                self.target_altitude += 0.05
                print(f"target altitude: {self.target_altitude} [m]")
            elif token == "SD":
                self.target_altitude -= 0.05
                print(f"target altitude: {self.target_altitude} [m]")

        # Compute the roll, pitch, yaw and vertical inputs
        roll_input = k_roll_p * clamp(roll, -1.0, 1.0) + roll_velocity + roll_disturbance
        pitch_input = k_pitch_p * clamp(pitch, -1.0, 1.0) + pitch_velocity + pitch_disturbance
        yaw_input = yaw_disturbance
        clamped_difference_altitude = clamp(self.target_altitude - altitude + k_vertical_offset, -1.0, 1.0)
        vertical_input = k_vertical_p * (clamped_difference_altitude ** 3)

        # Actuate the motors taking into consideration all the computed inputs
        front_left_motor_input = k_vertical_thrust + vertical_input - roll_input + pitch_input - yaw_input
        front_right_motor_input = k_vertical_thrust + vertical_input + roll_input + pitch_input + yaw_input
        rear_left_motor_input = k_vertical_thrust + vertical_input - roll_input - pitch_input + yaw_input
        rear_right_motor_input = k_vertical_thrust + vertical_input + roll_input - pitch_input - yaw_input

        self.front_left_motor.setVelocity(front_left_motor_input)
        self.front_right_motor.setVelocity(-front_right_motor_input)
        self.rear_left_motor.setVelocity(-rear_left_motor_input)
        self.rear_right_motor.setVelocity(rear_right_motor_input)

    def run(self, tokens):
        # Main loop
        while self.robot.step(self.timestep) != -1:
            self.compute_inputs(tokens)



