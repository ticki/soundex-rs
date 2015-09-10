use super::SoundexWord;

#[test]
fn soundex_reduce() {
    assert!(&SoundexWord::new("abc").to_string() == "A12");
    assert!(&SoundexWord::new("hello").to_string() == "H4");
    assert!(&SoundexWord::new("STUPID").to_string() == "S313");
    assert!(&SoundexWord::new("What").to_string() == "W3");
}

#[test]
fn sounds_similar() {
    assert!(SoundexWord::new("rupert") == SoundexWord::new("robert"));
    assert!(SoundexWord::new("hey") == SoundexWord::new("hei"));
    assert!(SoundexWord::new("sOundex") == SoundexWord::new("saundecs"));
}

#[test]
fn does_not_sound_similar() {
    assert!(SoundexWord::new("cool") != SoundexWord::new("fool"));
    assert!(SoundexWord::new("nope") != SoundexWord::new("doop"));
    assert!(SoundexWord::new("goat") != SoundexWord::new("goal"));
}
