from json import loads
from base64 import urlsafe_b64decode
from Crypto.Cipher import AES


class Encoder:
    _secrets: dict[str, tuple[bytes, bytes]] = None
    _company_stamp: bytes = "HammerIndustries".encode("ascii")

    def __init__(self, secrets: bytes):
        # Recover secrets
        if type(secrets) is not bytes:
            raise TypeError("secrets is not a byte-string")
        secrets_data: dict[str, list[str]] = loads(secrets.decode("utf-8"))
        if any((len(secret_list) != 2 for secret_list in secrets_data.values())):
            raise ValueError("Found improper amount of secret pairs for channel")
        self._secrets = {k: (urlsafe_b64decode(v[0]), urlsafe_b64decode(v[1]))
                         for k, v in secrets_data.items()}

        # Secrets bounds checking
        # if len(self._secrets) > 10:
        #     raise ValueError("Too many secret pairs generated (max 8+2):", self._secrets)
        if "master" not in self._secrets or "0" not in self._secrets:
            raise ValueError("Could not find master secret pair or channel 0 secret pair")
        if any(((int(channel_num) < 0 or int(channel_num) > 2**32 - 1)
                for channel_num in secrets if channel_num != 'master')):
            raise ValueError("Found invalid channel numbers in secrets")
        if any((len(secret[0]) != 32 for secret in self._secrets.values())):
            raise ValueError("Found invalid AES key: not 256 bits")
        if any((len(secret[1]) != 16 for secret in self._secrets.values())):
            raise ValueError("Found invalid CBC IV: not 128 bits")

    def encode(self, channel: int, frame: bytes, timestamp: int) -> bytes:
        # Args bounds checking
        if type(channel) is not int:
            raise TypeError("channel is not an int")
        if channel < 0 or channel > 2**32 - 1:
            raise ValueError("channel is not representable as u32")
        if str(channel) not in self._secrets:
            raise ValueError("Could not find secret for channel:", channel)
        if type(timestamp) is not int:
            raise TypeError("timestamp is not an int")
        if timestamp < 0 or timestamp > 2**64 - 1:
            raise ValueError("timestamp is not representable as u64")
        if type(frame) is not bytes:
            raise TypeError("frame is not a byte-string")
        if len(frame) == 0:
            raise ValueError("Cannot encode empty frame")
        if len(frame) > 64:
            raise ValueError("Cannot encode frame bigger than 64 bytes")

        # Encrypt package
        pad_bytes_needed = (16 - len(frame)) % 16
        channel_cipher: AES.CbcMode = AES.new(self._secrets[str(channel)][0],
                                              AES.MODE_CBC,
                                              iv=self._secrets[str(channel)][1])
        encoded_data: bytes = channel_cipher.encrypt(self._company_stamp
                                                     + frame
                                                     + (b'\x00' * pad_bytes_needed))
        master_cipher: AES.CbcMode = AES.new(self._secrets["master"][0],
                                             AES.MODE_CBC,
                                             iv=self._secrets["master"][1])
        encoded_frame: bytes = master_cipher.encrypt(timestamp.to_bytes(8, 'little')
                                                     + channel.to_bytes(4, 'little')
                                                     + len(frame).to_bytes(4, 'little')
                                                     + encoded_data)

        return encoded_frame
