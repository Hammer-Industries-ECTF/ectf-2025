# flake8: noqa
from pythonfuzz.main import PythonFuzz
from json import loads
from base64 import urlsafe_b64decode
import sys, os
sys.path.append(os.path.join(os.path.dirname(__file__), '..'))
from ectf25_design.gen_secrets import gen_secrets


EXPECTED_TYPE_ERRORS = {
    "channels is not a list",
    "Detected non-integer channel",
}

EXPECTED_VALUE_ERRORS = {
    # "Too many channels (max 8):",
    "Invalid channel number: not representable as u32"
}


def input_transformer(buf: bytes) -> tuple[list[int]]:
    channels: list[int] = list()
    while len(buf) > 0:
        if len(buf) < 4:
            channels.append(int.from_bytes(buf, 'little'))
            buf = b''
        else:
            channels.append(int.from_bytes(buf[0:4], 'little'))
            buf = buf[4:]
    return (channels,)


def output_verifier(gen_secrets_output: bytes):
    # Recover secrets
    if type(gen_secrets_output) is not bytes:
        raise TypeError("secrets is not a byte-string")
    secrets_data: dict[str, list[str]] = loads(gen_secrets_output.decode("utf-8"))
    if any((len(secret_list) != 2 for secret_list in secrets_data.values())):
        raise ValueError("Found improper amount of secret pairs for channel")
    secrets = {k: (urlsafe_b64decode(v[0]), urlsafe_b64decode(v[1]))
                        for k, v in secrets_data.items()}

    # Secrets bounds checking
    if "master" not in secrets or "0" not in secrets:
        raise ValueError("Could not find master secret pair or channel 0 secret pair")
    if any(((int(channel_num) < 0 or int(channel_num) > 2**32 - 1)
            for channel_num in gen_secrets_output if channel_num != 'master')):
        raise ValueError("Found invalid channel numbers in secrets")
    if any((len(secret[0]) != 32 for secret in secrets.values())):
        raise ValueError("Found invalid AES key: not 256 bits")
    if any((len(secret[1]) != 16 for secret in secrets.values())):
        raise ValueError("Found invalid CBC IV: not 128 bits")


def fuzz(buf: bytes):
    inputs = input_transformer(buf)
    gen_secrets_output = None
    try:
        gen_secrets_output = gen_secrets(*inputs)
    except TypeError as e:
        if e.args[0] not in EXPECTED_TYPE_ERRORS:
            raise e
    except ValueError as e:
        if e.args[0] not in EXPECTED_VALUE_ERRORS:
            raise e
    else:
        output_verifier(gen_secrets_output)


fuzz_exec = PythonFuzz(fuzz)


if __name__ == '__main__':
    fuzz_exec()
