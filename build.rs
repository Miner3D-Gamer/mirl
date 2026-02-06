// fn main() {
//     let version = rustc_version::version_meta().unwrap();

//     if version.channel == rustc_version::Channel::Nightly {
//         println!("cargo:rustc-cfg=nightly");
//     }
// }
// fn main() {
//     let is_miri = std::env::var("MIRI").is_ok()
//         || std::env::var("CARGO_CFG_MIRI").is_ok();

//     if is_miri {
//         println!("cargo:rustc-cfg=miri");
//         println!("cargo:warning=Building under Miri");
//     }
// }
fn main() {}
