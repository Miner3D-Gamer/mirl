#[cfg(feature = "discord")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
/// Send stuff to discord webhooks, created because `discord-webhooks` kinda sucks and `discord-webhook2` expects to be called in an async environment
pub mod discord;
