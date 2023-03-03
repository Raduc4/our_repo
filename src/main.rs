// use vinyana::NeuralNetwork;

use std::{error::Error, io, process};

mod client;

use client::Record;

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .escape(Some(b'\\'))
        .from_reader(io::stdin());

    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Record = result?;

        // println!("{:?}", record);
        // break;
        records.push(record);
    }

    let with_churn = records
        .iter()
        .filter(|it| it.is_churn.is_some() && it.is_churn.unwrap() == 1);

    println!("Total {} with churn {}", records.len(), with_churn.count());

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
