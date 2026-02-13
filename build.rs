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
// fn main() {}
// use std::env;

fn main() {
    // Check if we're using the nightly compiler
    let is_nightly = version_check::is_feature_flaggable().expect("Unable to infer rust metadata using `version_check` crate");
    
    
    if !is_nightly {
       // eprintln!("\n");
        eprintln!("╔═══════════════════════════════════════════════════════════════╗");
        eprintln!("║                       NIGHTLY  REQUIRED                       ║");
        eprintln!("╠═══════════════════════════════════════════════════════════════╣");
        eprintln!("║ `Mirl` requires Rust nightly to compile.                      ║");
        eprintln!("║                                                               ║");
        eprintln!("║ To install and use nightly:                                   ║");
        eprintln!("║                                                               ║");
        eprintln!("║ 1. Install nightly toolchain:                                 ║");
        eprintln!("║    `rustup install nightly`                                   ║");
        eprintln!("║                                                               ║");
        eprintln!("║ 2. Use nightly for this project (recommended):                ║");
        eprintln!("║    `rustup override set nightly`                              ║");
        eprintln!("║                                                               ║");
        eprintln!("║ Or use nightly for a single build:                            ║");
        eprintln!("║    `cargo +nightly build`                                     ║");
        eprintln!("╚═══════════════════════════════════════════════════════════════╝");
        
        // Uncomment the line below to make this a hard error instead of a warning
        // panic!("Nightly compiler required!");
        
        // Exit with error code
        std::process::exit(1);
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}