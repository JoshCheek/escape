pub fn escape_string(str:&str) -> String {
  str.chars()
     .map(escape_char)
     .fold(String::new(), |a, b| { a + &b })
}

pub fn escape_char(c:char) -> String {
  let cu32 = c as u32;
  if cu32 < 7                 ||
     (13 < cu32 && cu32 < 27) ||
     (27 < cu32 && cu32 < 32) ||
     127 == cu32 { format!("\\x{:02X}", cu32) }
  else if 32  <= cu32 && cu32 < 127 { c.to_string() }
  else if 127 <= cu32 { c.to_string() }
  else if cu32 ==   7 { "\\a".to_string() }
  else if cu32 ==   8 { "\\b".to_string() }
  else if cu32 ==   9 { "\\t".to_string() }
  else if cu32 ==  10 { "\\n".to_string() }
  else if cu32 ==  11 { "\\v".to_string() }
  else if cu32 ==  12 { "\\f".to_string() }
  else if cu32 ==  13 { "\\r".to_string() }
  else if cu32 ==  27 { "\\e".to_string() }
  else { "irrelevant".to_string() }
}
