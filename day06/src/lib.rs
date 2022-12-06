use parser;

fn unique(s: &str) -> bool {
    match s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .skip(i + 1)
            .find(|other| c == *other)
    }) {
        Some(_) => false,
        None => true,
    }
}

pub fn find_marker(c: &parser::Content) -> u32 {
    for i in 0..c.content.len() {
        if unique(&c.content[i..i+4]) {
            return i as u32 + 4;
        }
    }
    std::u32::MAX
}

pub fn find_msg_marker(c: &parser::Content) -> u32 {
    for i in 0..c.content.len() {
        if unique(&c.content[i..i+14]) {
            return i as u32 + 14;
        }
    }
    std::u32::MAX
}
