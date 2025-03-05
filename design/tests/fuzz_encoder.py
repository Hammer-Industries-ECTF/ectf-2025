# flake8: noqa
from pythonfuzz.main import PythonFuzz
from json import loads
from base64 import urlsafe_b64decode
from Crypto.Cipher import AES
import sys, os
sys.path.append(os.path.join(os.path.dirname(__file__), '..'))
from ectf25_design.encoder import Encoder
from ectf25_design.gen_secrets import gen_secrets


COMPANY_STAMP = "HammerIndustries".encode("ascii")


INIT_EXPECTED_TYPE_ERRORS = {
    "secrets is not a byte-string",
}

INIT_EXPECTED_VALUE_ERRORS = {
    "Found improper amount of secret pairs for channel",
    "Could not find master secret pair or channel 0 secret pair",
    "Found invalid channel numbers in secrets",
    "Found invalid AES key: not 256 bits",
    "Found invalid CBC IV: not 128 bits",
}

ENCODE_EXPECTED_TYPE_ERRORS = {
    "channel is not an int",
    "timestamp is not an int",
    "frame is not a byte-string"
}

ENCODE_EXPECTED_VALUE_ERRORS = {
    "channel is not representable as u32",
    "Could not find secret for channel:",
    "timestamp is not representable as u64",
    "Cannot encode empty frame",
    "Cannot encode frame bigger than 64 bytes"
}


def input_transformer(buf: bytes) -> tuple[bytes, int, bytes, int]:
    secrets: bytes
    channel: int
    frame: bytes
    timestamp: int
    if len(buf) < 12:
        raise ValueError("Insufficient buffer length")
    channel = int.from_bytes(buf[0:4], 'little')
    timestamp = int.from_bytes(buf[4:12], 'little')
    secrets = gen_secrets([channel])
    frame = buf[12:]
    return (secrets, channel, frame, timestamp)


def output_verifier(encode_output: bytes,
                    secrets: bytes,
                    expected_channel: int,
                    expected_frame: bytes,
                    expected_timestamp: int):
    # Recover secrets
    secrets_data: dict[str, list[str]] = loads(secrets.decode("utf-8"))
    secrets = {k: (urlsafe_b64decode(v[0]), urlsafe_b64decode(v[1]))
               for k, v in secrets_data.items()}

    # Decrypt frame master layer
    decoded_frame: bytes = anti_cbc_decrypt(secrets["master"][0],
                                            secrets["master"][1],
                                            encode_output)
    timestamp: int = int.from_bytes(decoded_frame[0:8], 'little')
    channel: int = int.from_bytes(decoded_frame[8:12], 'little')
    frame_length: int = int.from_bytes(decoded_frame[12:16], 'little')

    assert channel == expected_channel, "Decoded wrong channel"
    assert timestamp == expected_timestamp, "Decoded wrong timestamp"

    # Decrypt frame channel layer
    frame_package: bytes = anti_cbc_decrypt(secrets[str(channel)][0],
                                            secrets[str(channel)][1],
                                            decoded_frame[16:])
    company_stamp = frame_package[0:16]
    frame_data = frame_package[16:16+frame_length]

    assert company_stamp == COMPANY_STAMP, "Decoded wrong company stamp"
    assert frame_data == expected_frame, "Decoded wrong frame"


def anti_cbc_decrypt(key: bytes, iv: bytes, blocks: bytes) -> bytes:
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

    # Anti CBC Decrypt
    cipher: AES.EcbMode = AES.new(key, AES.MODE_ECB)
    output: bytes = bytes()
    cbc_intermediate: bytes = iv
    for block in (blocks[i:i+16] for i in range(0, len(blocks), 16)):
        aes_out: bytes = cipher.encrypt(block)
        output += bytes((_a ^ _b for _a, _b in zip(aes_out, cbc_intermediate)))
        cbc_intermediate = block
    return output


def fuzz(buf: bytes):
    inputs = None
    try:
        inputs = input_transformer(buf)
    except ValueError as e:
        if e.args[0] != "Insufficient buffer length":
            raise e
    else:
        encode_output = None
        encoder = None
        try:
            encoder = Encoder(inputs[0])
        except TypeError as e:
            if e.args[0] not in INIT_EXPECTED_TYPE_ERRORS:
                raise e
        except ValueError as e:
            if e.args[0] not in INIT_EXPECTED_VALUE_ERRORS:
                raise e
        else:
            try:
                encode_output = encoder.encode(*inputs[1:])
            except TypeError as e:
                if e.args[0] not in ENCODE_EXPECTED_TYPE_ERRORS:
                    raise e
            except ValueError as e:
                if e.args[0] not in ENCODE_EXPECTED_VALUE_ERRORS:
                    raise e
            else:
                output_verifier(encode_output, *inputs)


fuzz_exec = PythonFuzz(fuzz)


if __name__ == '__main__':
    fuzz_exec()
