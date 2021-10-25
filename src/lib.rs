//! wow this blew up
//!
//! Handles panics by linking the original poster's SoundCloud.
//! Backtrace is not displayed, because it would divert attention from the link.
//!
//! This crate is not affiliated or endorsed by SoundCloud.
//! It's just a meme, you know.
//!
//! The SoundCloud is assumed to be the using crate's homepage from `Cargo.toml`.

/// Initialises the panic handler
///
/// The SoundCloud is assumed to be the using crate's homepage from `Cargo.toml`.
///
/// ```
/// panic_soundcloud::setup!();
/// ```
#[macro_export]
macro_rules! setup {
    () => {
        std::panic::set_hook(Box::new(move |info: &std::panic::PanicInfo| {
            println!(
                "wow this blew up here's my soundcloud: {}",
                env!("CARGO_PKG_HOMEPAGE")
            );
        }));
    };
}
