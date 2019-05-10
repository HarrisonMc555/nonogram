pub mod nonogram;
pub use crate::nonogram::{Nonogram, Tile};

// cfg_if! {
//     if #[cfg(feature = "interactive")] {
//         mod interactive;
//         fn main() {
//             interactive::main();
//         }
//     } else {
//         pub mod formatter;
//         fn main() {
//             formatter::main();
//         }
//     }
// }
#[cfg(feature = "parser")]
mod parser;
#[cfg(feature = "parser")]
#[macro_use]
extern crate nom;

#[cfg(feature = "interactive")]
mod interactive;
#[cfg(feature = "interactive")]
fn main() {
    // interactive::main();
    println!("Interactive...");

    #[cfg(feature = "parser")]
    parser::main();
}

#[cfg(not(feature = "interactive"))]
pub mod formatter;
#[cfg(not(feature = "interactive"))]
fn main() {
    formatter::main();
}
