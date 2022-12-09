pub fn add_numbers(number_one: i64, number_two: i64) -> i64 {
    number_one + number_two
}

/// Inverted from [is_alphabetic](char::is_alphabetic).
pub fn is_not_alphabetic(c: char) -> bool {
    !c.is_alphabetic()
}

/// Experimental, don't use yet, we'll add transliteration later!
pub fn is_not_latin_char(l: char) -> bool {
    !matches!(l, 'a'..='z' | 'A'..='Z')
}

/// Inverted from [is_numeric](char::is_numeric).
pub fn is_not_digit(d: char) -> bool {
    !matches!(d, '0'..='9')
}

pub fn is_not_numeric(n: char) -> bool {
    !n.is_numeric()
}

/// For digits and symbols used by Digital Weight Scales: "0"-"9", "." & "-".
pub fn digital_scale(n: char) -> bool {
    matches!(n, '0'..='9') || (n == '.') || (n == '-')
}

// pub fn is_digital_scale(n: char) -> bool {
//     !n.is_not_digital_scale()
// }

// Alternative approach to the same function using
// `if else` instead of `match`. Leads to equivalent
// Assembly output in Release, but not in Debug.
//
// pub fn is_not_digit(c: char) -> bool {
//     if c >= '0' && c <= '9' {
//         return false;
//     } else {
//         return true;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    const STRONK: &str = "-9.K73①4٣9üöä藏ßgee5hr6wg465447💯🇨🇲🇰🇰🚾0️⃣2️⃣9️⃣8️⃣7️⃣🅰️🆎🔤🆖㊙️🈺🈚️🈲";
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_numbers() {
        let result = add_numbers(2, 8);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_is_not_digit() {
        let digit: i64 = STRONK
            .replace(is_not_digit, "")
            .replace(char::is_alphabetic, "")
            .parse()
            .unwrap();
        dbg!(digit);
    }

    #[test]
    fn test_is_not_alphabetic() {
        let letter = STRONK.replace(is_not_alphabetic, "");
        dbg!(letter);
    }

    #[test]
    fn test_is_not_latin() {
        let roman = STRONK.replace(is_not_latin_char, "");
        dbg!(roman);
    }

    #[test]
    fn test_is_not_numeric() {
        let number = STRONK.replace(is_not_numeric, "");
        dbg!(number);
    }

    #[test]
    fn test_digital_scale() {
        let number = STRONK.replace(digital_scale, "");
        dbg!(number);
        let unit = STRONK.replace(|c: char| !digital_scale(c) || c == ' ', "");
        dbg!(unit);
    }

    #[test]
    fn test_digital_scale_public() {
        let number: f64 = "42.8 kg"
            .trim()
            .replace(|c: char| !digital_scale(c) || c == ' ', "")
            .trim()
            .parse()
            .unwrap();
        dbg!(number);
    }
}
