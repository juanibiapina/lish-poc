extern crate regex;

macro_rules! regex {
    ($e:expr) => (regex::Regex::new($e).unwrap())
}

pub fn escape_str(s: &str) -> String {
    let mut escaped = String::new();
    escaped.push('"');
    for c in s.chars() {
        let _ = match c {
          '"' => escaped.push_str("\\\""),
          '\\' => escaped.push_str("\\\\"),
          '\x08' => escaped.push_str("\\b"),
          '\x0c' => escaped.push_str("\\f"),
          '\n' => escaped.push_str("\\n"),
          '\r' => escaped.push_str("\\r"),
          '\t' => escaped.push_str("\\t"),
          _ => escaped.push(c),
        };
    };

    escaped.push('"');

    escaped
}

pub fn unescape_str(s: &str) -> String {
    let re1 = regex!(r#"\\""#);
    let re2 = regex!(r#"\n"#);
    re2.replace_all(&re1.replace_all(&s, "\""), "\n")
}
