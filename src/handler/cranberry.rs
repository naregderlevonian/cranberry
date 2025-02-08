use crate::alias::Alphabet;
use crate::alias::Cyrillic;
use crate::alias::Latin;
use crate::alphabet::helper::class;
use crate::alphabet::exclusion;

pub fn process(
    alphabet: &Alphabet,
    current: &Cyrillic,
    prev: Option<&Cyrillic>,
    next: Option<&Cyrillic>,
) -> Latin {
    match alphabet.get(&current) {
        Some(converted) => {
            if class::is_iotized(current) {
                if let Some(prev) = prev {
                    if !alphabet.contains_key(&prev)
                    || !class::is_consonant(&prev)
                    {
                        return exclusion::cranberry::get()
                            .get(&current)
                            .unwrap_or(&converted.clone())
                            .to_string();
                    }

                    if class::is_sibilant(prev) {
                        return exclusion::basis::get()
                            .get(&current)
                            .unwrap_or(&converted.clone())
                            .to_string();
                    }
                } else {
                    return exclusion::cranberry::get()
                        .get(&current)
                        .unwrap_or(&converted.clone())
                        .to_string();
                }
            }

            if class::is_soft(current) {
                if let Some(prev) = prev {
                    if class::is_sibilant(&prev) {
                        return String::from("");
                    }
                }

                if let Some(next) = next {
                    if class::is_iotized(&next) {
                        return String::from("");
                    }
                }
            }

            converted.clone()
        }
        None => current.to_string(),
    }
}

