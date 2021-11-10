use std::fmt;

/**
Return a string with the given time using words from the clock in correct order

# Examples
```
use word_clock_fr::time_text;
assert_eq!(time_text(true, 1 as u32, 5 as u32), String::from("IL EST UNE HEURE CINQ"));
assert_eq!(time_text(true, 2 as u32, 10 as u32), String::from("IL EST DEUX HEURES DIX"));
assert_eq!(time_text(true, 3 as u32, 15 as u32), String::from("IL EST TROIS HEURES ET QUART"));
assert_eq!(time_text(true, 4 as u32, 21 as u32), String::from("IL EST QUATRE HEURES VINGT"));
assert_eq!(time_text(true, 5 as u32, 29 as u32), String::from("IL EST CINQ HEURES VINGT CINQ"));
assert_eq!(time_text(true, 6 as u32, 30 as u32), String::from("IL EST SIX HEURES ET DEMIE"));
assert_eq!(time_text(true, 7 as u32, 35 as u32), String::from("IL EST HUIT HEURES MOINS VINGT CINQ"));
assert_eq!(time_text(true, 8 as u32, 40 as u32), String::from("IL EST NEUF HEURES MOINS VINGT"));
assert_eq!(time_text(true, 9 as u32, 45 as u32), String::from("IL EST DIX HEURES MOINS LE QUART"));
assert_eq!(time_text(true, 10 as u32, 51 as u32), String::from("IL EST ONZE HEURES MOINS DIX"));
assert_eq!(time_text(true, 11 as u32, 55 as u32), String::from("IL EST MINUIT MOINS CINQ"));
assert_eq!(time_text(false, 11 as u32, 55 as u32), String::from("IL EST MIDI MOINS CINQ"));
```
*/
pub fn time_text(is_pm: bool, hour: u32, minute: u32) -> String {
    assert!(hour < 12);
    let words: Vec<String> = time_tokens(is_pm, hour, minute)
        .iter()
        .map(|x| x.to_string())
        .collect();
    words.join(" ")
}

/**
Return a vector with the light numbers that need to be lit to display the given time.

# Examples

```
use word_clock_fr::time_lights;
assert_eq!(time_lights(true, 1 as u32, 5 as u32), vec![0, 1, 6, 16, 24]);
```
*/
pub fn time_lights(is_pm: bool, hour: u32, minute: u32) -> Vec<u16> {
    assert!(hour < 12);
    let mut lights: Vec<u16> = vec![];
    for token in time_tokens(is_pm, hour, minute) {
        lights.append(&mut token.lights())
    }
    lights
}

// Tokens for French
enum TokFr {
    Opening,
    HourTwo,
    HourFour,
    HourThree,
    HourNine,
    HourOne,
    HourSeven,
    HourEight,
    HourSix,
    HourFive,
    HourNoon,
    HourTen,
    HourMidnight,
    HourEleven,
    HourSeparatorSingular,
    HourSeparatorPlural,
    MinuteMinus,
    MinuteThe,
    MinuteTen,
    MinuteAnd,
    MinuteQuarter,
    MinuteTwenty,
    MinuteFive,
    MinuteHalf,
}

impl fmt::Display for TokFr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match *self {
            TokFr::Opening => "IL EST",
            TokFr::HourTwo => "DEUX",
            TokFr::HourFour => "QUATRE",
            TokFr::HourThree => "TROIS",
            TokFr::HourNine => "NEUF",
            TokFr::HourOne => "UNE",
            TokFr::HourSeven => "SEPT",
            TokFr::HourEight => "HUIT",
            TokFr::HourSix => "SIX",
            TokFr::HourFive => "CINQ",
            TokFr::HourNoon => "MIDI",
            TokFr::HourTen => "DIX", // overlap
            TokFr::HourMidnight => "MINUIT",
            TokFr::HourEleven => "ONZE",
            TokFr::HourSeparatorSingular => "HEURE",
            TokFr::HourSeparatorPlural => "HEURES",
            TokFr::MinuteMinus => "MOINS",
            TokFr::MinuteThe => "LE",
            TokFr::MinuteTen => "DIX",
            TokFr::MinuteAnd => "ET",
            TokFr::MinuteQuarter => "QUART",
            TokFr::MinuteTwenty => "VINGT",
            TokFr::MinuteFive => "CINQ",
            TokFr::MinuteHalf => "DEMIE",
        };
        write!(f, "{}", text)
    }
}

