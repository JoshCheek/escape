pub fn escape_string(str:&str) -> String {
  str.chars()
     .map(escape_char)
     .fold(String::new(), |a:String, b:String| {
       a + &b
     })
}

pub fn escape_char(c:char) -> String {
       if c == '\u{0}'  { "\\x00".to_string() }
  else if c == '\u{1}'  { "\\x01".to_string() }
  else if c == '\u{2}'  { "\\x02".to_string() }
  else if c == '\u{3}'  { "\\x03".to_string() }
  else if c == '\u{4}'  { "\\x04".to_string() }
  else if c == '\u{5}'  { "\\x05".to_string() }
  else if c == '\u{6}'  { "\\x06".to_string() }
  else if c == '\u{7}'  { "\\a".to_string() }
  else if c == '\u{8}'  { "\\b".to_string() }
  else if c == '\u{9}'  { "\\t".to_string() }
  else if c == '\u{a}' { "\\n".to_string() }
  else if c == '\u{b}' { "\\v".to_string() }
  else if c == '\u{c}' { "\\f".to_string() }
  else if c == '\u{d}' { "\\r".to_string() }
  else                  { format!("NOTHING FOR {:?}", c) }
}
