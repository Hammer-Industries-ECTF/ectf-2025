from json import loads
from base64 import urlsafe_b64decode
from Crypto.Cipher import AES
from argparse import ArgumentParser, FileType
from pathlib import Path


def gen_subscription(secrets: bytes, device_id: int, start: int, end: int, channel: int) -> bytes:
    # Recover secrets
    if type(secrets) is not bytes:
        raise TypeError("secrets is not a byte-string")
    secrets_data: dict[str, list[str]] = loads(secrets.decode("utf-8"))
    if any((len(secret_list) != 2 for secret_list in secrets_data.values())):
        raise ValueError("Found improper amount of secret pairs for channel")
    secrets: dict[str, tuple[bytes, bytes]] = {k:
                                               (urlsafe_b64decode(v[0]), urlsafe_b64decode(v[1]))
                                               for k, v in secrets_data.items()}

    # Secrets bounds checking
    if "master" not in secrets or "0" not in secrets:
        raise ValueError("Could not find master secret pair or channel 0 secret pair")
    if any(((int(channel_num) < 0 or int(channel_num) > 2**32 - 1)
            for channel_num in secrets if channel_num != 'master')):
        raise ValueError("Found invalid channel numbers in secrets")
    if any((len(secret[0]) != 32 for secret in secrets.values())):
        raise ValueError("Found invalid AES key: not 256 bits")
    if any((len(secret[1]) != 16 for secret in secrets.values())):
        raise ValueError("Found invalid CBC IV: not 128 bits")

    # Other args bounds checking
    if type(device_id) is not int:
        raise TypeError("device_id is not an int")
    if device_id < 0 or device_id > 2**32 - 1:
        raise ValueError("device_id is not representable as u32")
    if type(start) is not int:
        raise TypeError("start timestamp is not an int")
    if type(end) is not int:
        raise TypeError("end timestamp is not an int")
    if start < 0 or end < 0 or start > 2**64 - 1 or end > 2**64 - 1:
        raise ValueError("timestamps are not representable as u64")
    if end < start:
        raise ValueError("end is less than start")
    if type(channel) is not int:
        raise TypeError("channel is not an int")
    if channel < 0 or channel > 2**32 - 1:
        raise ValueError("channel is not representable as u32")
    if channel == 0:
        raise ValueError("Cannot generate subscription for channel 0")
    if str(channel) not in secrets:
        raise ValueError("Could not find secret for channel:", channel)

    # Encrypt package
    channel_cipher: AES.CbcMode = AES.new(secrets[str(channel)][0],
                                          AES.MODE_CBC,
                                          iv=secrets[str(channel)][1])
    encoded_device_id: bytes = channel_cipher.decrypt(device_id.to_bytes(16, 'little'))
    master_cipher: AES.CbcMode = AES.new(secrets["master"][0],
                                         AES.MODE_CBC,
                                         iv=secrets["master"][1])
    encoded_update: bytes = master_cipher.decrypt(channel.to_bytes(16, 'little')
                                                  + end.to_bytes(8, 'little')
                                                  + start.to_bytes(8, 'little')
                                                  + encoded_device_id)

    return encoded_update


def parse_args():
    parser = ArgumentParser()
    parser.add_argument(
        "--force",
        "-f",
        action="store_true",
        help="Force creation of subscription file, overwriting existing file"
    )
    parser.add_argument(
        "secrets_file",
        type=FileType("rb"),
        help="Path to the secrets file created by ectf25_design.gen_secrets"
    )
    parser.add_argument(
        "subscription_file",
        type=Path,
        help="Path to the subscription file to be generated"
    )
    parser.add_argument(
        "device_id",
        type=lambda x: int(x, 0),
        help="Device ID of the update recipient"
    )
    parser.add_argument(
        "start",
        type=int,
        help="Subscription start timestamp"
    )
    parser.add_argument(
        "end",
        type=int,
        help="Subscription end timestamp"
    )
    parser.add_argument(
        "channel",
        type=int,
        help="Channel to subscribe to"
    )
    return parser.parse_args()


def main():
    args = parse_args()
    subscription = gen_subscription(args.secrets_file.read(),
                                    args.device_id,
                                    args.start,
                                    args.end,
                                    args.channel)
    with open(args.subscription_file, "wb" if args.force else "xb") as f:
        f.write(subscription)


if __name__ == "__main__":
    main()