impl TokFr {
    // Return array of lights to lit up for each token.
    fn lights(&self) -> Vec<u16> {
        match *self {
            TokFr::Opening => vec![0, 1],
            TokFr::HourTwo => vec![2],
            TokFr::HourFour => vec![3],
            TokFr::HourThree => vec![4],
            TokFr::HourNine => vec![5],
            TokFr::HourOne => vec![6],
            TokFr::HourSeven => vec![7],
            TokFr::HourEight => vec![8],
            TokFr::HourSix => vec![9],
            TokFr::HourFive => vec![10],
            TokFr::HourNoon => vec![11, 12],
            TokFr::HourTen => vec![12, 13], // overlap
            TokFr::HourMidnight => vec![14],
            TokFr::HourEleven => vec![15],
            TokFr::HourSeparatorSingular => vec![16],
            TokFr::HourSeparatorPlural => vec![16, 17],
            TokFr::MinuteMinus => vec![18],
            TokFr::MinuteThe => vec![19],
            TokFr::MinuteTen => vec![20],
            TokFr::MinuteAnd => vec![21],
            TokFr::MinuteQuarter => vec![22],
            TokFr::MinuteTwenty => vec![23],
            TokFr::MinuteFive => vec![24],
            TokFr::MinuteHalf => vec![25, 26],
        }
    }
}

fn time_tokens(is_pm: bool, hour: u32, minute: u32) -> Vec<TokFr> {
    assert!(minute < 60);
    // round to 5 minutes
    let minute = minute - (minute % 5);
    let minute_words = match minute {
        0 => vec![],
        5 => vec![TokFr::MinuteFive],
        10 => vec![TokFr::MinuteTen],
        15 => vec![TokFr::MinuteAnd, TokFr::MinuteQuarter],
        20 => vec![TokFr::MinuteTwenty],
        25 => vec![TokFr::MinuteTwenty, TokFr::MinuteFive],
        30 => vec![TokFr::MinuteAnd, TokFr::MinuteHalf],
        35 => vec![TokFr::MinuteMinus, TokFr::MinuteTwenty, TokFr::MinuteFive],
        40 => vec![TokFr::MinuteMinus, TokFr::MinuteTwenty],
        45 => vec![TokFr::MinuteMinus, TokFr::MinuteThe, TokFr::MinuteQuarter],
        50 => vec![TokFr::MinuteMinus, TokFr::MinuteTen],
        55 => vec![TokFr::MinuteMinus, TokFr::MinuteFive],
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
        0 => vec![if is_pm {
            TokFr::HourNoon
        } else {
            TokFr::HourMidnight
        }],
        1 => vec![TokFr::HourOne, TokFr::HourSeparatorSingular],
        2 => vec![TokFr::HourTwo, TokFr::HourSeparatorPlural],
        3 => vec![TokFr::HourThree, TokFr::HourSeparatorPlural],
        4 => vec![TokFr::HourFour, TokFr::HourSeparatorPlural],
        5 => vec![TokFr::HourFive, TokFr::HourSeparatorPlural],
        6 => vec![TokFr::HourSix, TokFr::HourSeparatorPlural],
        7 => vec![TokFr::HourSeven, TokFr::HourSeparatorPlural],
        8 => vec![TokFr::HourEight, TokFr::HourSeparatorPlural],
        9 => vec![TokFr::HourNine, TokFr::HourSeparatorPlural],
        10 => vec![TokFr::HourTen, TokFr::HourSeparatorPlural],
        11 => vec![TokFr::HourEleven, TokFr::HourSeparatorPlural],
        _ => panic!("hour needs to be between 0 and 11"),
    };

    let mut tokens = vec![TokFr::Opening];
    tokens.extend(hour_words);
    tokens.extend(minute_words);
    tokens
}
