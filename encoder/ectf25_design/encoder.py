class Encoder:
    def __init__(self, secrets: bytes):
        raise NotImplementedError
    
    def encode(self, channel: int, frame: bytes, timestamp: int) -> bytes:
        raise NotImplementedError
