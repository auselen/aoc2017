use std::fs::File;
use std::io::prelude::*;
use std::num::Wrapping;

fn main() {
    let mut f = File::open("input").expect("err");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("err");
    let mut b = buffer.iter().zip(buffer[1..].iter())
        .filter(|&(a, b)| a == b)
        .map(|(a, _) | a - b'0')
        .fold(Wrapping(0u32), |sum, i| sum + Wrapping(i as u32));
    if buffer[0] == buffer[buffer.len() - 2] {
        b += Wrapping((buffer[0] - b'0') as u32);
    }
    println!("{}", b);
}
