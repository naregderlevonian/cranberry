use std::collections::HashMap;

use crate::dictionary::check;
use crate::dictionary;

pub fn parse(data: HashMap<char, String>, current: char, prev: Option<char>, next: Option<char>) -> String {
    match data.get(&current) {
        Some(converted) => {

            if dictionary::check::is_iotized(current) {
                if let Some(prev) = prev {

                    if !data.contains_key(&prev) || dictionary::check::is_hard(prev) || dictionary::check::is_vowel(prev) || dictionary::check::is_soft(prev) {
                        return dictionary::iotized_digram::new().get(&current).unwrap_or(&converted.clone()).to_string();
                    }

                    if dictionary::check::is_sibilant(prev) {
                        return dictionary::iotized_basis::new().get(&current).unwrap_or(&converted.clone()).to_string();
                    }

                } else {
                    return dictionary::iotized_digram::new().get(&current).unwrap_or(&converted.clone()).to_string();
                }
            }

            if dictionary::check::is_soft(current) {

                if let Some(prev) = prev {
                    if dictionary::check::is_sibilant(prev) {
                        return String::from("");
                    }
                }

                if let Some(next) = next {
                    if dictionary::check::is_iotized(next) {
                        return String::from("");
                    }
                }
            }

            converted.clone()
        },
        None => current.to_string(),
    }
}
