extern crate inspect;
use inspect::{escape_string};

fn escapes_to(expected:&str, to_escape:&str) {
  assert_eq!(escape_string(to_escape),
            expected.to_string());
}

#[test]
fn it_hex_escapes_0_through_6_function() {
  escapes_to("\\x00\\x01\\x02\\x03\\x04\\x05\\x06",
             "\x00\x01\x02\x03\x04\x05\x06");
}

#[test]
fn it_uses_common_escaped_chars_for_7_13() {
  escapes_to("\\a\\b\\t\\n\\v\\f\\r",
             "\x07\x08\x09\x0A\x0B\x0C\x0D");
}

/* #[test] */
/* fn it_hex_escapes_14_26() { */
/*   escapes_to("\\x0E\\x0F\\x10\\x11\\x12\\x13\\x14\\x15\\x16\\x17\\x18\\x19\\x1A", */
/*              "\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A"); */
/* } */

/* #[test] */
/* fn it_uses_escaped_e_for_27() { */
/*   escapes_to("\\e", "\x1B"); */
/* } */

/* #[test] */
/* fn it_hex_escapes_28_31() { */
/*   escapes_to("\\x1C\\x1D\\x1E\\x1F", */
/*              "\x1C\x1D\x1E\x1F"); */
/* } */

/* #[test] */
/* fn it_uses_common_symbols_for_32_47() { */
/*   escapes_to(" !\"#$%&'()*+,-./", */
/*              "\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F"); */
/* } */

/* #[test] */
/* fn it_uses_digits_for_48_57() { */
/*   escapes_to("0123456789", */
/*              "\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39"); */
/* } */

/* #[test] */
/* fn it_uses_common_symbols_for_58_64() { */
/*   escapes_to(":;<=>?@", "\x3A\x3B\x3C\x3D\x3E\x3F\x40"); */
/* } */

/* #[test] */
/* fn it_uses_uppercase_letters_for_65_90() { */
/*   escapes_to("ABCDEFGHIJKLMNOPQRSTUVWXYZ", */
/*              "\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A"); */
/* } */

/* #[test] */
/* fn it_uses_common_symbols_for_91_96() { */
/*   escapes_to("[\\]^_`", "\x5B\x5C\x5D\x5E\x5F\x60"); */
/* } */

/* #[test] */
/* fn it_uses_lowercase_letters_for_97_122() { */
/*   escapes_to("abcdefghijklmnopqrstuvwxyz", */
/*              "\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A"); */
/* } */

/* #[test] */
/* fn it_uses_common_symbols_for_123_126() { */
/*   escapes_to("{|}~", "\x7B\x7C\x7D\x7E"); */
/* } */

/* #[test] */
/* fn it_hex_escapes_127() { */
/*   escapes_to("\\x7F", "\x7F"); */
/* } */

/* #[test] */
/* fn it_does_not_escape_unicode_characters() { */
/*   escapes_to("¡™£", "¡™£"); */
/* } */
