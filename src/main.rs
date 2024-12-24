mod gen_words;
mod pre_proc;
mod words;

#[allow(unused)]
mod prelude {
    pub use crate::gen_words::gen_words_12;
    pub use crate::gen_words::gen_words_24;
    pub use crate::pre_proc::generate;
    pub use crate::words::WORDS;
}

use crate::prelude::*;

fn main() {
    // generate();
    let x = gen_words_12();
    println!("{:?}", x);
}
