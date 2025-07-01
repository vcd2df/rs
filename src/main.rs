#![feature(test)]
extern crate test;
use rs::vcd2pl;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("Please provide a file as the first argument");
    }

    let name = args.remove(1);
    vcd2pl(name);
}

#[cfg(test)]
pub mod tests {
    use rs::vcd2pl;
    use test::Bencher;

    #[bench]
    fn testbench(b: &mut Bencher) {
        let name = format!("{}/tests/testbench.vcd", env!("CARGO_MANIFEST_DIR"));
        b.iter(|| vcd2pl(&name));
    }
}
