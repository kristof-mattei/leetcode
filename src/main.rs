#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![deny(warnings)]
// exceptions
#![deny(let_underscore_drop)]
#![deny(non_ascii_idents)]
#![allow(clippy::uninlined_format_args)]

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

// async fn something_to_await(_: PathBuf) {
//     println!("{}", foo());
//     println!("{}", bar());
//     todo!("TODO");
// }

// async fn run_app() {
//     env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();

//     let Opt { some_value } = Opt::from_args();

//     let mut tasks = FuturesUnordered::new();

//     tasks.push(Box::pin(something_to_await(some_value)));

//     loop {
//         match tasks.next().await {
//             None => {
//                 info!("Done!");
//                 return;
//             }
//             _ => {
//                 info!("Waiting...")
//             }
//         }
//     }
// }

// #[tokio::main]
// async fn main() {
//     run_app().await;
// }

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    println!("{}", foo());
    println!("{}", bar());
    println!("{}", quz());
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
