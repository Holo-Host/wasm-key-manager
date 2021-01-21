pub mod key_manager;
pub mod seed_from;
pub mod util;

pub use key_manager::KeyManager;
pub use seed_from::derive_seed_from;

// wee_alloc shaves off ~4KB off WASM file size.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
