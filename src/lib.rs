pub fn escape_string(str:String) -> String {
  str.chars()
     .map(escape_char)
     .fold(String::new(), |a, b| { a + &b })
}

pub fn escape_char(c:char) -> String {
  let cu32 = c as u32;

  match cu32 {
    32 ... 126 => { c.to_string() },
    0  ...  6 |
    14 ... 26 |
    28 ... 31 |
    127 => { format!("\\x{:02X}", cu32) },
    7   => { "\\a".to_string() },
    8   => { "\\b".to_string() },
    9   => { "\\t".to_string() },
    10  => { "\\n".to_string() },
    11  => { "\\v".to_string() },
    12  => { "\\f".to_string() },
    13  => { "\\r".to_string() },
    27  => { "\\e".to_string() },
    _   => { c.to_string()     },
  }
}
