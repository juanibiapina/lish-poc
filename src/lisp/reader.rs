extern crate regex;

macro_rules! regex {
    ($e:expr) => (regex::Regex::new($e).unwrap())
}

use lisp::error::Error;
use lisp::types;
use lisp::printer;

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn new(str: &str) -> Reader {
        Reader{
            tokens: tokenize(str),
            position: 0
        }
    }

    pub fn read_form(&mut self) -> types::LispResult {
        let option_token = self.peek();
        let string_token = option_token.unwrap();
        let token = &string_token[..];

        match token {
            "'" => {
                let _ = self.next();
                match self.read_form() {
                    Ok(f) => Ok(types::list(vec![types::symbol("quote"), f])),
                    Err(e) => Err(e),
                }
            },
            "`" => {
                let _ = self.next();
                match self.read_form() {
                    Ok(f) => Ok(types::list(vec![types::symbol("quasiquote"), f])),
                    Err(e) => Err(e),
                }
            },
            "~" => {
                let _ = self.next();
                match self.read_form() {
                    Ok(f) => Ok(types::list(vec![types::symbol("unquote"), f])),
                    Err(e) => Err(e),
                }
            },
            "~@" => {
                let _ = self.next();
                match self.read_form() {
                    Ok(f) => Ok(types::list(vec![types::symbol("splice-unquote"), f])),
                    Err(e) => Err(e),
                }
            },

            ")" => Err(Error::Parser("unexpected ')'".to_string())),
            "(" => self.read_list(),

            "]" => Err(Error::Parser("unexpected ']'".to_string())),
            "[" => self.read_vector(),

            _   => self.read_atom()
        }
    }

    fn read_list(&mut self) -> types::LispResult {
        let seq = try!(self.read_seq("(", ")"));

        Ok(types::list(seq))
    }

    fn read_vector(&mut self) -> types::LispResult {
        let seq = try!(self.read_seq("[", "]"));

        Ok(types::vector(seq))
    }

    fn read_atom(&mut self) -> types::LispResult {
        let option_token = self.next();
        let string_token = option_token.unwrap();
        let token = &string_token[..];

        if regex!(r"^-?[0-9]+$").is_match(token) {
            let num : Option<isize> = token.parse().ok();
            Ok(types::_int(num.unwrap()))
        } else if regex!(r#"^".*"$"#).is_match(token) {
            let new_str = &token[1..token.len()-1];
            Ok(types::string(printer::unescape_str(new_str)))
        } else if regex!(r#"^:"#).is_match(token) {
            Ok(types::string(format!("\u{29e}{}", &token[1..])))
        } else if token == "nil" {
            Ok(types::_nil())
        } else if token == "true" {
            Ok(types::_true())
        } else if token == "false" {
            Ok(types::_false())
        } else {
            Ok(types::symbol(token))
        }
    }

    fn read_seq(&mut self, start: &str, end: &str) -> Result<Vec<types::LispValue>, Error> {
        let option_token = self.next();
        let string_token = option_token.unwrap();
        let token = &string_token[..];
        if token != start {
            return Err(Error::Parser(format!("parser: expected '{}', got {}", start, token)))
        }

        let mut ast_vec: Vec<types::LispValue> = vec![];
        loop {
            let option_token = self.peek();
            if option_token.is_none() {
                return Err(Error::Parser(format!("expected '{}', got EOF", end)));
            }
            let string_token = option_token.unwrap();
            let token = &string_token[..];
            if token == end { break; }

            match self.read_form() {
                Ok(mv) => ast_vec.push(mv),
                Err(e) => return Err(e),
            }
        }

        self.next();

        Ok(ast_vec)
    }

    fn next(&mut self) -> Option<String> {
        if self.position < self.tokens.len() {
            self.position += 1;
            Some(self.tokens[self.position-1].to_string())
        } else {
            None
        }
    }
    fn peek(&self) -> Option<String> {
        if self.position < self.tokens.len() {
            Some(self.tokens[self.position].to_string())
        } else {
            None
        }
    }
}

pub fn read(str: &str) -> types::LispResult {
    let mut reader = Reader::new(str);

    reader.read_form()
}

fn tokenize(str: &str) -> Vec<String> {
    let mut results = vec![];
    let re = regex!(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"###);
    for cap in re.captures_iter(&str) {
        let group = cap.at(1).unwrap_or("");
        if group == "" { break; }
        if group.starts_with(";") { continue; }
        results.push(group.to_owned());
    }
    results
}
