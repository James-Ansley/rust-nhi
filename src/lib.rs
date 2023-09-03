//! Checks strings against the New Zealand Ministry of Health NHI Validation Routine.
//! Supports the old and new NHI number formats specified in
//! [HISO 10046:2023](https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/).
//!
//! ## Usage
//!
//! A simple [is_nhi] function can check whether a string is valid:
//!
//! ```rust
//! use nhi::is_nhi;
//!
//! assert_eq!(is_nhi("ZAC5361"),  true);
//! assert_eq!(is_nhi("ZBN77VL"),  true);
//! assert_eq!(is_nhi("ZZZ0044"),  false);
//! assert_eq!(is_nhi("ZZZ00AA"),  false);
//!
//! ```
//!
//! Alternatively, strings can be parsed to [NHI] values:
//!
//! ```rust
//! use nhi::NHI;
//!
//! let nhi: NHI = "zbn77vl".parse().unwrap();
//! assert_eq!(nhi.as_str(), "ZBN77VL");
//! ```
//!
//! Checks are case-insensitive.
//!
//! ***Note:*** This does not check that the NHI number has been _assigned_ to
//! a person, it merely checks the NHI is consistent with the HISO 10046:2023
//! standard.
//!
//! ### Excluding Testcases
//!
//! NHI numbers that begin with `Z` are reserved for testing.
//! If you wish to exclude these values using [is_nhi], you will need to manually check for a `Z`
//! prefix:
//!
//! ```rust
//! use nhi::is_nhi;
//!
//! let value = "zvb97xq";
//!
//! assert_eq!(is_nhi(value),  true);
//! assert_eq!(!value.to_uppercase().starts_with('Z') && is_nhi(value),  false);
//!
//! ```
//!
//! Alternatively, parsed [NHI] values provide [NHI::is_test] and [NHI::is_not_test] methods:
//!
//! ```rust
//! use nhi::NHI;
//!
//! let reserved: NHI = "ZAA0105".parse().unwrap();
//! let unreserved: NHI = "JBX3656".parse().unwrap();
//!
//! assert!(reserved.is_test());
//! assert!(unreserved.is_not_test());
//! ```
//!
//! ***Note:*** This check does not mean that the NHI number has been _assigned_ to
//! a person, it just means that the NHI value is not reserved for testing.
//!
//! ## See Also
//!
//! - <https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/>
//! - <https://www.tewhatuora.govt.nz/our-health-system/digital-health/health-identity/national-health-index/information-for-health-it-vendors-and-developers>

use std::fmt;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref OLD_NHI_FORMAT: Regex = Regex::new(r"^[A-HJ-NP-Z]{3}\d{4}$").unwrap();
    static ref NEW_NHI_FORMAT: Regex = Regex::new(r"^[A-HJ-NP-Z]{3}\d{2}[A-HJ-NP-Z]{2}$").unwrap();
}

/// Represents a valid NHI number that satisfies the
/// [HISO 10046:2023](https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/)
/// standard.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NHI(String);

impl NHI {
    /// Extracts a string slice containing this NHI number's underlying string value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts this NHI to its underlying String value
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns `true` if this NHI is reserved for testing and `false` otherwise
    pub fn is_test(&self) -> bool {
        self.0.starts_with('Z')
    }

    /// Returns `true` if this NHI is NOT reserved for testing and `false` otherwise
    pub fn is_not_test(&self) -> bool {
        !self.0.starts_with('Z')
    }
}

impl fmt::Display for NHI {
    /// Formats this NHI as its underlying NHI value
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Empty struct to indicate an invalid NHI string
#[derive(Debug)]
pub struct NHIParseError;

impl FromStr for NHI {
    type Err = NHIParseError;

    /// Parses a string to an [NHI] iff the given string satisfies the
    /// [HISO 10046:2023](https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/)
    /// standard, otherwise returns an error.
    ///
    /// # Arguments
    ///
    /// * `s`: a potential NHI string
    ///
    /// returns: Result<NHI, ParseNHIError>
    ///
    /// # Examples
    ///
    /// ```
    /// use nhi::NHI;
    /// use std::str::FromStr;
    ///
    /// let nhi = NHI::from_str("zbn77vl").unwrap();
    /// assert_eq!(nhi.as_str(), "ZBN77VL")
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nhi = s.to_uppercase();
        if OLD_NHI_FORMAT.is_match(&nhi) {
            let checksum = checksum(&nhi) % 11;
            let check_digit = (11 - checksum) % 10;
            if checksum != 0 && check_digit == char_code(nhi.chars().last().unwrap()) {
                return Ok(NHI(nhi));
            }
        } else if NEW_NHI_FORMAT.is_match(&nhi) {
            let checksum = checksum(&nhi) % 23;
            let check_digit = 23 - checksum;
            if check_digit == char_code(nhi.chars().last().unwrap()) {
                return Ok(NHI(nhi));
            }
        }
        Err(NHIParseError)
    }
}

