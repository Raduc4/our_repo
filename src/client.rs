use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename(deserialize = "ACCOUNTID"))]
    pub account_id: String,

    #[serde(rename(deserialize = "CODE"))]
    pub code: String,

    #[serde(rename(deserialize = "CODE_MA"))]
    pub code_ma: String,

    #[serde(rename(deserialize = "PERIOADA"))]
    pub epoch: String,

    #[serde(rename(deserialize = "CONTRACT_LENGTH"))]
    pub contract_length: Option<i32>,

    #[serde(rename(deserialize = "CONTRACT_EXPIRATION_DATE"))]
    pub contract_expiration_date: String,

    #[serde(rename(deserialize = "CREANTE_REST"))]
    pub remaining_receivables: Option<i32>,

    #[serde(rename(deserialize = "CONCURENTI"))]
    pub competitors: Option<i32>,

    #[serde(rename(deserialize = "INCIDENTE"))]
    pub incidents: Option<i32>,

    #[serde(rename(deserialize = "STATUT_CONTRACT"))]
    pub contract_status: String,

    #[serde(rename(deserialize = "SOLICITARI_REZILIERE"))]
    pub termination_request: String,

    #[serde(rename(deserialize = "NET_PARAMS"))]
    pub net_params: String,

    #[serde(rename(deserialize = "PRET_ABON"))]
    pub subscription_price: String,

    #[serde(rename(deserialize = "QUALITY_PERCEPTION"))]
    pub quality_perception: String,

    #[serde(rename(deserialize = "COST_PERCEPTION"))]
    pub cost_perception: String,

    #[serde(rename(deserialize = "TECHNOLOGY"))]
    pub technology: String,

    // "IPTV_STB_QUANTITY";
    // "INET_PACK";
    // "IPTV_PACK";
    // "QNT_APELARI";
    // "QNT_PORT_REZ";
    // "AVG_PERCEPTION";
    // "QNT_SUSP";
    // "QNT_INCEDENT";
    // "LUNA_SUSPENDARI";
    // "LUNA_INCIDENT";
    // "LUNA_APEL";
    // "LUNI_DATOR";
    // "PERIOADA_ACHITARII";
    // "SUMA_ACHITARII";
    // "MULTIPLAY";
    // "CNT_SERVICII";
    #[serde(rename(deserialize = "IS_CHURN"))]
    pub is_churn: Option<u8>,

    #[serde(rename(deserialize = "CHURN_DATE"))]
    pub churn_date: String,
}

impl Record {
    fn to_train_record(&self) -> Result<(Vec<f32>, Vec<f32>), Box<dyn Error>> {
        // vec![self.incidents]

        Ok((vec![], vec![]))
    }
}
