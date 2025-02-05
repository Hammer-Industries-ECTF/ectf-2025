from secrets import randbits
from json import dumps

def gen_secrets(channels: list[int]) -> bytes:
    # Bounds checking
    if type(channels) != list:
        raise TypeError("channels is not a list")
    channels: set[int] = set(channels)
    channels.add(0)
    if len(channels) > 9:
        raise ValueError(f"Too many channels: {len(channels)-1} (max 8)")
    if any((type(channel_id) != int for channel_id in channels)):
        raise TypeError("Detected non-integer channel")
    if any((channel_num < 0 or channel_num > 2**32 - 1 for channel_num in channels)):
        raise ValueError("Invalid channel number: not-representable as u32")

    # Generate 256-bit AES keys
    secrets: dict[int, int] = dict()
    secrets[-1] = randbits(256)
    for channel_num in channels:
        secrets[channel_num] = randbits(256)
    
    # Non-duplicate check
    while len(set(secrets.values())) < len(secrets.values()):
        # Generate 256-bit AES keys again
        secrets: dict[int, int] = dict()
        secrets[-1] = randbits(256)
        for channel_num in channels:
            secrets[channel_num] = randbits(256)

    # Encode secrets as utf-8 json
    return dumps(secrets).encode("utf-8")
