extern crate itertools;

use itertools::Itertools;


#[derive(PartialEq, Hash, Clone)]
pub enum SoundexChar {
    // B, F, P, V
    S1,
    // C, G, J, K, Q, S, X, Z
    S2,
    // D, T
    S3,
    // L
    S4,
    // M, N
    S5,
    // R
    S6,
    // Vowels
    Vowel,
    // H, W
    HW,
    // Space
    Space,
}

fn encode(ch: char) -> SoundexChar {
    if let Some(c) = ch.to_lowercase().next() {
        match c {
            'b' | 'f' | 'p' | 'v'                         => SoundexChar::S1,
            'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => SoundexChar::S2,
            'd' | 't'                                     => SoundexChar::S3,
            'l'                                           => SoundexChar::S4,
            'm' | 'n'                                     => SoundexChar::S5,
            'r'                                           => SoundexChar::S6,
            'h' | 'w'                                     => SoundexChar::HW,
            ' '                                           => SoundexChar::Space,
            _                                             => SoundexChar::Vowel,

        }
    } else {
        SoundexChar::Vowel
    }
}

#[derive(PartialEq, Hash, Clone)]
pub struct SoundexWord {
    first_char: char,
    soundex_chars: Vec<SoundexChar>,
}

impl SoundexWord {
    fn new(word: &str) -> SoundexWord {
        let chars = word.chars();
        let first_char = chars.clone().nth(0).unwrap_or('_')
                              .to_uppercase().next().unwrap();
        let soundex_chars = chars.map(|x| encode(x)).skip(1)
                                 .filter(|x| *x != SoundexChar::HW)
                                 .dedup()
                                 .filter(|x| *x != SoundexChar::Vowel)
                                 .collect::<Vec<_>>();
        SoundexWord {
            first_char: first_char,
            soundex_chars: soundex_chars,
        }
    }
    fn to_string(&self) -> String {
        self.first_char.to_string() +
        &self.soundex_chars.iter().map(|x| {
            match *x {
                SoundexChar::S1 => "1",
                SoundexChar::S2 => "2",
                SoundexChar::S3 => "3",
                SoundexChar::S4 => "4",
                SoundexChar::S5 => "5",
                SoundexChar::S6 => "6",
                SoundexChar::Space => " ",
                _               => "_",
            }.to_string()
        }).join("")
    }
}

#[cfg(test)]
mod tests;
