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
// mod nonogram_file;

#[cfg(feature = "interactive")]
mod interactive;
#[cfg(feature = "interactive")]
fn main() {
    let run = true;
    if run {
        interactive::main();
        println!("Interactive...");
    }
}

#[cfg(not(feature = "interactive"))]
pub mod formatter;
#[cfg(not(feature = "interactive"))]
fn main() {
    formatter::main();
}
