//! IDK HOW THIS IS GONNA WORK

static mut TIMESTAMP: u64 = 0;

pub fn get_timestamp() -> u64 {
    unsafe { TIMESTAMP }
}

pub fn set_timestamp(timestamp: u64) -> () {
    unsafe { TIMESTAMP = timestamp; }
}