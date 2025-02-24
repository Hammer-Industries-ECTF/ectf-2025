use crate::message::{HostMessage, ResponseMessage};
use crate::message::{HostUpdateMessage, HostDecodeMessage};
use crate::message::{ResponseDebugMessage, ResponseListMessage, ResponseUpdateMessage, ResponseDecodeMessage};

use crate::sys::secure_memory::{retrieve_secret, retrieve_subscription, retrieve_subscriptions};

#[derive(Debug, Clone)]
pub enum CommandError {
    InvalidSubscriptionChannel(u32),
    InvalidSecretChannel(u32),
    SubscriptionNotValid(u32),
    SubscriptionFuture(u32, u64),
    SubscriptionPast(u32, u64)
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
    let existing = retrieve_subscription(message.channel_id);
    if existing.is_none() { return Err(CommandError::InvalidSubscriptionChannel(message.channel_id)); }
    let secret = retrieve_secret(message.channel_id);
    if existing.is_none() { return Err(CommandError::InvalidSecretChannel(message.channel_id)); }
    // Decrypt and verify decoder id
    // CRITICAL_BEGIN
        // Pull all subscription memory
        // Update local subscription
        // Push all subscription memory
    // CRITICAL_END
    Err(unimplemented!())
}

fn decode_message(message: HostDecodeMessage) -> Result<ResponseDecodeMessage, CommandError> {
    // Verify increasing timestamp
    // Verify frame length
    let subscription = retrieve_subscription(message.channel_id);
    if subscription.is_none() { return Err(CommandError::InvalidSubscriptionChannel(message.channel_id)); }
    let subscription = subscription.unwrap();
    if !subscription.valid { return Err(CommandError::SubscriptionNotValid(message.channel_id)); }
    if message.timestamp < subscription.start { return Err(CommandError::SubscriptionFuture(message.channel_id, subscription.start)); }
    if message.timestamp > subscription.end { return Err(CommandError::SubscriptionPast(message.channel_id, subscription.end)); }
    let secret = retrieve_secret(message.channel_id);
    if secret.is_none() { return Err(CommandError::InvalidSecretChannel(message.channel_id)); }
    // Decrypt and verify stamp
    // Decrypt frame data
    // Update timestamp
    Err(unimplemented!())
}
