use std::process::exit;

use clap::{ArgGroup, Parser};

use element::{ChemicalElement, CreationError};
use subshell::AsSubshells;

mod element;
mod subshell;
mod vector;

#[derive(Parser)]
#[clap(group(
    ArgGroup::new("search-criteria")
        .required(true)
        .args(&["atomic_number", "symbol", "name"])
))]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Search a element by its atomic number
    #[arg(short, long)]
    atomic_number: Option<u8>,

    /// Search a element by its symbol
    #[arg(short, long)]
    symbol: Option<String>,

    /// Search a element by its name
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let element = match cli {
        Cli {
            atomic_number: Some(atomic_number),
            ..
        } => ChemicalElement::from_atomic_number(atomic_number),

        Cli {
            symbol: Some(symbol),
            ..
        } => ChemicalElement::from_symbol(&symbol),
        Cli {
            name: Some(name), ..
        } => ChemicalElement::from_name(&name),
        _ => {
            unreachable!("Clap should handle this arm.")
        }
    };

    let element = element.unwrap_or_else(|err| {
        let message = match err {
            CreationError::AtomicNumberNotFound(num) => {
                format!("There is no chemical element with the atomic number {num}.")
            }
            CreationError::NameNotFound(name) => {
                format!("There is no chemical element with the name {name}.")
            }
            CreationError::SymbolNotFound(symbol) => {
                format!("There is no chemical element with the symbol {symbol}.")
            }
        };

        eprintln!("{message}");
        exit(1);
    });

    println!(
        "Symbol: {}, Name: {}, AtomicNumber: {}",
        element.symbol(),
        element.name(),
        element.atomic_number()
    );

    println!("\nSubshells:\n{}", element.as_subshells());
}
