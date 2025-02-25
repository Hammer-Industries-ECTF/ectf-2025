use crate::message::{HostMessage, ResponseMessage};
use crate::message::{HostUpdateMessage, HostDecodeMessage};
use crate::message::{ResponseDebugMessage, ResponseListMessage, ResponseDecodeMessage};

use crate::message::packet::extract_decoder_id;
use crate::message::packet::PacketError;

use crate::sys::secure_memory::{overwrite_subscription, retrieve_channel_secret, retrieve_subscription, retrieve_subscriptions, verify_decoder_id};
use crate::sys::secure_memory::{Subscription, SecureMemoryError};

use crate::sys::decrypt::{decrypt_block, decrypt_blocks};

#[derive(Debug, Clone)]
pub enum CommandError {
    InvalidSubscriptionChannel(u32),
    InvalidSecretChannel(u32),
    NotSubscribed(u32),
    SubscriptionFuture(u32, u64),
    SubscriptionPast(u32, u64),
    InvalidDecoderID,
    SecureMemoryError(SecureMemoryError),
    PacketError(PacketError)
}

pub fn message_respond(host_message: HostMessage) -> Result<ResponseMessage, CommandError> {
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
}

fn list_subscriptions() -> Result<ResponseListMessage, CommandError> {
    let mut subscriptions = retrieve_subscriptions();
    subscriptions.retain(|sub| sub.valid);
    Ok(ResponseListMessage{subscriptions})
}

fn update_subscription(message: HostUpdateMessage) -> Result<(), CommandError> {
    if retrieve_subscription(message.channel_id).is_none() { return Err(CommandError::InvalidSubscriptionChannel(message.channel_id)); }
    let secret = retrieve_channel_secret(message.channel_id);
    if secret.is_none() { return Err(CommandError::InvalidSecretChannel(message.channel_id)); }
    let secret = secret.unwrap();
    let decrypted_decoder_id_block = decrypt_block(secret, message.encrypted_decoder_id);
    let decoder_id = extract_decoder_id(decrypted_decoder_id_block);
    if decoder_id.is_err() { return Err(CommandError::PacketError(decoder_id.unwrap_err())); }
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
    // Verify increasing timestamp
    // Verify frame length
    let subscription = retrieve_subscription(message.channel_id);
    if subscription.is_none() { return Err(CommandError::InvalidSubscriptionChannel(message.channel_id)); }
    let subscription = subscription.unwrap();
    if !subscription.valid { return Err(CommandError::NotSubscribed(message.channel_id)); }
    if message.timestamp < subscription.start { return Err(CommandError::SubscriptionFuture(message.channel_id, subscription.start)); }
    if message.timestamp > subscription.end { return Err(CommandError::SubscriptionPast(message.channel_id, subscription.end)); }
    let secret = retrieve_channel_secret(message.channel_id);
    if secret.is_none() { return Err(CommandError::InvalidSecretChannel(message.channel_id)); }
    // Decrypt and verify stamp
    // Decrypt frame data
    // Update timestamp
    todo!()
}
