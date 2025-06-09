pub fn capitalize(s: &str) -> String {
    let mut s_chars = s.chars();
    let mut res = String::with_capacity(s.len());

    if let Some(c) = s_chars.next() {
        res.extend(c.to_uppercase());
    }

    res.push_str(s_chars.as_str());
    res
}
