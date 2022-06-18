extern crate serde_json;
extern crate serde_state ;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_derive_state;

mod serde_with_state;
mod serde_map_to_seq;

fn main() {
    println!("Hello, world!");
    // serde_with_state::main();
    serde_map_to_seq::main();
}

