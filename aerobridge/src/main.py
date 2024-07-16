from bridge.aerobridge import RpcServer, SimpleXMLRPCServer, RequestHandler
import threading

if __name__ == "__main__":
    drone_backend = RpcServer()
    server = SimpleXMLRPCServer(("localhost", 8000), requestHandler=RequestHandler)
    server.register_instance(drone_backend)
    print("RPC server running on port 8000...")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("Server interrupted")
    finally:
        drone_backend.stop()
        print("Server stopped")
