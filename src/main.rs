// use std::path::PathBuf;

// use env_logger::Env;
// use futures::{stream::FuturesUnordered, StreamExt};
// use log::info;

// use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// #[structopt(
//     // name, // from Cargo.toml,
//     about, // needed otherwise it doesn't show description from Cargo.toml,
//     author // needed otherwise it doesn't show author from Cargo.toml
// )]
// struct Opt {
//     #[structopt(
//         // verbatim_doc_comment,
//         help = "Some help",
//         parse(from_os_str)
//     )]
//     some_value: PathBuf,
// }

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
