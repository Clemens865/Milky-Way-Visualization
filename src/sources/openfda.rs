//! OpenFDA — drug adverse events, recalls, labels.
//! No auth required. REST API, JSON format.
//! https://open.fda.gov/apis/

use crate::client::Explorer;
use anyhow::Result;
use serde::Deserialize;

const BASE: &str = "https://api.fda.gov";

#[derive(Debug, Deserialize)]
pub struct FdaResponse {
    pub meta: Option<Meta>,
    pub results: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub disclaimer: Option<String>,
    pub results: Option<MetaResults>,
}

#[derive(Debug, Deserialize)]
pub struct MetaResults {
    pub total: Option<u64>,
    pub skip: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CountResult {
    pub term: String,
    pub count: u64,
}

#[derive(Debug, Deserialize)]
pub struct CountResponse {
    pub meta: Option<Meta>,
    pub results: Option<Vec<CountResult>>,
}

/// Search adverse events for a drug.
pub async fn adverse_events(
    explorer: &Explorer,
    drug_name: &str,
    limit: u32,
) -> Result<FdaResponse> {
    let url = format!(
        "{BASE}/drug/event.json?search=patient.drug.openfda.brand_name:\"{drug_name}\"&limit={limit}"
    );
    explorer.fetch_json(&url).await
}

/// Count adverse event reactions for a drug.
pub async fn reaction_counts(
    explorer: &Explorer,
    drug_name: &str,
    limit: u32,
) -> Result<CountResponse> {
    let url = format!(
        "{BASE}/drug/event.json?search=patient.drug.openfda.brand_name:\"{drug_name}\"&count=patient.reaction.reactionmeddrapt.exact&limit={limit}"
    );
    explorer.fetch_json(&url).await
}

/// Count events by year for a drug.
pub async fn events_by_year(
    explorer: &Explorer,
    drug_name: &str,
) -> Result<CountResponse> {
    let url = format!(
        "{BASE}/drug/event.json?search=patient.drug.openfda.brand_name:\"{drug_name}\"&count=receivedate"
    );
    explorer.fetch_json(&url).await
}

/// Top reported drugs across all adverse events.
pub async fn top_drugs(explorer: &Explorer, limit: u32) -> Result<CountResponse> {
    let url = format!(
        "{BASE}/drug/event.json?count=patient.drug.openfda.brand_name.exact&limit={limit}"
    );
    explorer.fetch_json(&url).await
}

pub fn print_reactions(drug: &str, result: &CountResponse) {
    use colored::Colorize;

    println!("\n{} — Top Adverse Reactions\n", drug.bold());

    if let Some(ref results) = result.results {
        let total: u64 = results.iter().map(|r| r.count).sum();

        println!(
            "  {:>6}  {:>5}  {}",
            "Count".bold(),
            "%".bold(),
            "Reaction".bold(),
        );
        println!("  {}", "─".repeat(60));

        for r in results.iter().take(25) {
            let pct = (r.count as f64 / total as f64) * 100.0;
            println!("  {:>6}  {:>4.1}%  {}", r.count, pct, r.term);
        }

        println!("\n  Total reports across top reactions: {total}");
    }
}
