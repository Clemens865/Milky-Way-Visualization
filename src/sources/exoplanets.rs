//! NASA Exoplanet Archive — confirmed exoplanets and their properties.
//! No auth required. TAP API with CSV/JSON output.
//! https://exoplanetarchive.ipac.caltech.edu/docs/TAP/usingTAP.html

use crate::client::Explorer;
use anyhow::Result;
use serde::Deserialize;

const TAP_BASE: &str = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync";

#[derive(Debug, Deserialize)]
pub struct Planet {
    pub pl_name: Option<String>,
    pub hostname: Option<String>,
    pub sy_dist: Option<f64>,     // distance in parsecs
    pub pl_orbper: Option<f64>,   // orbital period (days)
    pub pl_rade: Option<f64>,     // planet radius (Earth radii)
    pub pl_bmasse: Option<f64>,   // planet mass (Earth masses)
    pub pl_eqt: Option<f64>,     // equilibrium temperature (K)
    pub pl_orbsmax: Option<f64>, // semi-major axis (AU)
    pub st_teff: Option<f64>,    // stellar effective temperature (K)
    pub st_rad: Option<f64>,     // stellar radius (solar radii)
    pub disc_year: Option<i32>,  // discovery year
    pub discoverymethod: Option<String>,
}

/// Fetch confirmed exoplanets with key properties.
pub async fn confirmed(explorer: &Explorer, limit: u32) -> Result<Vec<Planet>> {
    let query = format!(
        "SELECT pl_name,hostname,sy_dist,pl_orbper,pl_rade,pl_bmasse,pl_eqt,\
         pl_orbsmax,st_teff,st_rad,disc_year,discoverymethod \
         FROM ps WHERE default_flag=1 \
         ORDER BY disc_year DESC \
         FETCH FIRST {limit} ROWS ONLY"
    );
    let url = format!(
        "{TAP_BASE}?query={}&format=json",
        urlencoded(&query)
    );
    explorer.fetch_json(&url).await
}

/// Find potentially habitable planets (rough criteria).
pub async fn habitable_candidates(explorer: &Explorer) -> Result<Vec<Planet>> {
    let query =
        "SELECT pl_name,hostname,sy_dist,pl_orbper,pl_rade,pl_bmasse,pl_eqt,\
         pl_orbsmax,st_teff,st_rad,disc_year,discoverymethod \
         FROM ps WHERE default_flag=1 \
         AND pl_rade BETWEEN 0.5 AND 2.0 \
         AND pl_eqt BETWEEN 200 AND 320 \
         ORDER BY pl_eqt ASC";
    let url = format!(
        "{TAP_BASE}?query={}&format=json",
        urlencoded(query)
    );
    explorer.fetch_json(&url).await
}

/// Search by discovery method.
pub async fn by_method(
    explorer: &Explorer,
    method: &str,
    limit: u32,
) -> Result<Vec<Planet>> {
    let query = format!(
        "SELECT pl_name,hostname,sy_dist,pl_orbper,pl_rade,pl_bmasse,pl_eqt,\
         pl_orbsmax,st_teff,st_rad,disc_year,discoverymethod \
         FROM ps WHERE default_flag=1 \
         AND discoverymethod='{method}' \
         ORDER BY disc_year DESC \
         FETCH FIRST {limit} ROWS ONLY"
    );
    let url = format!(
        "{TAP_BASE}?query={}&format=json",
        urlencoded(&query)
    );
    explorer.fetch_json(&url).await
}

pub fn print_summary(planets: &[Planet]) {
    use colored::Colorize;

    println!("\n{} — {} planets\n", "Exoplanet Results".bold(), planets.len());
    println!(
        "  {:>25}  {:>6}  {:>8}  {:>6}  {:>6}  {}",
        "Name".bold(),
        "Year".bold(),
        "Rad(Re)".bold(),
        "T(K)".bold(),
        "Dist".bold(),
        "Method".bold(),
    );
    println!("  {}", "─".repeat(80));

    for p in planets {
        let temp = p.pl_eqt.unwrap_or(0.0);
        let temp_str = format!("{temp:6.0}");
        let temp_colored = if (200.0..=320.0).contains(&temp) {
            temp_str.green()
        } else if temp > 500.0 {
            temp_str.red()
        } else {
            temp_str.normal()
        };

        println!(
            "  {:>25}  {:>6}  {:>8.2}  {}  {:>6.1}  {}",
            p.pl_name.as_deref().unwrap_or("?"),
            p.disc_year.map(|y| y.to_string()).unwrap_or_default(),
            p.pl_rade.unwrap_or(0.0),
            temp_colored,
            p.sy_dist.unwrap_or(0.0),
            p.discoverymethod.as_deref().unwrap_or("?"),
        );
    }
}

fn urlencoded(s: &str) -> String {
    s.replace(' ', "+")
        .replace('\'', "%27")
        .replace('=', "%3D")
        .replace(',', "%2C")
        .replace('(', "%28")
        .replace(')', "%29")
}
