// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
use polars::prelude::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn vcd2pl<P: AsRef<Path>>(filename: P) -> DataFrame {
    let mut lines = read_lines(filename).expect("File DNE");
    let mut names = BTreeMap::<String, String>::new();
    let mut stage = 0;
    // Stage 0: Read to `dumpvars`
    while stage == 0 {
        let line = lines
            .next()
            .expect("VCD ill-formed wrt dumpvars.")
            .expect("VCD ill-formed wrt lines.");
        if line.trim() == "$dumpvars" {
            stage = 1;
        }
        let mut splits = line.split(" ");
        let word = splits.next().expect("VCD line ill-formed.");
        if word == "$var" {
            splits.next();
            splits.next(); // Consume var/reg and size
            names.insert(
                String::from(splits.next().expect("Varname illformed")), // VCD nickname
                String::from(splits.next().expect("Varname illformed")), // Verilog reg/var name
            );
        }
    }
    // Intermediate - stage the value storage
    // Polars can interpret integer options as nullable ints 
    // Pico uses 32 bit, use 64 until we figure out to how to select.
    let mut curr = BTreeMap::<String, Option<u64>>::new();
    for key in names.keys() {
        curr.insert(key.clone(), None);
    }
    let names: Vec<String> = names.into_values().collect();
    let mut times: Vec<Column> = vec![Column::new("Names".into(), names)];
    let mut time = String::from("#0");
    // Stage 1: Read times into a BTreeMap
    while let Some(Ok(line)) = lines.next() {
        if line.chars().nth(0).expect("Line ill-formed") == '#' {
            let tmp: Vec<Option<u64>> = curr.values().cloned().collect();
            times.push(Column::new(time.into(), tmp));
            time = String::from(&line);
        }
        // Two cases - singular or plural
        if line.contains(char::is_whitespace) {
            // Plural
            let mut splits = line.split(" ");
            let mut num = splits.next().unwrap().chars();
            num.next(); // Clip the 'b'
            let num = num.as_str();
            let num = u64::from_str_radix(num, 2).ok();
            let reg = splits.next().unwrap();
            if curr.contains_key(reg) {
                curr.insert(String::from(reg), num);
            }
        } else {
            let mut line = line.chars();
            let num = u64::from_str_radix(&line.next().unwrap().to_string(), 2).ok();
            let reg = line.as_str();
            if curr.contains_key(reg) {
                curr.insert(String::from(reg), num);
            }
        }
    }
    DataFrame::new(times).unwrap()
}
