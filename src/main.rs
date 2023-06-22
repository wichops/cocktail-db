use std::error::Error;

use clap::Parser;
use serde::Deserialize;

use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

#[derive(Deserialize, Debug)]
struct Resp {
    drinks: Vec<Drink>,
}

#[derive(Deserialize, Debug)]
struct Drink {
    #[serde(rename = "strDrink")]
    name: String,
    #[serde(rename = "strInstructions")]
    instructions: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let url = format!(
        "https://www.thecocktaildb.com/api/json/v1/1/search.php?s={}",
        args.name
    );

    let response = reqwest::blocking::get(url)?;
    let body: Resp = response.json()?;

    for drink in body.drinks {
        let instructions_list = drink.instructions.split('.').filter(|x| !x.is_empty());

        println!();
        println!("{}", drink.name.bold().bright_red().reversed());
        println!();
        println!("{}", "Instructions".bold().green());
        for (i, ins) in instructions_list.enumerate() {
            println!("{}. {}", i + 1, ins.trim().italic());
        }
        println!();
    }
    Ok(())
}
