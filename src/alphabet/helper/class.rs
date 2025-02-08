use crate::alias::Cyrillic;

pub fn is_vowel(cyrillic: &Cyrillic) -> bool {
    matches!(cyrillic, 'А' | 'а' | 'О' | 'о' | 'У' | 'у' | 'Ы' | 'ы' | 'Э' | 'э' | 'И' | 'и')
}

pub fn is_consonant(cyrillic: &Cyrillic) -> bool {
    matches!(cyrillic, 'Б' | 'б' | 'В' | 'в' | 'Г' | 'г' | 'Д' | 'д' | 'Ж' | 'ж' | 'З' | 'з' | 'Й' | 'й' | 'К' | 'к' | 'Л' | 'л' | 'М' | 'м' | 'Н' | 'н' | 'П' | 'п' | 'Р' | 'р' | 'С' | 'с' | 'Т' | 'т' | 'Ф' | 'ф' | 'Х' | 'х' | 'Ц' | 'ц' | 'Ч' | 'ч' | 'Ш' | 'ш' | 'Щ' | 'щ' )
}

pub fn is_iotized(cyrillic: &Cyrillic) -> bool {
    matches!(cyrillic, 'Е' | 'е' | 'Ё' | 'ё' | 'Ю' | 'ю' | 'Я' | 'я')
}

pub fn is_sibilant(cyrillic: &Cyrillic) -> bool {
    matches!(cyrillic, 'Ж' | 'ж' | 'Ш' | 'ш' | 'Щ' | 'щ' | 'Ч' | 'ч')
}

pub fn is_hard(cyrillic: &Cyrillic) -> bool {
    matches!(cyrillic, 'Ъ' | 'ъ')
}

pub fn is_soft(cyrillic: &Cyrillic) -> bool {
    matches!(cyrillic, 'Ь' | 'ь')
}

pub fn is_jot(cyrillic: &Cyrillic) -> bool {
    matches!(cyrillic, 'Й' | 'й')
}
