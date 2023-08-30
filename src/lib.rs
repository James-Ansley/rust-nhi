use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref OLD_NHI_FORMAT_REGEX: Regex =
        Regex::new(r"^[A-HJ-NP-Z]{3}\d{4}$").unwrap();
    static ref NEW_NHI_FORMAT_REGEX: Regex =
        Regex::new(r"^[A-HJ-NP-Z]{3}\d{2}[A-HJ-NP-Z]{2}$").unwrap();
}

/// Checks a string against the New Zealand Ministry of Health NHI specification
/// defined by HISO 10046:2023 and the NHI validation routine
///
/// ## See Also
/// https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/
///
/// # Arguments
///
/// * `nhi`: A potential NHI string
///
/// returns: True if the given string satisfies the New Zealand NHI Validation Routine and False otherwise
///
/// # Examples
///
/// ```
/// use nhi::is_nhi;
///
/// is_nhi("ZAC5361");  // true
/// is_nhi("ZBN77VL");  // true
/// is_nhi("ZZZ0044");  // false
/// is_nhi("ZZZ00AA");  // false
/// ```
pub fn is_nhi(nhi: &str) -> bool {
    let nhi = &nhi.to_uppercase();
    if OLD_NHI_FORMAT_REGEX.is_match(nhi) {
        let checksum = checksum(nhi) % 11;
        let check_digit = (11 - checksum) % 10;
        checksum != 0 && check_digit == char_code(nhi.chars().last().unwrap())
    } else if NEW_NHI_FORMAT_REGEX.is_match(nhi) {
        let checksum = checksum(nhi) % 23;
        let check_digit = 23 - checksum;
        check_digit == char_code(nhi.chars().last().unwrap())
    } else {
        false
    }
}

fn checksum(nhi: &str) -> u32 {
    nhi.chars()
        .enumerate()
        .map(|(i, c)| char_code(c) * (7 - i as u32))
        .take(6)
        .sum::<u32>()
}

