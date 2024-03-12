pub const FALSE: bool = false;

cfg_if::cfg_if! {if #[cfg(feature = "leptos")] {
    mod err;
    pub use err::*;
}}
