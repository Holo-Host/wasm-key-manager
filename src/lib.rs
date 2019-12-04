pub mod hcs0;
pub mod key_manager;
pub mod seed_from;
pub mod util;

pub use hcs0::{from_hcs0, to_hcs0};
pub use key_manager::KeyManager;
pub use seed_from::derive_seed_from;

// wee_alloc shaves off ~4KB off WASM file size.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
