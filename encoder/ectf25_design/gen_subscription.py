from json import loads

def gen_subscription(secrets: bytes, device_id: int, start: int, end: int, channel: int) -> bytes:
    # Recover secrets
    if type(secrets) != bytes:
        raise TypeError("secrets is not a byte-string")
    secrets_data: dict[str, int] = loads(secrets.decode("utf-8"))
    if any((not secrets_dict_key.isdecimal() for secrets_dict_key in secrets_data)):
        raise TypeError("Failure in recovering secrets: could not convert secret channel id to int")
    secrets: dict[int, int] = {int(k): v for k, v in secrets_data.items()}

    # Secrets bounds checking
    if len(secrets) > 10:
        raise ValueError(f"Too many keys generated: {len(secrets)} (max 8+2)")
    if -1 not in secrets or 0 not in secrets:
        raise ValueError("Could not find master secret or channel 0 secret")
    if any((type(aes_key) != int for aes_key in secrets.values())):
        raise TypeError("Found non-integer secret")
    if any(((channel_num < 0 or channel_num > 2**32 - 1) and channel_num != -1 for channel_num in secrets)):
        raise ValueError("Found invalid channel numbers in secrets")
    if any((secret < 0 or secret > 2**256 - 1 for secret in secrets.values())):
        raise ValueError("Found invalid secret: not representable as u256")
    
    # Other args bounds checking
    if type(device_id) != int:
        raise TypeError("device_id is not an int")
    if device_id < 0 or device_id > 2**32 - 1:
        raise ValueError("device_id is not representable as u32")
    if type(start) != int:
        raise TypeError("start timestamp is not an int")
    if type(end) != int:
        raise TypeError("end timestamp is not an int")
    if start < 0 or end < 0 or start > 2**64 - 1 or end > 2**64 - 1:
        raise ValueError("timestamps are not representable as u64")
    if type(channel) != int:
        raise TypeError("channel is not an int")
    if channel < 0 or channel > 2**32 - 1:
        raise ValueError("channel is not representable as u32")
    if channel not in secrets:
        raise ValueError(f"Could not find secret for channel {channel}")

    











    raise NotImplementedError