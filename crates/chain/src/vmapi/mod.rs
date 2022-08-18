use cfg_if::cfg_if;


cfg_if! {
    if #[cfg(all(not(feature = "std"), target_arch = "wasm32"))] {
        mod on_chain;
        pub use on_chain::*;
    } else if #[cfg(feature = "std")] {
        mod off_chain;
        pub use off_chain::*;
    } else {
        compile_error! {
            "chain only support compilation as `std` or `no_std` + `wasm32-unknown`"
        }
    }
}
