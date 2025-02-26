use crate::message::{HostMessage, ResponseMessage};
use crate::message::{HostUpdateMessage, HostDecodeMessage};
use crate::message::{ResponseDebugMessage, ResponseListMessage, ResponseDecodeMessage};

use crate::message::packet::verify_company_stamp;

use crate::sys::secure_memory::{overwrite_subscription, retrieve_subscription, retrieve_subscriptions, verify_decoder_id};
use crate::sys::secure_memory::{Subscription, SecureMemoryError};

use crate::sys::decrypt::{decrypt_company_stamp, decrypt_decoder_id, decrypt_frame, DecryptError};

use crate::utils::timestamp::{get_timestamp, set_timestamp};

#[derive(Debug, Clone)]
pub enum CommandError {
    DebugRequested,
    InvalidSubscriptionChannel(u32),
    NotSubscribed(u32),
    SubscriptionFuture(u32, u64),
    SubscriptionPast(u32, u64),
    InvalidDecoderID,
    FramePast(u64),
    FrameLengthIncorrect(u32),
    FrameCompanyStampIncorrect(u128),
    EmptyFrameData,
    SecureMemoryError(SecureMemoryError),
    DecryptError(DecryptError)
}

pub fn execute_command(host_message: HostMessage) -> Result<ResponseMessage, CommandError> {
    match host_message {
        HostMessage::Debug => Ok(ResponseMessage::Debug(debug_info()?)),
        HostMessage::List => Ok(ResponseMessage::List(list_subscriptions()?)),
        HostMessage::Update(host_update_message) => Ok(ResponseMessage::Update(update_subscription(host_update_message)?)),
        HostMessage::Decode(host_decode_message) => Ok(ResponseMessage::Decode(decode_message(host_decode_message)?))
    }
}

fn debug_info() -> Result<ResponseDebugMessage, CommandError> {
    // idk man
    Ok(ResponseDebugMessage{})
    // Err(CommandError::DebugRequested)
}

fn list_subscriptions() -> Result<ResponseListMessage, CommandError> {
    let mut subscriptions = retrieve_subscriptions();
    subscriptions.retain(|sub| sub.valid);
    Ok(ResponseListMessage{subscriptions})
}

fn update_subscription(message: HostUpdateMessage) -> Result<(), CommandError> {
    if retrieve_subscription(message.channel_id).is_none() { return Err(CommandError::InvalidSubscriptionChannel(message.channel_id)); }
    let decoder_id = decrypt_decoder_id(message.channel_id, message.encrypted_decoder_id);
    if decoder_id.is_err() { return Err(CommandError::DecryptError(decoder_id.unwrap_err())); }
    let decoder_id = decoder_id.unwrap();
    if !verify_decoder_id(decoder_id) { return Err(CommandError::InvalidDecoderID); }
    let subscription = Subscription {
        channel_id: message.channel_id,
        valid: true,
        end: message.end,
        start: message.start
    };
    match overwrite_subscription(subscription) {
        Ok(()) => Ok(()),
        Err(secure_memory_error) => Err(CommandError::SecureMemoryError(secure_memory_error))
    }
}

fn decode_message(message: HostDecodeMessage) -> Result<ResponseDecodeMessage, CommandError> {
    if message.timestamp <= get_timestamp() { return Err(CommandError::FramePast(message.timestamp)); }
    if message.encrypted_frame.len() < 2 { return Err(CommandError::EmptyFrameData); }
    if message.frame_length == 0 || message.frame_length > 64 { return Err(CommandError::FrameLengthIncorrect(message.frame_length)); }
    if ((((message.frame_length - 1) / 16) + 2) as usize) != message.encrypted_frame.len() { return Err(CommandError::FrameLengthIncorrect(message.frame_length)); }
    let subscription = retrieve_subscription(message.channel_id);
    if subscription.is_none() { return Err(CommandError::InvalidSubscriptionChannel(message.channel_id)); }
    let subscription = subscription.unwrap();
    if !subscription.valid { return Err(CommandError::NotSubscribed(message.channel_id)); }
    if message.timestamp < subscription.start { return Err(CommandError::SubscriptionFuture(message.channel_id, subscription.start)); }
    if message.timestamp > subscription.end { return Err(CommandError::SubscriptionPast(message.channel_id, subscription.end)); }
    let decrypted_company_stamp = decrypt_company_stamp(message.channel_id, *message.encrypted_frame.first().unwrap());
    if decrypted_company_stamp.is_err() { return Err(CommandError::DecryptError(decrypted_company_stamp.unwrap_err())); }
    let decrypted_company_stamp = decrypted_company_stamp.unwrap();
    if !verify_company_stamp(decrypted_company_stamp) { return Err(CommandError::FrameCompanyStampIncorrect(decrypted_company_stamp)); }
    let decrypted_frame = decrypt_frame(message.channel_id, message.encrypted_frame);
    if decrypted_frame.is_err() { return Err(CommandError::DecryptError(decrypted_frame.unwrap_err())); }
    let decrypted_frame = decrypted_frame.unwrap();
    set_timestamp(message.timestamp);
    Ok(ResponseDecodeMessage{frame: decrypted_frame})
}
