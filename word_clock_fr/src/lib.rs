// #![allow(dead_code)]

// Text lit for each light number
const LIT_TEXT: [&str; 27] = [
    "IL", " EST ", "DEUX", "QUATRE", "TROIS", "NEUF", "UNE", "SEPT", "HUIT", "SIX", "CINQ", "MI",
    "DI", "X", "MINUIT", "ONZE", " HEURE", "S", " MOINS", " LE", " DIX", " ET", " QUART", " VINGT",
    " CINQ", " DEMI", "E",
];

const OPENING: &[usize] = &[0, 1];
const HOUR_TWO: &[usize] = &[2];
const HOUR_FOUR: &[usize] = &[3];
const HOUR_THREE: &[usize] = &[4];
const HOUR_NINE: &[usize] = &[5];
const HOUR_ONE: &[usize] = &[6];
const HOUR_SEVEN: &[usize] = &[7];
const HOUR_EIGHT: &[usize] = &[8];
const HOUR_SIX: &[usize] = &[9];
const HOUR_FIVE: &[usize] = &[10];
const HOUR_NOON: &[usize] = &[11, 12];
const HOUR_TEN: &[usize] = &[12, 13];
const HOUR_MIDNIGHT: &[usize] = &[14];
const HOUR_ELEVEN: &[usize] = &[15];
const HOUR_SEPARATOR_SINGULAR: &[usize] = &[16];
const HOUR_SEPARATOR_PLURAL: &[usize] = &[16, 17];
const MINUTE_MINUS: &[usize] = &[18];
const MINUTE_THE: &[usize] = &[19];
const MINUTE_TEN: &[usize] = &[20];
const MINUTE_AND: &[usize] = &[21];
const MINUTE_QUARTER: &[usize] = &[22];
const MINUTE_TWENTY: &[usize] = &[23];
const MINUTE_FIVE: &[usize] = &[24];
const MINUTE_HALF_SINGULAR: &[usize] = &[25];
const MINUTE_HALF_PLURAL: &[usize] = &[25, 26];

/// Return a vector with the light numbers that need to be lit to display the given time.
/// Panics if minute >= 60 or hour >=12
///
/// # Examples
/// ```
/// use word_clock_fr::lights_for_time;
/// assert_eq!(lights_for_time(true, 1 as u32, 5 as u32), vec![0, 1, 6, 16, 24]);
/// ```
pub fn lights_for_time(is_pm: bool, hour: u32, minute: u32) -> Vec<usize> {
    assert!(minute < 60 && hour < 12);
    // round to 5 minutes
    let minute = minute - (minute % 5);
    let minute_words = match minute {
        0 => vec![],
        5 => MINUTE_FIVE.to_vec(),
        10 => MINUTE_TEN.to_vec(),
        15 => [MINUTE_AND, MINUTE_QUARTER].concat(),
        20 => MINUTE_TWENTY.to_vec(),
        25 => [MINUTE_TWENTY, MINUTE_FIVE].concat(),
        30 => {
            if hour == 0 {
                [MINUTE_AND, MINUTE_HALF_SINGULAR].concat()
            } else {
                [MINUTE_AND, MINUTE_HALF_PLURAL].concat()
            }
        }
        35 => [MINUTE_MINUS, MINUTE_TWENTY, MINUTE_FIVE].concat(),
        40 => [MINUTE_MINUS, MINUTE_TWENTY].concat(),
        45 => [MINUTE_MINUS, MINUTE_THE, MINUTE_QUARTER].concat(),
        50 => [MINUTE_MINUS, MINUTE_TEN].concat(),
        55 => [MINUTE_MINUS, MINUTE_FIVE].concat(),
        _ => panic!("that value of minute cannot happen"),
    };

    // after half-past, borrow from next hour
    let mut is_pm = is_pm;
    let mut hour = hour;
    if minute > 30 {
        hour = (hour + 1) % 12;
        if hour == 0 {
            is_pm = !is_pm
        };
    }

    let hour_words = match hour {
        0 => {
            if is_pm {
                HOUR_NOON.to_vec()
            } else {
                HOUR_MIDNIGHT.to_vec()
            }
        }
        1 => [HOUR_ONE, HOUR_SEPARATOR_SINGULAR].concat(),
        2 => [HOUR_TWO, HOUR_SEPARATOR_PLURAL].concat(),
        3 => [HOUR_THREE, HOUR_SEPARATOR_PLURAL].concat(),
        4 => [HOUR_FOUR, HOUR_SEPARATOR_PLURAL].concat(),
        5 => [HOUR_FIVE, HOUR_SEPARATOR_PLURAL].concat(),
        6 => [HOUR_SIX, HOUR_SEPARATOR_PLURAL].concat(),
        7 => [HOUR_SEVEN, HOUR_SEPARATOR_PLURAL].concat(),
        8 => [HOUR_EIGHT, HOUR_SEPARATOR_PLURAL].concat(),
        9 => [HOUR_NINE, HOUR_SEPARATOR_PLURAL].concat(),
        10 => [HOUR_TEN, HOUR_SEPARATOR_PLURAL].concat(),
        11 => [HOUR_ELEVEN, HOUR_SEPARATOR_PLURAL].concat(),
        _ => panic!("hour needs to be between 0 and 11"),
    };

    let mut lights = OPENING.to_vec();
    lights.extend(hour_words);
    lights.extend(minute_words);
    lights
}

