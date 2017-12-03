use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input").expect("err");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("err");
    let sum =
    buffer.split("\n")
    .filter(|i| !i.is_empty())
    .map(
        |s| s.split("\t")
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
    )
    .map(|v: Vec<i32>| -> i32 {
        v.iter().max().unwrap() - v.iter().min().unwrap()
    })
    .fold(0, |sum, i| sum + i   );
    println!("{}", sum);
}
