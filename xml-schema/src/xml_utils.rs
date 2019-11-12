/// https://www.w3.org/TR/xml11/#NT-Char
#[inline(always)]
pub fn is_xml_char(c: char) -> bool {
    match c as u32 {
        0x0..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF => true,
        0xFFFE | 0xFFFF => false,
        _ => unreachable!(),
    }
}

/// https://www.w3.org/TR/xml-names11/#NT-NCNameStartChar
#[inline(always)]
pub fn is_name_start_char(c: char) -> bool {
    match c {
        ':' | 'A'..='Z' | '_' | 'a'..='z' | '\u{C0}'..='\u{D6}' | '\u{F8}'..='\u{2FF}' |
        '\u{370}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' | '\u{2070}'..='\u{218F}' |
        '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' | '\u{F900}'..='\u{FDCF}' |
        '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}' => true,
        _ => false,
    }
}

/// https://www.w3.org/TR/xml-names11/#NT-NCNameChar
#[inline(always)]
pub fn is_name_char(c: char) -> bool {
    match c {
        '-' | '.' | '0'..='9' | '\u{B7}' | '\u{0300}'..='\u{036F}' |
        '\u{203F}'..='\u{2040}' => true,
        _ => is_name_start_char(c),
    }
}
