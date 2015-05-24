pub fn escape_string(str:&str) -> String {
  str.chars()
     .map(|a| { "a".to_string() })
     .fold("".to_string(), |a:String, b:String| {
       a + &b
     })
}

pub fn escape_char(c:char) -> String {
  "a".to_string()
}
