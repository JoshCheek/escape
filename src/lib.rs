pub fn escape_string(str:&str) -> String {
  str.chars()
     .map(escape_char)
     .fold(String::new(), |a:String, b:String| {
       a + &b
     })
}

pub fn escape_char(c:char) -> String {
  c.to_string()
}
