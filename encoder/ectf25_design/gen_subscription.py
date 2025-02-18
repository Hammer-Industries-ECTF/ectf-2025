from json import loads
from base64 import urlsafe_b64decode
from Crypto.Cipher import AES


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
    if len(secrets) > 10:
        raise ValueError(f"Too many secret pairs generated: {len(secrets)} (max 8+2)")
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
    if type(channel) is not int:
        raise TypeError("channel is not an int")
    if channel < 0 or channel > 2**32 - 1:
        raise ValueError("channel is not representable as u32")
    if channel == 0:
        raise ValueError("Cannot generate subscription for channel 0")
    if str(channel) not in secrets:
        raise ValueError(f"Could not find secret for channel {channel}")

    # Encrypt package
    channel_cipher: AES.CbcMode = AES.new(secrets[str(channel)][0],
                                          AES.MODE_CBC,
                                          iv=secrets[str(channel)][1])
    encoded_device_id: bytes = channel_cipher.encrypt(device_id.to_bytes(16, 'little'))
    master_cipher: AES.CbcMode = AES.new(secrets["master"][0],
                                         AES.MODE_CBC,
                                         iv=secrets["master"][1])
    encoded_update: bytes = master_cipher.encrypt(channel.to_bytes(16, 'little')
                                                  + end.to_bytes(8, 'little')
                                                  + start.to_bytes(8, 'little')
                                                  + encoded_device_id)

    return encoded_update


def _subscription_verify(subscription_update_package: bytes,
                         secrets: bytes,
                         expected_device_id: int,
                         expected_start: int,
                         expected_end: int,
                         expected_channel: int) -> bool:
    # Recover secrets
    secrets_data: dict[str, list[str]] = loads(secrets.decode("utf-8"))
    secrets: dict[str, tuple[bytes, bytes]] = {k:
                                               (urlsafe_b64decode(v[0]), urlsafe_b64decode(v[1]))
                                               for k, v in secrets_data.items()}

    # Decrypt package master layer
    master_cipher: AES.CbcMode = AES.new(secrets["master"][0],
                                         AES.MODE_CBC,
                                         iv=secrets["master"][1])
    decoded_update: bytes = master_cipher.decrypt(subscription_update_package)
    channel: int = int.from_bytes(decoded_update[0:16], 'little')
    end: int = int.from_bytes(decoded_update[16:24], 'little')
    start: int = int.from_bytes(decoded_update[24:32], 'little')
    encoded_device_id: bytes = decoded_update[32:48]

    assert channel == expected_channel, "Decoded wrong channel"
    assert end == expected_end, "Decoded wrong end"
    assert start == expected_start, "Decoded wrong start"

    # Decrypt package channel layer
    channel_cipher: AES.CbcMode = AES.new(secrets[str(channel)][0],
                                          AES.MODE_CBC,
                                          iv=secrets[str(channel)][1])
    device_id: bytes = channel_cipher.decrypt(encoded_device_id)
    device_id: int = int.from_bytes(device_id, 'little')

    assert device_id == expected_device_id, "Decoded wrong device_id"

    return True
