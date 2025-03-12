//! Transmitter Functions

use core::fmt::write;

extern crate alloc;
use alloc::string::String;

use hal::{gpio::{Af1, Pin}, pac::Uart0, uart::BuiltUartPeripheral};

use super::{MessageHeader, ResponseListMessage, ResponseDecodeMessage, ResponseMessage};
use super::{MAGIC_BYTE, LIST_OPCODE, UPDATE_OPCODE, DECODE_OPCODE, ACK_OPCODE, ERR_OPCODE};

use super::receive::RXError;
use super::receive::receive_ack;

#[derive(Debug, Clone, Copy)]
pub enum TXError {
    RXError(RXError),
    InvalidSubscriptionCount(u32)
}

pub fn transmit_message(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, message: ResponseMessage) -> Result<(), TXError> {
    match message {
        ResponseMessage::List(list_response) => {
            let message_header = MessageHeader{ magic: MAGIC_BYTE, opcode: LIST_OPCODE, length: 4+(list_response.subscriptions.len()*20) as u16 };
            transmit_header(uart, message_header);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            transmit_list_body(uart, list_response)
        }
        ResponseMessage::Update(()) => {
            let message_header = MessageHeader{ magic: MAGIC_BYTE, opcode: UPDATE_OPCODE, length: 0 };
            transmit_header(uart, message_header);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        }
        ResponseMessage::Decode(decode_response) => {
            let message_header = MessageHeader{ magic: MAGIC_BYTE, opcode: DECODE_OPCODE, length: decode_response.frame.len() as u16 };
            transmit_header(uart, message_header);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            transmit_decode_body(uart, decode_response)
        }
    }
}

pub fn transmit_ack(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>) -> () {
    let header_bytes: [u8; 4] = [MAGIC_BYTE, ACK_OPCODE, 0, 0];
    uart.write_bytes(&header_bytes);
}

pub fn transmit_err<T: core::fmt::Debug>(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, error: T) -> Result<(), TXError> {
    let mut error_body = String::new();
    write(&mut error_body, format_args!("{:?}", error)).expect("Could not create error message");
    let message_header = MessageHeader{ magic: MAGIC_BYTE, opcode: ERR_OPCODE, length: error_body.len() as u16 };
    transmit_header(uart, message_header);
    let ack = receive_ack(uart);
    if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
    let error_bytes = error_body.as_bytes();
    uart.write_bytes(error_bytes);
    let ack = receive_ack(uart);
    if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
    Ok(())
}

fn transmit_header(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, header: MessageHeader) -> () {
    let header_bytes: [u8; 4] = [header.magic, header.opcode, header.length as u8, (header.length >> 8) as u8];
    uart.write_bytes(&header_bytes);
}

fn transmit_list_body(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, message: ResponseListMessage) -> Result<(), TXError> {
    match message.subscriptions.len() {
        0 => {
            let list_bytes: [u8; 4] = [0; 4];
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        1 => {
            let mut list_bytes: [u8; 24] = [0; 24];
            list_bytes[0] = 1;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        2 => {
            let mut list_bytes: [u8; 44] = [0; 44];
            list_bytes[0] = 2;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        3 => {
            let mut list_bytes: [u8; 64] = [0; 64];
            list_bytes[0] = 3;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        4 => {
            let mut list_bytes: [u8; 84] = [0; 84];
            list_bytes[0] = 4;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        5 => {
            let mut list_bytes: [u8; 104] = [0; 104];
            list_bytes[0] = 5;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        6 => {
            let mut list_bytes: [u8; 124] = [0; 124];
            list_bytes[0] = 6;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        7 => {
            let mut list_bytes: [u8; 144] = [0; 144];
            list_bytes[0] = 7;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        8 => {
            let mut list_bytes: [u8; 164] = [0; 164];
            list_bytes[0] = 8;
            for (i, subscription) in message.subscriptions.iter().enumerate() {
                let channel_id_bytes: [u8; 4] = subscription.channel_id.to_le_bytes();
                let start_bytes: [u8; 8] = subscription.start.to_le_bytes();
                let end_bytes: [u8; 8] = subscription.end.to_le_bytes();
                list_bytes[4+i*20] = channel_id_bytes[0];
                list_bytes[5+i*20] = channel_id_bytes[1];
                list_bytes[6+i*20] = channel_id_bytes[2];
                list_bytes[7+i*20] = channel_id_bytes[3];
                list_bytes[8+i*20] = start_bytes[0];
                list_bytes[9+i*20] = start_bytes[1];
                list_bytes[10+i*20] = start_bytes[2];
                list_bytes[11+i*20] = start_bytes[3];
                list_bytes[12+i*20] = start_bytes[4];
                list_bytes[13+i*20] = start_bytes[5];
                list_bytes[14+i*20] = start_bytes[6];
                list_bytes[15+i*20] = start_bytes[7];
                list_bytes[16+i*20] = end_bytes[0];
                list_bytes[17+i*20] = end_bytes[1];
                list_bytes[18+i*20] = end_bytes[2];
                list_bytes[19+i*20] = end_bytes[3];
                list_bytes[20+i*20] = end_bytes[4];
                list_bytes[21+i*20] = end_bytes[5];
                list_bytes[22+i*20] = end_bytes[6];
                list_bytes[23+i*20] = end_bytes[7];
            }
            uart.write_bytes(&list_bytes);
            let ack = receive_ack(uart);
            if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
            Ok(())
        },
        other => { return Err(TXError::InvalidSubscriptionCount(other as u32)); }
    }
}

fn transmit_decode_body(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, message: ResponseDecodeMessage) -> Result<(), TXError> {
    uart.write_bytes(message.frame.as_slice());
    let ack = receive_ack(uart);
    if ack.is_err() { return Err(TXError::RXError(ack.unwrap_err())); }
    Ok(())
}
