use readline;

use command_line::CommandLine;

fn process(input: String) {
    let command_line = CommandLine::parse(input);

    command_line.run();
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

        if input.len() == 0 {
            continue
        }

        process(input);
    }
}
