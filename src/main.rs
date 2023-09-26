#![allow(unused)]

extern crate cf_rust;
use cf_rust::*;

use std::collections::*;
use std::io::{self, BufWriter, StdoutLock, Write};

macro_rules! readline {
    ($($x:ty),+) => {unsafe {
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
        let mut iter = buf.split(char::is_whitespace);
        ($(iter.next().unwrap_unchecked().parse::<$x>().unwrap_unchecked()),*)
    }}
}

unsafe fn solve(out: &mut BufWriter<StdoutLock<'_>>) {}

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let t = readline!(i32);
    for _ in 0..t {
        unsafe {
            solve(&mut out);
        }
    }
}
