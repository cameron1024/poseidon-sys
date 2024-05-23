fn main() {
    let lib_path = find_lib();

    println!("cargo:rustc-link-search={lib_path}");
    println!("cargo:rustc-link-lib=poseidon");
}

fn find_lib() -> String {
    const PATH: &str = "LIBPOSEIDON_PATH";
    const DEFAULT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/poseidon-impl/target/release");

    println!("cargo:rerun-if-env-changed={PATH}");
    std::env::var(PATH).unwrap_or_else(|_| DEFAULT_PATH.to_string())
}
