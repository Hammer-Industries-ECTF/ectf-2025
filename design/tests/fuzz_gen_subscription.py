# flake8: noqa
from pythonfuzz.main import PythonFuzz
from json import loads
from base64 import urlsafe_b64decode
from Crypto.Cipher import AES
import sys, os
sys.path.append(os.path.join(os.path.dirname(__file__), '..'))
from ectf25_design.gen_subscription import gen_subscription
from ectf25_design.gen_secrets import gen_secrets


EXPECTED_TYPE_ERRORS = {
    "secrets is not a byte-string",
    "device_id is not an int",
    "start timestamp is not an int",
    "end timestamp is not an int",
    "channel is not an int"
}

EXPECTED_VALUE_ERRORS = {
    "Found improper amount of secret pairs for channel",
    "Could not find master secret pair or channel 0 secret pair",
    "Found invalid channel numbers in secrets",
    "Found invalid AES key: not 256 bits",
    "Found invalid CBC IV: not 128 bits",
    "device_id is not representable as u32",
    "timestamps are not representable as u64",
    "end is less than start",
    "channel is not representable as u32",
    "Cannot generate subscription for channel 0",
    "Could not find secret for channel:"
}


def input_transformer(buf: bytes) -> tuple[bytes, int, int, int, int]:
    secrets: bytes
    device_id: int
    start: int
    end: int
    channel: int
    if len(buf) < 24:
        raise ValueError("Insufficient buffer length")
    device_id = int.from_bytes(buf[0:4], 'little')
    start = int.from_bytes(buf[4:12], 'little')
    end = int.from_bytes(buf[12:20], 'little')
    channel = int.from_bytes(buf[20:24], 'little')
    secrets = gen_secrets([channel])
    return (secrets, device_id, start, end, channel)


def output_verifier(gen_subscription_output: bytes,
                    secrets: bytes,
                    expected_device_id: int,
                    expected_start: int,
                    expected_end: int,
                    expected_channel: int):
    # Recover secrets
    secrets_data: dict[str, list[str]] = loads(secrets.decode("utf-8"))
    secrets: dict[str, tuple[bytes, bytes]] = {k:
                                               (urlsafe_b64decode(v[0]), urlsafe_b64decode(v[1]))
                                               for k, v in secrets_data.items()}

    # Decrypt package master layer
    decoded_update: bytes = anti_cbc_decrypt(secrets["master"][0],
                                             secrets["master"][1],
                                             gen_subscription_output)
    channel: int = int.from_bytes(decoded_update[0:16], 'little')
    end: int = int.from_bytes(decoded_update[16:24], 'little')
    start: int = int.from_bytes(decoded_update[24:32], 'little')
    encoded_device_id: bytes = decoded_update[32:48]

    assert channel == expected_channel, "Decoded wrong channel"
    assert end == expected_end, "Decoded wrong end"
    assert start == expected_start, "Decoded wrong start"

    # Decrypt package channel layer
    device_id: bytes = anti_cbc_decrypt(secrets[str(channel)][0],
                                        secrets[str(channel)][1],
                                        encoded_device_id)
    device_id: int = int.from_bytes(device_id, 'little')

    assert device_id == expected_device_id, "Decoded wrong device_id"


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
        gen_subscription_output = None
        try:
            gen_subscription_output = gen_subscription(*inputs)
        except TypeError as e:
            if e.args[0] not in EXPECTED_TYPE_ERRORS:
                raise e
        except ValueError as e:
            if e.args[0] not in EXPECTED_VALUE_ERRORS:
                raise e
        else:
            output_verifier(gen_subscription_output, *inputs)  


fuzz_exec = PythonFuzz(fuzz)


if __name__ == '__main__':
    fuzz_exec()
