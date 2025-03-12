import sys
import time
import serial
import serial.serialutil
import serial.tools.list_ports
from time import sleep
import aes
from copy import copy

mk = 0x00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
mk_arr = aes.utils.int2arr8bit(mk, 32)
rk_arr = aes.core.key_expansion(mk_arr, 256)

def ph(arr):
    print("0x"+hex(aes.utils.arr8bit2int(arr))[2:].zfill(32))
    
def open_serial(portname):
    try:
        ser: serial.Serial = serial.Serial(
            port=portname,
            baudrate=115200,
            parity=serial.PARITY_NONE,
            stopbits=serial.STOPBITS_ONE,
            bytesize=serial.EIGHTBITS
        )
    except serial.serialutil.SerialException:
        raise serial.serialutil.SerialException("Failed to open port")

    ser.isOpen()
    ## Handshake ##
    ser.write(b'\x00')
    sleep(0.1)
    while ser.inWaiting() > 0:
        assert b'\x00' == ser.read(1)

    # print(type(ser))
    return ser


def send_uc(ser: serial.Serial, pkt: bytes):
    ## Send Command ##
    ser.write(b'\x01')

    ## Send Cyphertext ##
    ser.write(pkt)
    sleep(0.001)
    
    ## Retreive Plaintext ##
    dec = b''
    while ser.in_waiting > 0:
        dec += ser.read(1)

    return dec

def generate_encrypted(text, filename):
    plaintext = text.encode('utf-8')
    
    padding_needed = 16 - (len(plaintext) % 16)
    if padding_needed != 16:
        plaintext += b' ' * padding_needed

    with open(filename, "wb") as f:
        while plaintext:
            dec, plaintext = plaintext[:16], plaintext[16:]
            dec_arr = list(bytearray(dec))
            enc_arr = aes.core.decryption(dec_arr, rk_arr)
            f.write(bytes(enc_arr))

if __name__ == '__main__':
    ports = serial.tools.list_ports.comports()
    # print(f"{[port.name for port in ports]=}")
    
    print("Encrypting file...")
    with open(sys.argv[1], "r") as f:
        text = f.read()
    generate_encrypted(text, 'text.aes')
    
    print("Sending to decoder to decrypt...")
    with open_serial(ports[0].name) as ser:
        sleep(1)
        while ser.in_waiting > 0:
            _ = ser.read(1)
        
        packets_sent = 0
        max_time = 0
        start_time = time.perf_counter()
        with open('text.aes', "rb") as r:
            with open("text.txt", "wb") as w:
                while True:
                    enc = r.read(16)
                    if enc is None or len(enc) < 16: break
                    send_time = time.perf_counter()
                    dec: bytes = send_uc(ser, enc)
                    recv_time = time.perf_counter()
                    pkt_time = recv_time - send_time
                    max_time = max(max_time, pkt_time)
                    # print(enc.hex(), dec)
                    w.write(dec)
                    w.flush()
                    packets_sent += 1
        end_time = time.perf_counter()
        tot_time = end_time-start_time
    
    ser.close()
    print("============== Results! ==============")
    print(f"Packets Sent: {packets_sent}, Runtime: {tot_time:.2f} sec")
    print(f"Rate: {packets_sent / tot_time:.2f} packets/sec")
    print(f"Avg Time: {1000 * tot_time / packets_sent:.2f} ms/packet")
    print(f"Max Time: {1000 * max_time:.2f} ms")
    print("======================================")
    


    
