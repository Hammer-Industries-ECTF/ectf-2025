from json import loads
from base64 import urlsafe_b64decode
from Crypto.Cipher import AES
from argparse import ArgumentParser, FileType


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
        encoded_data: bytes = _anti_cbc_encrypt(self._secrets[str(channel)][0],
                                                self._secrets[str(channel)][1],
                                                (self._company_stamp
                                                 + frame
                                                 + (b'\x00' * pad_bytes_needed)))
        encoded_frame: bytes = _anti_cbc_encrypt(self._secrets["master"][0],
                                                 self._secrets["master"][1],
                                                 (timestamp.to_bytes(8, 'little')
                                                  + channel.to_bytes(4, 'little')
                                                  + len(frame).to_bytes(4, 'little')
                                                  + encoded_data))

        return encoded_frame


def _anti_cbc_encrypt(key: bytes, iv: bytes, blocks: bytes) -> bytes:
    # Bounds checking
    if type(key) is not bytes:
        raise TypeError("key is not a byte-string")
    if len(key) != 32:
        raise ValueError("key is not 256 bits")
    if type(iv) is not bytes:
        raise TypeError("iv is not a byte-string")
    if len(iv) != 16:
        raise ValueError("iv is not 128 bits")
    if type(blocks) is not bytes:
        raise TypeError("blocks is not a byte-string")
    if len(blocks) == 0:
        raise ValueError("blocks is empty")
    if len(blocks) % 16 != 0:
        raise ValueError("blocks is not a multiple of 128 bits long")

    # Anti CBC Encrypt
    cipher: AES.EcbMode = AES.new(key, AES.MODE_ECB)
    output: bytes = bytes()
    cbc_intermediate: bytes = iv
    for block in (blocks[i:i+16] for i in range(0, len(blocks), 16)):
        aes_in: bytes = bytes((_a ^ _b for _a, _b in zip(block, cbc_intermediate)))
        aes_out: bytes = cipher.decrypt(aes_in)
        cbc_intermediate = aes_out
        output += aes_out
    return output


def parse_args():
    parser = ArgumentParser(prog="ectf25_design.encoder")
    parser.add_argument(
        "secrets_file",
        type=FileType("rb"),
        help="Path to the secrets file generated by ectf25_design.gen_secrets"
    )
    parser.add_argument(
        "channel",
        type=int,
        help="Channel to encode for"
    )
    parser.add_argument(
        "frame",
        help="Contents of the frame"
    )
    parser.add_argument(
        "timestamp",
        type=int,
        help="Timestamp of the frame"
    )
    return parser.parse_args()


def main():
    args = parse_args()
    encoder = Encoder(args.secrets_file.read())
    print(repr(encoder.encode(args.channel, args.frame.encode(), args.timestamp)))


if __name__ == "__main__":
    main()
