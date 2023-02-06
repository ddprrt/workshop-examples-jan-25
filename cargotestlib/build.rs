use rand::prelude::*;
use std::{fs::File, io::Write};

fn main() {
    let mut f = File::create("text.txt").unwrap();
    f.write(format!("{}", random::<u128>()).as_bytes()).unwrap();
}
