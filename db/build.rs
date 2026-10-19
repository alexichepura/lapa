//! Track the migrations directory so adding/removing a migration rebuilds the
//! crate and `refinery::embed_migrations!` picks it up.
//!
//! Without this, cargo keeps a cached `db` build (the macro expands to
//! `include_str!` for the files present at compile time), so a newly added
//! migration is silently missing from `cargo cli migrate` until something else
//! forces a rebuild.

fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
