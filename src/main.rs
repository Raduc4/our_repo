// use vinyana::NeuralNetwork;

use serde::Deserialize;
use std::{error::Error, io, process};

// "CONCURENTI";
//"INCIDENTE";
//"STATUT_CONTRACT";"SOLICITARI_REZILIERE";"NET_PARAMS";"PRET_ABON";"QUALITY_PERCEPTION";"COST_PERCEPTION";"TECHNOLOGY";"IPTV_STB_QUANTITY";"INET_PACK";"IPTV_PACK";"QNT_APELARI";"QNT_PORT_REZ";"AVG_PERCEPTION";"QNT_SUSP";"QNT_INCEDENT";"LUNA_SUSPENDARI";"LUNA_INCIDENT";"LUNA_APEL";"LUNI_DATOR";"PERIOADA_ACHITARII";"SUMA_ACHITARII";"MULTIPLAY";"CNT_SERVICII";"IS_CHURN";"CHURN_DATE"

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename(deserialize = "ACCOUNTID"))]
    account_id: String,
    #[serde(rename(deserialize = "CODE"))]
    code: String,
    #[serde(rename(deserialize = "CODE_MA"))]
    code_ma: String,
    #[serde(rename(deserialize = "PERIOADA"))]
    period: String,
    #[serde(rename(deserialize = "CONTRACT_LENGTH"))]
    contract_length: String,
    #[serde(rename(deserialize = "CONTRACT_EXPIRATION_DATE"))]
    contract_expiration_date: String,
    #[serde(rename(deserialize = "CREANTE_REST"))]
    remaining_receivables: String,
    #[serde(rename(deserialize = "CONCURENTI"))]
    competitors: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .escape(Some(b'\\'))
        .from_reader(io::stdin());

    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Record = result?;

        println!("{:?}", record);
        break;
        records.push(record);
    }

    println!("Len {}", records.len());

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
