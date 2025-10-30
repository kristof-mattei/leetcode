use std::env;
use std::ffi::OsStr;

fn main() {
    export_var(
        "COMPILE_TIME_HOST",
        &env::var("HOST").expect("HOST is set for build scripts"),
    );
    export_var(
        "COMPILE_TIME_TARGET",
        &env::var("TARGET").expect("TARGET is set for build scripts"),
    );
    export_var(
        "COMPILE_TIME_TARGET_CPU",
        &extract_target_cpu_from_rustflags(
            &env::var_os("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default(),
        )
        .unwrap_or_default(),
    );

    println!("cargo:rerun-if-changed-env=HOST");
    println!("cargo:rerun-if-changed-env=TARGET");
    println!("cargo:rerun-if-changed-env=CARGO_ENCODED_RUSTFLAGS");
}

fn extract_target_cpu_from_rustflags(rustflags: &OsStr) -> Option<String> {
    const ASCII_SEPARATOR: char = '\x1f';

    let rustflags = rustflags.to_string_lossy();

    let mut cpu = None;

    for flag in rustflags.split(ASCII_SEPARATOR) {
        let stripped_c = flag.strip_prefix("-C").unwrap_or(flag);

        if let Some(target_cpu) = stripped_c.strip_prefix("target-cpu=") {
            cpu = Some(target_cpu);
        }
    }

    cpu.map(str::to_owned)
}

fn export_var(name: &str, value: &str) {
    println!("cargo:rustc-env={}={}", name, value);
}
