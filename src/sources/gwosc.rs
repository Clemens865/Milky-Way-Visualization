//! LIGO/Virgo/KAGRA Gravitational Wave Open Science Center.
//! No auth required. REST API.
//! https://gwosc.org/apidocs/

use crate::client::Explorer;
use anyhow::Result;
use serde::Deserialize;

const BASE: &str = "https://gwosc.org";

#[derive(Debug, Deserialize)]
pub struct EventList {
    pub events: std::collections::HashMap<String, Event>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(rename = "GPS")]
    pub gps: Option<f64>,
    pub mass_1_source: Option<f64>,
    pub mass_2_source: Option<f64>,
    pub final_mass_source: Option<f64>,
    pub luminosity_distance: Option<f64>, // Mpc
    pub chi_eff: Option<f64>,
    pub network_matched_filter_snr: Option<f64>,
    #[serde(rename = "catalog.shortName")]
    pub catalog: Option<String>,
    #[serde(rename = "jsonurl")]
    pub json_url: Option<String>,
}

/// List available catalogs.
pub async fn catalogs(explorer: &Explorer) -> Result<std::collections::HashMap<String, serde_json::Value>> {
    let url = format!("{BASE}/eventapi/json/");
    explorer.fetch_json(&url).await
}

/// Fetch events from a specific catalog (e.g., "GWTC-3-confident").
pub async fn catalog_events(explorer: &Explorer, catalog: &str) -> Result<EventList> {
    let url = format!("{BASE}/eventapi/json/{catalog}/");
    explorer.fetch_json(&url).await
}

/// Fetch all confident events across all GWTC catalogs.
pub async fn all_confident_events(explorer: &Explorer) -> Result<EventList> {
    let catalogs_to_fetch = [
        "GWTC-1-confident",
        "GWTC-2.1-confident",
        "GWTC-3-confident",
    ];

    let mut all_events = std::collections::HashMap::new();
    for cat in &catalogs_to_fetch {
        match catalog_events(explorer, cat).await {
            Ok(list) => all_events.extend(list.events),
            Err(e) => tracing::warn!("Failed to fetch {cat}: {e}"),
        }
    }

    Ok(EventList { events: all_events })
}

pub fn print_summary(events: &EventList) {
    use colored::Colorize;

    let mut sorted: Vec<_> = events.events.values().collect();
    sorted.sort_by(|a, b| {
        a.gps
            .partial_cmp(&b.gps)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    println!(
        "\n{} — {} events\n",
        "Gravitational Wave Events".bold(),
        sorted.len()
    );
    println!(
        "  {:>20}  {:>8}  {:>8}  {:>8}  {:>8}  {:>6}",
        "Name".bold(),
        "M1(M☉)".bold(),
        "M2(M☉)".bold(),
        "Mf(M☉)".bold(),
        "Dist".bold(),
        "SNR".bold(),
    );
    println!("  {}", "─".repeat(75));

    for e in &sorted {
        let name = e.common_name.as_deref().unwrap_or("?");

        println!(
            "  {:>20}  {:>8.1}  {:>8.1}  {:>8.1}  {:>7.0}  {:>6.1}",
            name,
            e.mass_1_source.unwrap_or(0.0),
            e.mass_2_source.unwrap_or(0.0),
            e.final_mass_source.unwrap_or(0.0),
            e.luminosity_distance.unwrap_or(0.0),
            e.network_matched_filter_snr.unwrap_or(0.0),
        );
    }
}
