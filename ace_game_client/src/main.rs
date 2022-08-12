#[macro_use]
extern crate cfg_if;

mod app;
mod resources;
mod systems;

fn main() {
    app::run();
}

// cfg_if! {
//     if #[cfg(not(target_arch = "wasm32"))] {
//     }
// }
