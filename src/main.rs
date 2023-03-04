// use vinyana::NeuralNetwork;

use rand::thread_rng;
use rand::Rng;
use std::{collections::HashMap, error::Error, path::Path, process};
use vinyana::NeuralNetwork;

mod client;

use client::Record;

fn train() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .escape(Some(b'\\'))
        .from_path("./data/etapa1_full.csv")?;

    let mut records = HashMap::new();
    for result in rdr.deserialize() {
        let record: Record = result?;

        let entry = records.entry(record.account_id.clone()).or_insert(vec![]);
        entry.push(record);
    }

    // let train_data:
    let mut scenarios = vec![];
    for (account, entries) in &records {
        let count = entries.len() as i32;
        let first = entries.first().unwrap();

        // let mut sums = (0, 0);

        let (sum_qty_incidents, sum_termination_requests) =
            entries.iter().fold((0, 0), |mut acc, rec| {
                acc.0 += rec.qnt_incident.unwrap_or(0);
                acc.1 += rec.termination_request.unwrap_or(0);

                acc
            });

        let train_set = vec![
            (sum_qty_incidents as f32 / count as f32) as f32,
            (sum_termination_requests as f32 / count as f32) as f32,
        ];

        scenarios.push((train_set, vec![first.is_churn.unwrap() as f32]));
    }

    let mut nn = NeuralNetwork::new(2, 3, 1, 0.05);

    let mut rng = thread_rng();

    for _ in 0..1000000 {
        let random = rng.gen_range(0..scenarios.len()) as usize;
        let (train_data, target_data) = scenarios.get(random).unwrap();

        // we will pick a random scenario from the dataset and feed it to the network with the expected target
        nn.train(train_data, target_data)
    }

    nn.save(Path::new("churn_model.bin")).unwrap();

    Ok(())
}

fn main() {
    // if let Err(err) = train() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }

    let nn = NeuralNetwork::load(Path::new("churn_model.bin")).unwrap();

    let result = nn.predict(&vec![0.3, 0.25]);
    println!("{:?}", result);
}