/// Return a string with the given time using words from the clock in correct order
/// Panics if minute >= 60 or hour >=12
///
/// # Examples
/// ```
/// use word_clock_fr::string_for_time;
/// assert_eq!(string_for_time(true, 1 as u32, 5 as u32), String::from("IL EST UNE HEURE CINQ"));
/// assert_eq!(string_for_time(true, 2 as u32, 10 as u32), String::from("IL EST DEUX HEURES DIX"));
/// assert_eq!(string_for_time(true, 3 as u32, 15 as u32), String::from("IL EST TROIS HEURES ET QUART"));
/// assert_eq!(string_for_time(true, 4 as u32, 21 as u32), String::from("IL EST QUATRE HEURES VINGT"));
/// assert_eq!(string_for_time(true, 5 as u32, 29 as u32), String::from("IL EST CINQ HEURES VINGT CINQ"));
/// assert_eq!(string_for_time(true, 6 as u32, 30 as u32), String::from("IL EST SIX HEURES ET DEMIE"));
/// assert_eq!(string_for_time(false, 7 as u32, 12 as u32), String::from("IL EST SEPT HEURES DIX"));
/// assert_eq!(string_for_time(true, 7 as u32, 35 as u32), String::from("IL EST HUIT HEURES MOINS VINGT CINQ"));
/// assert_eq!(string_for_time(true, 8 as u32, 40 as u32), String::from("IL EST NEUF HEURES MOINS VINGT"));
/// assert_eq!(string_for_time(true, 9 as u32, 45 as u32), String::from("IL EST DIX HEURES MOINS LE QUART"));
/// assert_eq!(string_for_time(true, 10 as u32, 51 as u32), String::from("IL EST ONZE HEURES MOINS DIX"));
/// assert_eq!(string_for_time(true, 0 as u32, 30 as u32), String::from("IL EST MIDI ET DEMI"));
/// assert_eq!(string_for_time(false, 0 as u32, 30 as u32), String::from("IL EST MINUIT ET DEMI"));
/// assert_eq!(string_for_time(true, 11 as u32, 55 as u32), String::from("IL EST MINUIT MOINS CINQ"));
/// assert_eq!(string_for_time(false, 11 as u32, 55 as u32), String::from("IL EST MIDI MOINS CINQ"));
/// ```
pub fn string_for_time(is_pm: bool, hour: u32, minute: u32) -> String {
    // get words from the LIGHTS vector and join them
    lights_for_time(is_pm, hour, minute)
        .iter()
        .map(|x| {
            *(LIT_TEXT
                .get(*x)
                .unwrap_or_else(|| panic!("Light index {} out of bounds.", *x)))
        })
        .collect::<String>()
}
