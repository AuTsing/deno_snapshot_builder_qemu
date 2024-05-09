use std::env;

fn main() {
    env::set_var("TARGET", "aarch64-linux-android");
    env::set_var("HOST", "aarch64-linux-android");
    env::set_var("PROFILE", "release");
    env::set_var("OUT_DIR", "./out");
    env::set_var("CARGO_MANIFEST_DIR", "./src/deno_builder");
    deno_builder::main();
}
