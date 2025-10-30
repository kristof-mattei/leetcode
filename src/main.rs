use color_eyre::config::HookBuilder;
use color_eyre::eyre;

use crate::build_env::get_build_env;
mod build_env;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn foo() -> &'static str {
    "Foo"
}

fn bar() -> &'static str {
    "Bar"
}

fn quz() -> &'static str {
    "Quz"
}

fn i_will_error() -> Result<(), eyre::Report> {
    Err(eyre::Report::msg("I promised you, I'd error!"))
}

fn main() -> Result<(), eyre::Report> {
    HookBuilder::default()
        .capture_span_trace_by_default(true)
        .install()?;

    println!("{}", foo());
    println!("{}", bar());
    println!("{}", quz());

    println!("Build setup: {}", get_build_env());

    i_will_error()
}

#[cfg(test)]
mod tests {
    use super::{bar, foo, quz};

    #[test]
    fn assert_foo() {
        assert_eq!(foo(), "Foo");
    }

    #[test]
    fn assert_bar() {
        assert_eq!(bar(), "Bar");
    }

    #[test]
    fn assert_quz() {
        assert_eq!(quz(), "Quz");
    }

    #[test]
    fn assert_combined() {
        assert_eq!(format!("{}-{}-{}", foo(), bar(), quz()), "Foo-Bar-Quz");
    }
}
