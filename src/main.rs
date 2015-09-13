extern crate lish;

#[cfg(not(test))]
use lish::repl;

#[cfg(not(test))]
fn main() {
    repl::run();
}
