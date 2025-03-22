//! Command Execution Code

use crate::message::{HostMessage, ResponseMessage};
use crate::message::{HostUpdateMessage, HostDecodeMessage};
use crate::message::{ResponseListMessage, ResponseDecodeMessage};

use hal::flc::Flc;

use hal::aes::{Aes, AesBlock};

use crate::message::packet::verify_company_stamp;

use crate::sys::secure_memory::{overwrite_subscription, retrieve_subscription, retrieve_subscriptions, verify_decoder_id, verify_timestamp, set_timestamp};
use crate::sys::secure_memory::{Subscription, SecureMemoryError};

use crate::sys::decrypt::{decrypt_company_stamp, decrypt_decoder_id, decrypt_frame, DecryptError};

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum CommandError {
    InvalidSubscriptionChannel(u32),
    NotSubscribed(u32),
    SubscriptionFuture(u32, u64),
    SubscriptionPast(u32, u64),
    InvalidDecoderID,
    FramePast(u64),
    FrameLengthIncorrect(u32),
    FrameCompanyStampIncorrect(AesBlock),
    EmptyFrameData,
    SecureMemoryError(SecureMemoryError),
    DecryptError(DecryptError)
}

pub fn execute_command(flc: &Flc, aes: &Aes, host_message: HostMessage) -> Result<ResponseMessage, CommandError> {
    match host_message {
        HostMessage::List => Ok(ResponseMessage::List(list_subscriptions(flc)?)),
        HostMessage::Update(host_update_message) => Ok(ResponseMessage::Update(update_subscription(flc, aes, host_update_message)?)),
        HostMessage::Decode(host_decode_message) => Ok(ResponseMessage::Decode(decode_message(flc, aes, host_decode_message)?))
    }
}

fn list_subscriptions(flc: &Flc) -> Result<ResponseListMessage, CommandError> {
    // Retrieve all subscriptions
    let subscriptions = retrieve_subscriptions(flc);
    if subscriptions.is_err() { return Err(CommandError::SecureMemoryError(subscriptions.unwrap_err())); }
    let mut subscriptions = subscriptions.unwrap();
    // Filter only valid subscriptions and return
    subscriptions.retain(|sub| sub.valid);
    Ok(ResponseListMessage{subscriptions})
}

fn update_subscription(flc: &Flc, aes: &Aes, message: HostUpdateMessage) -> Result<(), CommandError> {
    // Validate decoder id intact
    let decoder_id = decrypt_decoder_id(flc, aes, message.channel_id, message.encrypted_decoder_id);
    if decoder_id.is_err() { return Err(CommandError::DecryptError(decoder_id.unwrap_err())); }
    let decoder_id = decoder_id.unwrap();
    let verify_id = verify_decoder_id(flc, decoder_id);
    if verify_id.is_err() { return Err(CommandError::SecureMemoryError(verify_id.unwrap_err())); }
    if !verify_decoder_id(flc, decoder_id).unwrap() { return Err(CommandError::InvalidDecoderID); }
    // Overwrite subscription
    let subscription = Subscription {
        channel_id: message.channel_id,
        valid: true,
        end: message.end,
        start: message.start
    };
    match overwrite_subscription(flc, subscription) {
        Ok(()) => Ok(()),
        Err(secure_memory_error) => Err(CommandError::SecureMemoryError(secure_memory_error))
    }
}

fn decode_message(flc: &Flc, aes: &Aes, message: HostDecodeMessage) -> Result<ResponseDecodeMessage, CommandError> {
    // Validate metadata is within bounds
    if !verify_timestamp(message.timestamp) { return Err(CommandError::FramePast(message.timestamp)); }
    if message.encrypted_frame.len() < 2 { return Err(CommandError::EmptyFrameData); }
    if message.frame_length == 0 || message.frame_length > 64 { return Err(CommandError::FrameLengthIncorrect(message.frame_length)); }
    if ((((message.frame_length - 1) / 16) + 2) as usize) != message.encrypted_frame.len() { return Err(CommandError::FrameLengthIncorrect(message.frame_length)); }
    // Get and verify subscription if not on emergency broadcast channel
    if message.channel_id != 0 {
        let subscription = retrieve_subscription(flc, message.channel_id);
        if subscription.is_err() { return Err(CommandError::SecureMemoryError(subscription.unwrap_err())); }
        let subscription = subscription.unwrap();
        if !subscription.valid { return Err(CommandError::NotSubscribed(message.channel_id)); }
        if message.timestamp < subscription.start { return Err(CommandError::SubscriptionFuture(message.channel_id, subscription.start)); }
        if message.timestamp > subscription.end { return Err(CommandError::SubscriptionPast(message.channel_id, subscription.end)); }
    }
    // Validate company stamp intact
    let decrypted_company_stamp = decrypt_company_stamp(flc, aes, message.channel_id, *message.encrypted_frame.first().unwrap());
    if decrypted_company_stamp.is_err() { return Err(CommandError::DecryptError(decrypted_company_stamp.unwrap_err())); }
    let decrypted_company_stamp = decrypted_company_stamp.unwrap();
    if !verify_company_stamp(decrypted_company_stamp) { return Err(CommandError::FrameCompanyStampIncorrect(decrypted_company_stamp)); }
    // Decrypt frame data
    let decrypted_frame = decrypt_frame(flc, aes, message.channel_id, message.encrypted_frame);
    if decrypted_frame.is_err() { return Err(CommandError::DecryptError(decrypted_frame.unwrap_err())); }
    let decrypted_frame = decrypted_frame.unwrap();
    // Update timestamp and return
    set_timestamp(message.timestamp);
    Ok(ResponseDecodeMessage{frame: decrypted_frame})
}
