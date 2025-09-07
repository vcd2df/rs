use polars::prelude::*;
use vcd2df::vcd2df;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("Please provide a file as the first argument");
    }

    let name = args.remove(1);
    let mut df = vcd2df(&name);

    let mut file =
        std::fs::File::create(name.split(".").next().unwrap().to_owned() + ".parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
}
