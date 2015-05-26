extern crate escape;
use escape::{escape_string, escape_stream};

pub fn main() {
    let instream  = std::io::stdin();
    let inlock    = &mut instream.lock();

    let outstream = std::io::stdout();
    let outlock   = &mut outstream.lock();

    escape_stream(inlock, outlock);
}
