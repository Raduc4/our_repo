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

    let mut scenarios = vec![];

    let mut g_max_debt = f32::MIN;
    let mut g_min_debt = f32::MAX;

    let mut g_max_qty_incidents = f32::MIN;
    let mut g_min_qty_incidents = f32::MAX;

    let mut g_min_abs = f32::MAX;
    let mut g_max_abs = f32::MIN;

    let mut g_max_qty_calls = f32::MIN;
    let mut g_min_qty_calls: f32 = f32::MAX;

    let mut g_max_qty_port_rez = f32::MIN;
    let mut g_min_qty_port_rez: f32 = f32::MAX;

    for (_, entries) in &records {
        let last = entries.last().unwrap();
        let max_debt = last.debth_month.unwrap_or(0);
        let contract_status = last.contract_status.unwrap();
        let qty_calls = last.qnt_calls.unwrap_or(0);
        let qty_incidents = last.qnt_incident.unwrap_or(0);
        let qty_port_rez = last.qnt_port_res.unwrap_or(0);

        let payment_sum = {
            if last.payment_sum.contains(",") {
                last.payment_sum
                    .replace(",", ".")
                    .parse::<f32>()
                    .unwrap()
                    .floor() as i32
            } else if last.payment_sum != "" {
                last.payment_sum.parse::<i32>().unwrap()
            } else {
                0
            }
        };

        let subscription_price = {
            if last.subscription_price.contains(",") {
                last.subscription_price
                    .replace(",", ".")
                    .parse::<f32>()
                    .unwrap()
                    .floor() as i32
            } else {
                last.subscription_price.parse::<i32>().unwrap()
            }
        };

        let abs = payment_sum - subscription_price;

        g_max_debt = g_max_debt.max(max_debt as f32);
        g_min_debt = g_min_debt.min(max_debt as f32);

        g_max_qty_incidents = g_max_qty_incidents.max(qty_incidents as f32);
        g_min_qty_incidents = g_min_qty_incidents.min(qty_incidents as f32);

        g_min_abs = g_min_abs.min(abs as f32);
        g_max_abs = g_max_abs.max(abs as f32);

        g_max_qty_calls = g_max_qty_calls.max(qty_calls as f32);
        g_min_qty_calls = g_min_qty_calls.min(qty_calls as f32);

        g_min_qty_port_rez = g_min_qty_port_rez.min(qty_port_rez as f32);
        g_max_qty_port_rez = g_max_qty_port_rez.max(qty_port_rez as f32);

        let train_set = vec![
            max_debt as f32,
            contract_status as f32,
            qty_incidents as f32,
            abs as f32,
            qty_calls as f32,
            qty_port_rez as f32,
        ];

        scenarios.push((train_set, vec![last.is_churn.unwrap() as f32]));
    }

    let mut nn = NeuralNetwork::new(6, 6, 1, 0.03);
    let mut rng = thread_rng();

    for _ in 0..5000000 {
        let random = rng.gen_range(0..scenarios.len()) as usize;
        let (train_data, target_data) = scenarios.get(random).unwrap();

        let record = normalize_dataset(train_data);

        // we will pick a random scenario from the dataset and feed it to the network with the expected target
        nn.train(&record, target_data)
    }

    nn.save(Path::new("churn_model.bin")).unwrap();

    for _ in 0..100 {
        let random = rng.gen_range(0..scenarios.len()) as usize;
        let (train_data, expected) = scenarios.get(random).unwrap();

        let record = normalize_dataset(train_data);

        let result = nn.predict(&record);
        println!("{:?} = {:?}, {:?}", train_data, result, expected)
    }

    Ok(())
}

fn normalize_dataset(train_data: &Vec<f32>) -> Vec<f32> {
    let debt = train_data[0];
    let contract = train_data[1];
    let incidents = train_data[2];
    let abs = train_data[3];
    let qty_calls = train_data[4];
    let qty_port_rez = train_data[5];

    let record = vec![
        normalize(debt, 0.0, 6.0),
        normalize(contract, 0.0, 3.0),
        normalize(incidents, 0.0, 7.0),
        normalize(abs, -325.0, 2550.0),
        normalize(qty_calls, 0.0, 90.0),
        normalize(qty_port_rez, 0.0, 7.0),
    ];
    record
}

fn normalize(x: f32, min_x: f32, max_x: f32) -> f32 {
    (x - min_x) / (max_x - min_x)
}

fn main() {
    if let Err(err) = train() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
