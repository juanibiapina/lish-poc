extern crate lish;

#[cfg(not(test))]
use lish::repl::Repl;

#[cfg(not(test))]
fn main() {
    let repl = Repl::new();

    repl.run();
}
