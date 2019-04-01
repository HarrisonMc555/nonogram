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

#[cfg(not(feature = "interactive"))]
pub mod formatter;

#[cfg(feature = "interactive")]
mod interactive;

#[cfg(not(feature = "interactive"))]
fn main() {
    formatter::main();
}

#[cfg(feature = "interactive")]
fn main() {
    interactive::main();
}
