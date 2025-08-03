mod bld_logger {
    #[macro_export]
    macro_rules! info {
        ($content:expr) => {
            println!("cargo::warning=\r[{}] {}", "\x1b[34m^\x1b[0m", $content);
        };
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=exports.def");

    info!("Using exports.def...");
    println!("cargo:rustc-link-arg={}/exports.def", std::env::current_dir().unwrap().display());
}