fn char_code(char: char) -> u32 {
    if char.is_ascii_digit() {
        char.to_digit(10).unwrap()
    } else {
        char as u32 - 64 - ('I' < char) as u32 - ('O' < char) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_old_format_nhi_number() {
        assert!(is_nhi("JBX3656"));
        assert!(is_nhi("ZZZ0016"));
        assert!(is_nhi("ZZZ0024"));
        assert!(is_nhi("ZAA0067"));
        assert!(is_nhi("ZAA0075"));
        assert!(is_nhi("ZAA0083"));
        assert!(is_nhi("ZAA0091"));
        assert!(is_nhi("ZAA0105"));
        assert!(is_nhi("ZAA0113"));
        assert!(is_nhi("ZAA0121"));
        assert!(is_nhi("ZAA0130"));
        assert!(is_nhi("ZAA0148"));
        assert!(is_nhi("ZAA0156"));
        assert!(is_nhi("ZAC5361"));
    }

    #[test]
    fn valid_new_format_nhi_number() {
        assert!(is_nhi("ZBN77VL"));
        assert!(is_nhi("ZZZ00AC"));
        assert!(is_nhi("ZDR69YX"));
        assert!(is_nhi("ZSC21TN"));
        assert!(is_nhi("ZZB30NH"));
        assert!(is_nhi("ZYZ81ZV"));
        assert!(is_nhi("ZVB97XQ"));
        assert!(is_nhi("ZRA29VA"));
        assert!(is_nhi("ZYX61YS"));
    }

    #[test]
    fn invalid_old_format_nhi_numbers() {
        assert!(!is_nhi("ZZZ0044"));
        assert!(!is_nhi("ZZZ0017"));
        assert!(!is_nhi("DAB8233"));

        // Needs a checkdigit of 6
        assert!(!is_nhi("JBX3650"));
        assert!(!is_nhi("JBX3651"));
        assert!(!is_nhi("JBX3652"));
        assert!(!is_nhi("JBX3653"));
        assert!(!is_nhi("JBX3654"));
        assert!(!is_nhi("JBX3655"));
        assert!(!is_nhi("JBX3657"));
        assert!(!is_nhi("JBX3658"));
        assert!(!is_nhi("JBX3659"));
    }

    #[test]
    fn invalid_new_format_nhi_numbers() {
        assert!(!is_nhi("ZZZ00AA"));
        assert!(!is_nhi("ZZZ00AY"));
        assert!(!is_nhi("ZVU27KY"));
        assert!(!is_nhi("ZVU27KA"));

        // Needs a check character of V
        for c in "ABCDEFGHIJKLMNOPQRSTUWXYZ".chars() {
            assert!(!is_nhi(&format!("ZHW58C{c}")))
        }
    }

    #[test]
    fn random_strings_are_invalid() {
        assert!(!is_nhi("not an NHI"));
        assert!(!is_nhi("!@#$%&*"));
        assert!(!is_nhi("AAANNNC"));
        assert!(!is_nhi("AAANNAC"));
        assert!(!is_nhi("ZVU27K"));
        assert!(!is_nhi("JBX365"));
        assert!(!is_nhi(""));
    }

    #[test]
    fn is_nhi_is_case_insensitive() {
        // Valid cases
        assert!(is_nhi("jBx3656"));
        assert!(is_nhi("zZz0016"));
        assert!(is_nhi("zZz0024"));
        assert!(is_nhi("zAa0067"));
        assert!(is_nhi("zAa0075"));
        assert!(is_nhi("zAa0083"));
        assert!(is_nhi("zAa0091"));
        assert!(is_nhi("zAa0105"));
        assert!(is_nhi("zAa0113"));
        assert!(is_nhi("zAa0121"));
        assert!(is_nhi("zAa0130"));
        assert!(is_nhi("zAa0148"));
        assert!(is_nhi("zAa0156"));
        assert!(is_nhi("zAc5361"));
        assert!(is_nhi("zZz00aC"));
        assert!(is_nhi("zDr69yX"));
        assert!(is_nhi("zSc21tN"));
        assert!(is_nhi("zZb30nH"));
        assert!(is_nhi("zYz81Zv"));
        assert!(is_nhi("zVb97Xq"));
        assert!(is_nhi("zRa29Va"));
        assert!(is_nhi("zYx61Ys"));

        // Invalid cases
        assert!(!is_nhi("zzZ0044"));
        assert!(!is_nhi("zzZ0017"));
        assert!(!is_nhi("daB8233"));
        assert!(!is_nhi("jbX3650"));
        assert!(!is_nhi("jbX3651"));
        assert!(!is_nhi("jbX3652"));
        assert!(!is_nhi("jbX3653"));
        assert!(!is_nhi("jbX3654"));
        assert!(!is_nhi("jbX3655"));
        assert!(!is_nhi("jbX3657"));
        assert!(!is_nhi("jbX3658"));
        assert!(!is_nhi("jbX3659"));
        assert!(!is_nhi("zzZ00aa"));
        assert!(!is_nhi("zzZ00ay"));
        assert!(!is_nhi("zvU27ky"));
        assert!(!is_nhi("zvU27ka"));
        assert!(!is_nhi("zhW58cz"));
    }

    #[test]
    fn char_codes() {
        assert_eq!(char_code('A'), 1);
        assert_eq!(char_code('B'), 2);
        assert_eq!(char_code('C'), 3);
        assert_eq!(char_code('D'), 4);
        assert_eq!(char_code('E'), 5);
        assert_eq!(char_code('F'), 6);
        assert_eq!(char_code('G'), 7);
        assert_eq!(char_code('H'), 8);
        assert_eq!(char_code('J'), 9);
        assert_eq!(char_code('K'), 10);
        assert_eq!(char_code('L'), 11);
        assert_eq!(char_code('M'), 12);
        assert_eq!(char_code('N'), 13);
        assert_eq!(char_code('P'), 14);
        assert_eq!(char_code('Q'), 15);
        assert_eq!(char_code('R'), 16);
        assert_eq!(char_code('S'), 17);
        assert_eq!(char_code('T'), 18);
        assert_eq!(char_code('U'), 19);
        assert_eq!(char_code('V'), 20);
        assert_eq!(char_code('W'), 21);
        assert_eq!(char_code('X'), 22);
        assert_eq!(char_code('Y'), 23);
        assert_eq!(char_code('Z'), 24);
    }
}
