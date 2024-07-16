from bridge.node import Node
import time
import xmlrpc.client
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class AeroBridge:
    def __init__(self):
        self.server = None
        self.node = Node()
        self.running = False
        self.connected = False

    def connect(self):
        try:
            self.server = xmlrpc.client.ServerProxy('http://localhost:8000', allow_none=True)
            self.server.get_sensor_data()
            self.connected = True
            logger.info("Connected to RPC server successfully")
        except Exception as e:
            logger.error(f"Cannot establish connection: {e}")
            self.connected = False

    def run(self):
        self.running = True
        self.connect()

        x = self.server.start()
        print(x)

        try:
            while self.running:
                
                data = self.server.get_sensor_data()
                self.node.send_data(data)
                ci = self.node.receive_control_input().to_json()
                self.server.handle_control_input(ci)
                time.sleep(0.5)

        except KeyboardInterrupt:
            logger.info("Program interrupted")

        finally:
            self.running = False
            logger.info("Program finished")

if __name__ == "__main__":
    aero_bridge = AeroBridge()
    aero_bridge.run()