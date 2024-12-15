pub fn is_hard(current: char) -> bool {
    ['Ъ', 'ъ'].contains(&current)
}

pub fn is_iotized(current: char) -> bool {
    ['Е', 'е', 'Ё', 'ё', 'Ю', 'ю', 'Я', 'я'].contains(&current)
}

pub fn is_sibilant(current: char) -> bool {
    ['Ж', 'ж', 'Ш', 'ш', 'Щ', 'щ', 'Ч', 'ч'].contains(&current)
}

pub fn is_soft(current: char) -> bool {
    ['Ь', 'ь'].contains(&current)
}

pub fn is_vowel(current: char) -> bool {
    ['А', 'а', 'О', 'о', 'У', 'у', 'Ы', 'ы', 'Э', 'э', 'Я', 'я', 'Ё', 'ё', 'Ю', 'ю', 'И', 'и', 'Е',
        'е',].contains(&current)
}
