use crate::prelude::*;

pub fn gen_words_12() -> [&'static str; 12] {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..2048);
    let mut words: [&'static str; 12] = [""; 12];
    for word in &mut words {
        *word = WORDS[between.sample(&mut rng)];
    }
    words
}

pub fn gen_words_24() -> [&'static str; 24] {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..2048);
    let mut words: [&'static str; 24] = [""; 24];
    for word in &mut words {
        *word = WORDS[between.sample(&mut rng)];
    }
    words
}

pub fn words_12_to_str<'a>(arr: [&'static str; 12], buffer: &'a mut String) -> &'a str {
    buffer.clear();
    buffer.push_str(&arr.join(" "));
    &buffer[..]
}
