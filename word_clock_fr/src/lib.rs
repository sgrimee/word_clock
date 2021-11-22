//! This crate converts a time given in hour, minute, am/pm into either a full-sentence repesentation,
//! or a list of leds that need to be lit on a word-clock.

#[warn(missing_docs)]
#[derive(Debug, PartialEq)]
enum Token {
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
    MinuteHalfMasc,
    MinuteHalfFem,
}

// French language implementation
impl Token {
    fn to_tuple(&self) -> (&str, Vec<u32>) {
        match self {
            Token::Opening => ("IL EST", (0u32..=1).collect()),
            Token::HourTwo => ("DEUX", vec![2]),
            Token::HourFour => ("QUATRE", vec![3]),
            Token::HourThree => ("TROIS", vec![4]),
            Token::HourNine => ("NEUF", vec![5]),
            Token::HourOne => ("UNE", vec![6]),
            Token::HourSeven => ("SEPT", vec![7]),
            Token::HourEight => ("HUIT", vec![8]),
            Token::HourSix => ("SIX", vec![9]),
            Token::HourFive => ("CINQ", vec![10]),
            Token::HourNoon => ("MIDI", vec![11]),
            Token::HourTen => ("DIX", vec![12]),
            Token::HourMidnight => ("MINUIT", vec![13]),
            Token::HourEleven => ("ONZE", vec![14]),
            Token::HourSeparatorSingular => ("HEURE", vec![15]),
            Token::HourSeparatorPlural => ("HEURES", vec![16]),
            Token::MinuteMinus => ("MOINS", vec![17]),
            Token::MinuteThe => ("LE", vec![18]),
            Token::MinuteTen => ("DIX", vec![19]),
            Token::MinuteAnd => ("ET", vec![20]),
            Token::MinuteQuarter => ("QUART", vec![21]),
            Token::MinuteTwenty => ("VINGT", vec![22]),
            Token::MinuteFive => ("CINQ", vec![23]),
            Token::MinuteHalfMasc => ("DEMI", vec![24]),
            Token::MinuteHalfFem => ("DEMIE", vec![25]),
        }
    }

    fn to_str(&self) -> &str {
        self.to_tuple().0
    }

    fn to_leds(&self) -> Vec<u32> {
        self.to_tuple().1
    }
}

/// Return a vector with the Tokens needed to display time.
/// Panics if minute >= 60 or hour >=12
/// French language implementation.
fn tokens_for_time(is_pm: bool, hour: u32, minute: u32) -> Vec<Token> {
    assert!(minute < 60 && hour < 12);
    // round to 5 minutes
    let minute = minute - (minute % 5);
    let minute_words = match minute {
        0 => vec![],
        5 => vec![Token::MinuteFive],
        10 => vec![Token::MinuteTen],
        15 => vec![Token::MinuteAnd, Token::MinuteQuarter],
        20 => vec![Token::MinuteTwenty],
        25 => vec![Token::MinuteTwenty, Token::MinuteFive],
        30 => {
            if hour == 0 {
                vec![Token::MinuteAnd, Token::MinuteHalfMasc]
            } else {
                vec![Token::MinuteAnd, Token::MinuteHalfFem]
            }
        }
        35 => vec![Token::MinuteMinus, Token::MinuteTwenty, Token::MinuteFive],
        40 => vec![Token::MinuteMinus, Token::MinuteTwenty],
        45 => vec![Token::MinuteMinus, Token::MinuteThe, Token::MinuteQuarter],
        50 => vec![Token::MinuteMinus, Token::MinuteTen],
        55 => vec![Token::MinuteMinus, Token::MinuteFive],
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
                vec![Token::HourNoon]
            } else {
                vec![Token::HourMidnight]
            }
        }
        1 => vec![Token::HourOne, Token::HourSeparatorSingular],
        2 => vec![Token::HourTwo, Token::HourSeparatorPlural],
        3 => vec![Token::HourThree, Token::HourSeparatorPlural],
        4 => vec![Token::HourFour, Token::HourSeparatorPlural],
        5 => vec![Token::HourFive, Token::HourSeparatorPlural],
        6 => vec![Token::HourSix, Token::HourSeparatorPlural],
        7 => vec![Token::HourSeven, Token::HourSeparatorPlural],
        8 => vec![Token::HourEight, Token::HourSeparatorPlural],
        9 => vec![Token::HourNine, Token::HourSeparatorPlural],
        10 => vec![Token::HourTen, Token::HourSeparatorPlural],
        11 => vec![Token::HourEleven, Token::HourSeparatorPlural],
        _ => panic!("hour needs to be between 0 and 11"),
    };

    let mut tokens = vec![Token::Opening];
    tokens.extend(hour_words);
    tokens.extend(minute_words);
    tokens
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
    tokens_for_time(is_pm, hour, minute)
        .iter()
        .map(|x| x.to_str())
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Return list of leds to turn on to display the given time.
///
/// # Examples
/// ```
/// use word_clock_fr::leds_for_time;
/// assert_eq!(leds_for_time(true, 1 as u32, 5 as u32), vec![0, 1, 6, 15, 23]);
/// ```
pub fn leds_for_time(is_pm: bool, hour: u32, minute: u32) -> Vec<u32> {
    let mut leds = Vec::new();
    for token in tokens_for_time(is_pm, hour, minute).iter() {
        leds.extend(token.to_leds());
    }
    leds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_tuple() {
        assert_eq!(Token::Opening.to_tuple(), ("IL EST", vec![0, 1]));
    }

    #[test]
    fn test_tokens_for_time() {
        let expected = vec![
            Token::Opening,
            Token::HourOne,
            Token::HourSeparatorSingular,
            Token::MinuteFive,
        ];
        assert_eq!(tokens_for_time(true, 1 as u32, 5 as u32), expected);
    }
}
