use crate::alias::Alphabet;
use crate::alias::Cyrillic;
use crate::alias::Latin;
use crate::class;
use crate::iotized;

fn process(
    alphabet: &Alphabet,
    current: &Cyrillic,
    prev: &Option<Cyrillic>,
    next: &Option<Cyrillic>,
) -> Latin {
    match alphabet.get(&current) {
        Some(converted) => {
            if class::is_iotized(current) {
                if let Some(prev) = prev {
                    if !alphabet.contains_key(&prev)
                        || class::is_hard(&prev)
                        || class::is_vowel(&prev)
                        || class::is_iotized(&prev)
                        || class::is_soft(&prev)
                    {
                        return iotized::jot::get()
                            .get(&current)
                            .unwrap_or(&converted.clone())
                            .to_string();
                    }

                    if class::is_sibilant(prev) {
                        return iotized::basis::get()
                            .get(&current)
                            .unwrap_or(&converted.clone())
                            .to_string();
                    }
                } else {
                    return iotized::jot::get()
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

pub fn translate(source: &String, alphabet: &Alphabet) -> Latin {
    let chars: Vec<Cyrillic> = source.chars().collect();
    let mut result = String::new();

    for i in 0..chars.len() {
        let current = chars[i];
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = if i < chars.len() - 1 {
            Some(chars[i + 1])
        } else {
            None
        };

        result.push_str(&process(&alphabet, &current, &prev, &next));
    }

    result
}
