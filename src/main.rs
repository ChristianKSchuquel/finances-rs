mod finances;
use std::env;

use crate::finances::setup;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let y = finances::setup();

    todo!()
}
