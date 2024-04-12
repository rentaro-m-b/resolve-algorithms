use std::env::{self, Args};

pub fn input() -> Args {
    let args = env::args();
    args
}
