#![allow(dead_code)]
#![allow(unused_variables)]
use crate::apa102::Apa102;
use apa102_spi as apa102;
use chrono::{DateTime, Local, Timelike};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use smart_leds::colors::WHITE;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use std::error::Error;
use word_clock_fr::tokens_for_time;
// use word_clock_fr::Token;

/// Returns (is_pm, hour, minute) in local timezone
fn now_hm12() -> (bool, u32, u32) {
    let now: DateTime<Local> = Local::now();
    let (is_pm, hour) = now.hour12();
    (is_pm, hour, now.minute())
}

const NUM_PIXELS: usize = 72;
const OFF: RGB8 = RGB8 { r: 0, g: 0, b: 0 };

fn main() -> Result<(), Box<dyn Error>> {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
    let mut dotstar = Apa102::new(spi);
    let mut pixels = vec![OFF; NUM_PIXELS];

    let (is_pm, hour, min) = now_hm12();
    // we use tokens_for_time and not leds_for_time so we can later colour each word differently
    for token in tokens_for_time(is_pm, hour, min) {
        for led_id in token.to_leds() {
            pixels[led_id as usize] = WHITE;
        }
    }
    dotstar
        .write(brightness(pixels.iter().cloned(), 10))
        .unwrap();
    Ok(())
}
