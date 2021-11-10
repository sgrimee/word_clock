use chrono::{DateTime, Local, Timelike};
use word_clock_fr::time_text;

// Return current hour, minute and am/pm in local timezone.
// Returns (is_pm, hour, minute)
// While trivial, this function abstracts how to get current time on a specific platform
fn now_hm12() -> (bool, u32, u32) {
    let now: DateTime<Local> = Local::now();
    let (is_pm, hour) = now.hour12();
    (is_pm, hour, now.minute())
}

fn main() {
    let (is_pm, hour, min) = now_hm12();
    println!("{}", time_text(is_pm, hour, min));
}
