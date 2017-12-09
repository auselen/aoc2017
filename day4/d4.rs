use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut f = File::open("input").expect("err");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("err");
    let sum =
    buffer.split("\n")
    .filter(|i| !i.is_empty())
    .filter(|i| {
        let c1 = i.split(" ").count();
        let h1: HashSet<String> = HashSet::from_iter(
            i.split(" ").map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort(); // comment out for first answer
                chars.into_iter().collect()
                }
        ));
        c1 == h1.len()
    }).count();
    println!("{}", sum);
}
