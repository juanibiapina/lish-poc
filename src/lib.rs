use std::io::{self,BufRead};

fn read(s: String) -> String {
    s
}

fn eval(s: String) -> String {
    s
}

fn print(s: String) -> String {
    s
}

fn rep(s: String) -> String {
    print(eval(read(s)))
}

pub fn repl() {
    loop {
        let stdin = io::stdin();
        let mut input = String::new();

        // TODO handle end of input
        stdin.lock().read_line(&mut input).unwrap();

        let result = rep(input);

        print!("{}", result);
    }
}
