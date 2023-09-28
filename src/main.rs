#![allow(unused)]

extern crate cf_rust;
use cf_rust::*;

use std::collections::*;
use std::io::{self, BufWriter, StdoutLock, Write};
use std::ops::Bound::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pair<U, V> {
    first: U,
    second: V,
}

macro_rules! readline {
    ($($x:ty),+) => {{
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
        let mut iter = buf.split(char::is_whitespace);
        ($(iter.next().unwrap().parse::<$x>().unwrap()),*)
    }}
}

fn solve(out: &mut BufWriter<StdoutLock<'_>>) {}

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let t = readline!(i32);
    for _ in 0..t {
        solve(&mut out);
    }
}