/// Checks a string against the New Zealand Ministry of Health NHI specification
/// defined by the
/// [HISO 10046:2023](https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/)
/// standard
///
/// # See Also
/// <https://www.tewhatuora.govt.nz/publications/hiso-100462023-consumer-health-identity-standard/>
///
/// # Arguments
///
/// * `nhi`: A potential NHI string
///
/// returns: `true` if the given string satisfies the New Zealand NHI Validation Routine and `false` otherwise
///
/// # Examples
///
/// ```
/// use nhi::is_nhi;
///
/// assert_eq!(is_nhi("ZAC5361"), true);
/// assert_eq!(is_nhi("ZBN77VL"), true);
/// assert_eq!(is_nhi("ZZZ0044"), false);
/// assert_eq!(is_nhi("ZZZ00AA"), false);
/// ```
pub fn is_nhi(nhi: &str) -> bool {
    NHI::from_str(nhi).is_ok()
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

    const VALID_OLD: [&str; 15] = [
        "JBX3656", "ZZZ0016", "ZZZ0024", "ZAA0067", "ZAA0075", "ZAA0083", "ZAA0091",
        "ZAA0105", "ZAA0113", "ZAA0121", "ZAA0130", "ZAA0148", "ZAA0156", "ZAC5361",
        "ABC1235",
    ];
    const VALID_NEW: [&str; 11] = [
        "ZBN77VL", "ZZZ00AC", "ZDR69YX", "ZSC21TN", "ZZB30NH", "ZYZ81ZV", "ZVB97XQ",
        "ZRA29VA", "ZYX61YS", "ABC12AY", "XYZ12AN",
    ];
    const INVALID_OLD: [&str; 3] = ["ZZZ0044", "ZZZ0017", "DAB8233"];
    const INVALID_NEW: [&str; 4] = ["ZZZ00AA", "ZZZ00AY", "ZVU27KY", "ZVU27KA"];
    const RANDOM_STRINGS: [&str; 7] = [
        "not an NHI", "!@#$%&*", "AAANNNC", "AAANNAC", "ZVU27K", "JBX365", ""
    ];

    #[test]
    fn is_nhi_recognises_valid_old_format_nhi_numbers() {
        for nhi in VALID_OLD {
            assert!(is_nhi(nhi));
        }
    }

    #[test]
    fn is_nhi_rejects_invalid_old_format_nhi_numbers() {
        for nhi in INVALID_OLD {
            assert!(!is_nhi(nhi));
        }
        // Needs a check digit of 6
        for i in 0..10 {
            if i != 6 {
                assert!(!is_nhi(&format!("JBX365{i}")));
            }
        }
    }

    #[test]
    fn no_digit_can_be_added_to_an_old_format_nhi_with_a_checksum_of_0_to_make_it_valid() {
        for i in 0..10 {
            assert!(!is_nhi(&format!("ZZZ004{i}")));
        }
    }

    #[test]
    fn is_nhi_recognises_valid_new_format_nhi_numbers() {
        for nhi in VALID_NEW {
            assert!(is_nhi(nhi));
        }
    }

    #[test]
    fn is_nhi_rejects_invalid_new_format_nhi_numbers() {
        for nhi in INVALID_NEW {
            assert!(!is_nhi(nhi));
        }
        // Needs a check character of V
        for c in "ABCDEFGHJKLMNPQRSTUWXYZ".chars() {
            assert!(!is_nhi(&format!("ZHW58C{c}")))
        }
    }

    #[test]
    fn is_nhi_rejects_random_strings() {
        for nhi in RANDOM_STRINGS {
            assert!(!is_nhi(nhi));
        }
    }

    #[test]
    fn is_nhi_is_case_insensitive() {
        for nhi in VALID_OLD.iter().chain(VALID_NEW.iter()) {
            assert!(is_nhi(&nhi.to_lowercase()))
        }
        for nhi in INVALID_OLD.iter().chain(INVALID_NEW.iter()).chain(RANDOM_STRINGS.iter()) {
            assert!(!is_nhi(&nhi.to_lowercase()))
        }
    }

    #[test]
    fn nhi_numbers_can_be_parsed_from_strings_to_results() {
        for nhi_str in VALID_OLD.iter().chain(VALID_NEW.iter()) {
            assert!(NHI::from_str(nhi_str).is_ok());
        }
        for nhi_str in INVALID_OLD.iter().chain(INVALID_NEW.iter()).chain(RANDOM_STRINGS.iter()) {
            assert!(NHI::from_str(nhi_str).is_err());
        }
    }

    #[test]
    fn nhi_numbers_can_be_converted_to_strings() {
        for nhi_str in VALID_OLD.iter().chain(VALID_NEW.iter()) {
            let nhi = NHI::from_str(&nhi_str.to_lowercase()).unwrap();
            assert_eq!(nhi.as_str(), nhi_str.to_uppercase());
            assert_eq!(nhi.into_string(), nhi_str.to_uppercase());
        }
    }

    #[test]
    fn nhi_numbers_identify_values_reserved_for_testing() {
        let reserved = vec!["ZAA0105", "ZAA0113", "ZBN77VL", "ZZZ00AC"];
        let unreserved = vec!["JBX3656", "ABC1235", "ABC12AY", "XYZ12AN"];
        for nhi in reserved {
            let nhi: NHI = nhi.parse().unwrap();
            assert!(nhi.is_test());
            assert!(!nhi.is_not_test());
        }
        for nhi in unreserved {
            let nhi: NHI = nhi.parse().unwrap();
            assert!(!nhi.is_test());
            assert!(nhi.is_not_test());
        }
    }

    #[test]
    fn nhi_numbers_can_be_formatted() {
        for nhi_str in VALID_OLD.iter().chain(VALID_NEW.iter()) {
            let nhi = NHI::from_str(&nhi_str.to_lowercase()).unwrap();
            assert_eq!(format!("{nhi}"), nhi_str.to_uppercase());
        }
    }

    #[test]
    fn char_codes() {
        for (i, c) in ('A'..'I').enumerate() {
            assert_eq!(char_code(c), i as u32 + 1);
        }
        for (i, c) in ('J'..'O').enumerate() {
            assert_eq!(char_code(c), i as u32 + 9);
        }
        for (i, c) in ('P'..='Z').enumerate() {
            assert_eq!(char_code(c), i as u32 + 14);
        }
    }
}
