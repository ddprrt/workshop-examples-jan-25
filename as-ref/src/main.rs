use std::{path::Path, io::{self, Read}, fs::File};

pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    fn inner(path: &Path) -> io::Result<Vec<u8>> {
      let mut file = File::open(path)?;
      let mut bytes = Vec::new();
      file.read_to_end(&mut bytes)?;
      Ok(bytes)
    }
    inner(path.as_ref())
  }
  

fn main() {
    let content = read("test.txt").unwrap();
    println!("{:?}", content);
    let content = read(String::from("test.txt")).unwrap();
    println!("{:?}", content);
    let x = String::from("test.txt");
    let content = read(&x).unwrap();
    println!("{:?}", content);
}
