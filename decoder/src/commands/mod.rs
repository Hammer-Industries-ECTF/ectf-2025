use crate::message::{HostMessage, ResponseMessage};
use crate::message::{HostDebugMessage, HostListMessage, HostUpdateMessage, HostDecodeMessage};
use crate::message::{ResponseDebugMessage, ResponseListMessage, ResponseUpdateMessage, ResponseDecodeMessage};

fn message_respond(host_message: HostMessage) -> ResponseMessage {
    match host_message {
        HostMessage::Debug (host_debug_message)   => ResponseMessage::Debug (debug_info(host_debug_message)),
        HostMessage::List  (host_list_message)     => ResponseMessage::List  (list_subscriptions(host_list_message)),
        HostMessage::Update(host_update_message) => ResponseMessage::Update(update_subscription(host_update_message)),
        HostMessage::Decode(host_decode_message) => ResponseMessage::Decode(decode_message(host_decode_message))
    }
}

fn debug_info(_message: HostDebugMessage) -> ResponseDebugMessage {
    // idk man
    ResponseDebugMessage{}
}

fn list_subscriptions(_message: HostListMessage) -> ResponseListMessage {
    // Pull from subscription memory
}

fn update_subscription(message: HostUpdateMessage) -> ResponseUpdateMessage {
    // Verify channel id is valid
    // Decrypt and verify decoder id
    // CRITICAL_BEGIN
        // Pull all subscription memory
        // Update local subscription
        // Push all subscription memory
    // CRITICAL_END
}

fn decode_message(message: HostDecodeMessage) -> ResponseDecodeMessage {
    // Verify channel number
    // Verify frame length
    // Verify increasing timestamp
    // Verify valid subscription with timestamp
    // CRITICAL_BEGIN
        // Decrypt and verify stamp
        // Decrypt frame data
        // Update timestamp
    // CRITICAL_END
}
