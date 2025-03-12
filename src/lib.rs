use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct EarningsEntry {
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Block")]
    pub block: String,
    #[serde(rename = "Share Log %")]
    pub share_log_pct: String,
    #[serde(rename = "Share Count")]
    pub share_count: u64,
    #[serde(rename = "Earnings (BTC)")]
    pub earnings_btc: f64,
    #[serde(rename = "Pool Fees (BTC)")]
    pub pool_fees_btc: f64,
}

/// Retrieves earnings entries directly for a given account name.
pub async fn get_earnings(account_name: &str) -> Result<Vec<EarningsEntry>> {
    let client = Client::new();
    let url = format!("https://ocean.xyz/data/csv/{}/earnings", account_name);

    let resp = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?
        .error_for_status()?;


    let text = resp.text().await?;
    parse_earnings(&text)
}

fn parse_earnings(csv_data: &str) -> Result<Vec<EarningsEntry>> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let entries = reader
        .deserialize()
        .collect::<Result<Vec<EarningsEntry>, csv::Error>>()?;

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_earnings() {
        let sample_csv = "\
Time,Block,Share Log %,Share Count,Earnings (BTC),Pool Fees (BTC)
2025-03-12 13:03,000000000000000000016e74e3547695c7e411e9716af53d389a8e24b75197a8,0.00072201%,6477840384,0.00002261,0.00000026
2025-03-12 10:51,000000000000000000005d6da9647c846dcc2e7bda7ec77d90c3239443b9710d,0.00072126%,6471155712,0.00002257,0.00000025
";

        let parsed = parse_earnings(sample_csv).expect("Parsing failed");
        assert_eq!(parsed.len(), 2);

        assert_eq!(
            parsed[0],
            EarningsEntry {
                time: "2025-03-12 13:03".to_string(),
                block: "000000000000000000016e74e3547695c7e411e9716af53d389a8e24b75197a8".to_string(),
                share_log_pct: "0.00072201%".to_string(),
                share_count: 6477840384,
                earnings_btc: 0.00002261,
                pool_fees_btc: 0.00000026,
            }
        );

        assert_eq!(
            parsed[1],
            EarningsEntry {
                time: "2025-03-12 10:51".to_string(),
                block: "000000000000000000005d6da9647c846dcc2e7bda7ec77d90c3239443b9710d".to_string(),
                share_log_pct: "0.00072126%".to_string(),
                share_count: 6471155712,
                earnings_btc: 0.00002257,
                pool_fees_btc: 0.00000025,
            }
        );
    }
}


