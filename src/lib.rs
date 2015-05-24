pub fn escape_string(str:&str) -> String {
  str.chars()
     .map(escape_char)
     .fold(String::new(), |a:String, b:String| {
       a + &b
     })
}

pub fn escape_char(c:char) -> String {
  match c {
    '\u{0}' => { "\\x00".to_string() },
    '\u{1}' => { "\\x01".to_string() },
    '\u{2}' => { "\\x02".to_string() },
    '\u{3}' => { "\\x03".to_string() },
    '\u{4}' => { "\\x04".to_string() },
    '\u{5}' => { "\\x05".to_string() },
    '\u{6}' => { "\\x06".to_string() },
    '\u{7}' => { "\\a".to_string()   },
    '\u{8}' => { "\\b".to_string()   },
    '\u{9}' => { "\\t".to_string()   },
    '\u{a}' => { "\\n".to_string()   },
    '\u{b}' => { "\\v".to_string()   },
    '\u{c}' => { "\\f".to_string()   },
    '\u{d}' => { "\\r".to_string()   },
    _       => { format!("NOTHING FOR {:?}", c) },
  }
}
