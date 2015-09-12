use readline;

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

pub fn run() {
    loop {
        let input = match readline::readline(":) ") {
            Some(input) => input,
            None => {
                println!("");
                break;
            },
        };

        let result = rep(input);

        println!("{}", result);
    }
}
