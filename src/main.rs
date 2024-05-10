use std::env;
use std::fs;

fn main() {
    let working_dir = env::current_dir().unwrap();
    let out_dir = working_dir.join("out");
    let cargo_manifest_dir = working_dir.join("src/deno_builder/deno/cli");

    env::set_current_dir(&cargo_manifest_dir).unwrap();
    fs::create_dir(&out_dir).unwrap();
    env::set_var("TARGET", "aarch64-linux-android");
    env::set_var("HOST", "aarch64-linux-android");
    env::set_var("PROFILE", "release");
    env::set_var("OUT_DIR", &out_dir);
    env::set_var("CARGO_MANIFEST_DIR", &cargo_manifest_dir);
    deno_builder::main();
}
