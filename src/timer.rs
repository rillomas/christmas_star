extern crate time;

pub fn get_msec_tick(t: time::Timespec) -> i64 {
    (t.sec * 1000 + (t.nsec / 1000).to_i64().unwrap())
}

pub fn get_tick() -> i64 {
    let time = time::get_time();
    return get_msec_tick(time);
}
