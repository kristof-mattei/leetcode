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

#[expect(clippy::todo, reason = "Seed code")]
fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    println!("{}", foo());
    println!("{}", bar());
    println!("{}", quz());

    println!(
        "BUILT FOR {} {}",
        std::env::var("TARGETARCH").unwrap(),
        std::env::var("TARGETVARIANT").unwrap()
    );

    todo!("TODO");
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
