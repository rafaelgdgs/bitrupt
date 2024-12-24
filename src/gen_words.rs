use crate::prelude::*;

pub fn gen_words() -> [&'static str; 12] {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..2048);
    let mut words: [&'static str; 12] = [""; 12];
    for word in &mut words {
        *word = WORDS[between.sample(&mut rng)];
    }
    words
}
