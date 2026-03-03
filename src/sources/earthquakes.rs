//! USGS Earthquake API — real-time global seismic data.
//! No auth required. GeoJSON format.
//! https://earthquake.usgs.gov/fdsnws/event/1/

use crate::client::Explorer;
use anyhow::Result;
use serde::Deserialize;

const BASE: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query";

#[derive(Debug, Deserialize)]
pub struct FeatureCollection {
    pub metadata: Metadata,
    pub features: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub generated: u64,
    pub count: u32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    pub properties: Properties,
    pub geometry: Geometry,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub mag: Option<f64>,
    pub place: Option<String>,
    pub time: Option<u64>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub title: Option<String>,
    pub tsunami: Option<u32>,
    pub sig: Option<u32>,
    pub depth: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<f64>, // [lon, lat, depth_km]
}

impl Feature {
    pub fn lon(&self) -> f64 {
        self.geometry.coordinates.first().copied().unwrap_or(0.0)
    }
    pub fn lat(&self) -> f64 {
        self.geometry.coordinates.get(1).copied().unwrap_or(0.0)
    }
    pub fn depth_km(&self) -> f64 {
        self.geometry.coordinates.get(2).copied().unwrap_or(0.0)
    }
}

/// Fetch recent earthquakes.
/// `period`: "hour", "day", "week", "month"
/// `min_magnitude`: "significant", "4.5", "2.5", "1.0", "all"
pub async fn recent(
    explorer: &Explorer,
    period: &str,
    min_magnitude: &str,
) -> Result<FeatureCollection> {
    let url = format!(
        "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/{min_magnitude}_{period}.geojson"
    );
    explorer.fetch_json(&url).await
}

/// Custom query with date range, location bounds, etc.
pub async fn query(
    explorer: &Explorer,
    start: &str, // ISO 8601
    end: &str,
    min_mag: f64,
    max_mag: Option<f64>,
    limit: u32,
) -> Result<FeatureCollection> {
    let mut url = format!(
        "{BASE}?format=geojson&starttime={start}&endtime={end}&minmagnitude={min_mag}&limit={limit}&orderby=magnitude"
    );
    if let Some(max) = max_mag {
        url.push_str(&format!("&maxmagnitude={max}"));
    }
    explorer.fetch_json(&url).await
}

/// Print a summary table of earthquake features.
pub fn print_summary(data: &FeatureCollection) {
    use colored::Colorize;

    println!(
        "\n{} — {} events\n",
        data.metadata.title.bold(),
        data.metadata.count
    );
    println!(
        "  {:>5}  {:>7}  {:>7}  {}",
        "Mag".bold(),
        "Depth".bold(),
        "Sig".bold(),
        "Location".bold()
    );
    println!("  {}", "─".repeat(70));

    for f in &data.features {
        let mag = f.properties.mag.unwrap_or(0.0);
        let mag_str = format!("{mag:5.1}");
        let mag_colored = if mag >= 6.0 {
            mag_str.red().bold()
        } else if mag >= 4.5 {
            mag_str.yellow()
        } else {
            mag_str.normal()
        };

        println!(
            "  {}  {:>6.1}km  {:>7}  {}",
            mag_colored,
            f.depth_km(),
            f.properties.sig.unwrap_or(0),
            f.properties.place.as_deref().unwrap_or("Unknown"),
        );
    }
}
