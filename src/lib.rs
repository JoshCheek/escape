pub fn escape_string(str:&str) -> String {
  str.chars()
     .map(escape_char)
     .fold(String::new(), |a:String, b:String| {
       a + &b
     })
}

pub fn escape_char(c:char) -> String {
       if '\u{0}' == c { "\\x00".to_string() }
  else if '\u{1}' == c { "\\x01".to_string() }
  else if '\u{2}' == c { "\\x02".to_string() }
  else if '\u{3}' == c { "\\x03".to_string() }
  else if '\u{4}' == c { "\\x04".to_string() }
  else if '\u{5}' == c { "\\x05".to_string() }
  else if '\u{6}' == c { "\\x06".to_string() }
  else            { c.to_string() }
}
