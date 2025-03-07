//! IDK HOW THIS IS GONNA WORK

#[derive(Debug, Clone)]
enum Timestamp {
    Uninitialized,
    CurrentTime(u64)
}

static mut TIMESTAMP: Timestamp = Timestamp::Uninitialized;

pub fn verify_timestamp(frame_timestamp: u64) -> bool {
    unsafe {
        match TIMESTAMP {
            Timestamp::Uninitialized => true,
            Timestamp::CurrentTime(current_time) => current_time < frame_timestamp
        }
    }
}

pub fn set_timestamp(timestamp: u64) -> () {
    unsafe { TIMESTAMP = Timestamp::CurrentTime(timestamp); }
}