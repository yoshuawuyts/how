use std::io::BufReader;
use std::fs::File;
use std::io::Read;

// how
pub fn how (location: &str) {
  let f = File::open(location).unwrap();
  let mut rs = BufReader::new(f);

  let mut file = String::new();
  rs.read_to_string(&mut file).unwrap();
  parse_markdown(file)
}

// parse markdown string
fn parse_markdown (md: String) {
  println!("{}", md);
}
