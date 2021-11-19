use chrono::{DateTime, Local, Timelike};
use word_clock_fr::string_for_time;

// Returns (is_pm, hour, minute) in local timezone
fn now_hm12() -> (bool, u32, u32) {
    let now: DateTime<Local> = Local::now();
    let (is_pm, hour) = now.hour12();
    (is_pm, hour, now.minute())
}

fn main() {
    let (is_pm, hour, min) = now_hm12();
    println!("{}", string_for_time(is_pm, hour, min));
}
