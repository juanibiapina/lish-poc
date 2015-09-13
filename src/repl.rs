use readline;

fn read(s: String) -> String {
    s
}

#[test]
fn read_returns_the_input_string() {
    assert_eq!(read("value".to_string()), "value".to_string())
}

fn eval(s: String) -> String {
    s
}

#[test]
fn eval_returns_the_input_string() {
    assert_eq!(eval("value".to_string()), "value".to_string())
}

fn print(s: String) -> String {
    s
}

#[test]
fn print_returns_the_input_string() {
    assert_eq!(print("value".to_string()), "value".to_string())
}

fn rep(s: String) -> String {
    print(eval(read(s)))
}

#[test]
fn rep_returns_the_input_string() {
    assert_eq!(rep("value".to_string()), "value".to_string())
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
