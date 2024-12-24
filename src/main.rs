mod gen_words;
mod pre_proc;
mod words;

#[allow(unused)]
mod prelude {
    pub use crate::gen_words::gen_words;
    pub use crate::pre_proc::show;
    pub use crate::words::WORDS;
}

use crate::prelude::*;

fn main() {
    // show();
    let x = gen_words();
    println!("{:?}", x);
}
