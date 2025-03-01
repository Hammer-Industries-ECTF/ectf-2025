from secrets import token_urlsafe
from json import dumps
from argparse import ArgumentParser
from pathlib import Path


def gen_secrets(channels: list[int]) -> bytes:
    # Bounds checking
    if type(channels) is not list:
        raise TypeError("channels is not a list")
    channels: set[int] = set(channels)
    channels.add(0)
    # if len(channels) > 9:
    #     raise ValueError("Too many channels (max 8):", len(channels)-1)
    if any((type(channel_id) is not int for channel_id in channels)):
        raise TypeError("Detected non-integer channel")
    if any((channel_num < 0 or channel_num > 2**32 - 1 for channel_num in channels)):
        raise ValueError("Invalid channel number: not-representable as u32")

    # Generate 256-bit AES keys and 128-bit CBC IVs
    secret_nums: list[str] = list()
    for _ in range(len(channels)+1):
        secret_nums.append(token_urlsafe(32) + "=")
        secret_nums.append(token_urlsafe(16) + "==")

    # Non-duplicate check
    while len(set(secret_nums)) < len(secret_nums):
        # Generate 256-bit AES keys and 128-bit CBC IVs
        secret_nums: list[str] = list()
        for _ in range(len(channels)+1):
            secret_nums.append(token_urlsafe(32))
            secret_nums.append(token_urlsafe(16))

    # Assign secrets
    secrets: dict[str, tuple[str, str]] = dict()
    secrets["master"] = (secret_nums[0], secret_nums[1])
    for i, channel_num in enumerate(channels):
        secrets[str(channel_num)] = (secret_nums[2*i+2], secret_nums[2*i+3])

    # Encode secrets as utf-8 json
    return dumps(secrets).encode("utf-8")


def parse_args():
    parser = ArgumentParser()
    parser.add_argument(
        "--force",
        "-f",
        action="store_true",
        help="Force creation of secrets file, overwriting existing file"
    )
    parser.add_argument(
        "secrets_file",
        type=Path,
        help="Path to the secrets file to be created"
    )
    parser.add_argument(
        "channels",
        nargs="+",
        type=int,
        help="Channel list for this deployment. Channel 0 is always valid and should not be entered here."
    )
    return parser.parse_args()


def main():
    args = parse_args()
    secrets = gen_secrets(args.channels)
    with open(args.secrets_file, "wb" if args.force else "xb") as f:
        f.write(secrets)


if __name__ == "__main__":
    main()
