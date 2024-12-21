pub mod lib {
  use std::{fs::File, io::Read};
  
  pub fn read_file(path: &str) -> std::io::Result<String> {
    let mut file= File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
  }
}
