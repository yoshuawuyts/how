use std::io::BufReader;
use std::fs::File;
use std::io::Read;

// how
pub fn how (location: &str) {
  let f = File::open(location).unwrap();
  let mut rs = BufReader::new(f);

  let mut file = String::new();
  rs.read_to_string(&mut file).unwrap();

  println!("{}", file);
}
