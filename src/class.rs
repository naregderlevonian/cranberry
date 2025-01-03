use crate::alias::Cyrillic;

pub fn is_vowel(current: &Cyrillic) -> bool {
    ['А', 'а', 'О', 'о', 'У', 'у', 'Ы', 'ы', 'Э', 'э', 'И', 'и'].contains(&current)
}

pub fn is_iotized(current: &Cyrillic) -> bool {
    ['Е', 'е', 'Ё', 'ё', 'Ю', 'ю', 'Я', 'я'].contains(&current)
}

pub fn is_sibilant(current: &Cyrillic) -> bool {
    ['Ж', 'ж', 'Ш', 'ш', 'Щ', 'щ', 'Ч', 'ч'].contains(&current)
}

pub fn is_hard(current: &Cyrillic) -> bool {
    ['Ъ', 'ъ'].contains(&current)
}

pub fn is_soft(current: &Cyrillic) -> bool {
    ['Ь', 'ь'].contains(&current)
}
