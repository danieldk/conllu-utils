use unicode_normalization::UnicodeNormalization;

/// Types of unicode normalization.
#[derive(Copy, Clone)]
pub enum Normalization {
    None,
    NFD,
    NFKD,
    NFC,
    NFKC,
}

fn normalization_iter<'a, I>(iter: I, norm: Normalization) -> Box<dyn Iterator<Item = char> + 'a>
where
    I: 'a + Iterator<Item = char>,
{
    use self::Normalization::*;

    match norm {
        None => Box::new(iter),
        NFD => Box::new(iter.nfd()),
        NFKD => Box::new(iter.nfkd()),
        NFC => Box::new(iter.nfc()),
        NFKC => Box::new(iter.nfkc()),
    }
}

pub enum Conversion {
    Char(char),
    String(String),
    None(char),
}

// Source of punctuation Unicode -> ASCII mappings:
// http://lexsrv3.nlm.nih.gov/LexSysGroup/Projects/lvg/current/docs/designDoc/UDF/unicode/DefaultTables/symbolTable.html
pub fn simplify_unicode_lookup(c: char) -> Conversion {
    match c {
        '«' => Conversion::Char('"'),
        '´' => Conversion::Char('\''),
        '»' => Conversion::Char('"'),
        '÷' => Conversion::Char('/'),
        'ǀ' => Conversion::Char('|'),
        'ǃ' => Conversion::Char('!'),
        'ʹ' => Conversion::Char('\''),
        'ʺ' => Conversion::Char('"'),
        'ʼ' => Conversion::Char('\''),
        '˄' => Conversion::Char('^'),
        'ˆ' => Conversion::Char('^'),
        'ˈ' => Conversion::Char('\''),
        'ˋ' => Conversion::Char('`'),
        'ˍ' => Conversion::Char('_'),
        '˜' => Conversion::Char('~'),
        '։' => Conversion::Char(':'),
        '׀' => Conversion::Char('|'),
        '׃' => Conversion::Char(':'),
        '٪' => Conversion::Char('%'),
        '٭' => Conversion::Char('*'),
        '‐' => Conversion::Char('-'),
        '‑' => Conversion::Char('-'),
        '‒' => Conversion::Char('-'),
        '–' => Conversion::Char('-'),
        '—' => Conversion::Char('-'),
        '―' => Conversion::Char('-'),
        '‗' => Conversion::Char('_'),
        '‘' => Conversion::Char('\''),
        '’' => Conversion::Char('\''),
        '‚' => Conversion::Char(','),
        '‛' => Conversion::Char('\''),
        '“' => Conversion::Char('"'),
        '”' => Conversion::Char('"'),
        '„' => Conversion::Char('"'),
        '‟' => Conversion::Char('"'),
        '′' => Conversion::Char('\''),
        '″' => Conversion::Char('"'),
        '‵' => Conversion::Char('`'),
        '‶' => Conversion::Char('"'),
        '‸' => Conversion::Char('^'),
        '‹' => Conversion::Char('<'),
        '›' => Conversion::Char('>'),
        '‽' => Conversion::String("?!".to_string()),
        '⁄' => Conversion::Char('/'),
        '⁎' => Conversion::Char('*'),
        '⁒' => Conversion::Char('%'),
        '⁓' => Conversion::Char('~'),
        '−' => Conversion::Char('-'),
        '∕' => Conversion::Char('/'),
        '∖' => Conversion::Char('\\'),
        '∗' => Conversion::Char('*'),
        '∣' => Conversion::Char('|'),
        '∶' => Conversion::Char(':'),
        '∼' => Conversion::Char('~'),
        '⌃' => Conversion::Char('^'),
        '♯' => Conversion::Char('#'),
        '✱' => Conversion::Char('*'),
        '❘' => Conversion::Char('|'),
        '❢' => Conversion::Char('!'),
        '⟦' => Conversion::Char('['),
        '⟨' => Conversion::Char('<'),
        '⟩' => Conversion::Char('>'),
        '⦃' => Conversion::Char('{'),
        '⦄' => Conversion::Char('}'),
        '〃' => Conversion::Char('"'),
        '〈' => Conversion::Char('<'),
        '〉' => Conversion::Char('>'),
        '〛' => Conversion::Char(']'),
        '〜' => Conversion::Char('~'),
        '〝' => Conversion::Char('"'),
        '〞' => Conversion::Char('"'),
        '‖' => Conversion::String("||".to_string()),
        '‴' => Conversion::String("'''".to_string()),
        '‷' => Conversion::String("'''".to_string()),
        '≤' => Conversion::String("<=".to_string()),
        '≥' => Conversion::String(">=".to_string()),
        '≦' => Conversion::String("<=".to_string()),
        '≧' => Conversion::String(">=".to_string()),
        '…' => Conversion::String("...".to_string()),

        // Fractions
        '¼' => Conversion::String("1/4".to_string()),
        '½' => Conversion::String("1/2".to_string()),
        '¾' => Conversion::String("3/4".to_string()),
        '⅐' => Conversion::String("1/7".to_string()),
        '⅑' => Conversion::String("1/9".to_string()),
        '⅒' => Conversion::String("1/10".to_string()),
        '⅓' => Conversion::String("1/3".to_string()),
        '⅔' => Conversion::String("2/3".to_string()),
        '⅕' => Conversion::String("1/5".to_string()),
        '⅖' => Conversion::String("2/5".to_string()),
        '⅗' => Conversion::String("3/5".to_string()),
        '⅘' => Conversion::String("4/5".to_string()),
        '⅙' => Conversion::String("1/6".to_string()),
        '⅚' => Conversion::String("5/6".to_string()),
        '⅛' => Conversion::String("1/8".to_string()),
        '⅜' => Conversion::String("3/8".to_string()),
        '⅝' => Conversion::String("5/8".to_string()),
        '⅞' => Conversion::String("7/8".to_string()),
        '⅟' => Conversion::String("1/".to_string()),
        '↉' => Conversion::String("0/3".to_string()),

        // Subscript/superscript
        '⁻' => Conversion::Char('-'),
        '⁰' => Conversion::Char('0'),
        '¹' => Conversion::Char('1'),
        '²' => Conversion::Char('2'),
        '³' => Conversion::Char('3'),
        '⁴' => Conversion::Char('4'),
        '⁵' => Conversion::Char('5'),
        '⁶' => Conversion::Char('6'),
        '⁷' => Conversion::Char('7'),
        '⁸' => Conversion::Char('8'),
        '⁹' => Conversion::Char('9'),

        // Subscript
        '₋' => Conversion::Char('-'),
        '₀' => Conversion::Char('0'),
        '₁' => Conversion::Char('1'),
        '₂' => Conversion::Char('2'),
        '₃' => Conversion::Char('3'),
        '₄' => Conversion::Char('4'),
        '₅' => Conversion::Char('5'),
        '₆' => Conversion::Char('6'),
        '₇' => Conversion::Char('7'),
        '₈' => Conversion::Char('8'),
        '₉' => Conversion::Char('9'),

        _ => Conversion::None(c),
    }
}

pub fn simplify_unicode(s: &str, norm: Normalization) -> String {
    normalization_iter(s.chars(), norm).fold(String::with_capacity(s.len()), |mut s, c| {
        match simplify_unicode_lookup(c) {
            Conversion::Char(c) => s.push(c),
            Conversion::String(ss) => s.push_str(&ss),
            Conversion::None(c) => s.push(c),
        }

        s
    })
}
