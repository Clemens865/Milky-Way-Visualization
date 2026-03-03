//! ESA Gaia DR3 — 1.8 billion stars in the Milky Way.
//! No auth required. TAP/ADQL API via POST.
//! https://gea.esac.esa.int/archive/
//!
//! Key columns (DR3):
//!   parallax (mas) → distance (pc) = 1000 / parallax
//!   bp_rp: color index — blue/hot < 0.5, Sun ~0.82, red/cool > 2.0
//!   teff_gspphot: effective temperature from GSP-Phot (K)
//!   logg_gspphot: surface gravity (log g)
//!   distance_gspphot: photo-geometric distance (pc)

use crate::client::Explorer;
use anyhow::Result;

const TAP_URL: &str = "https://gea.esac.esa.int/tap-server/tap/sync";

/// Standard columns we query from gaia_source.
const COLS: &str = "source_id, ra, dec, parallax, pmra, pmdec, \
    phot_g_mean_mag, bp_rp, teff_gspphot, logg_gspphot, \
    distance_gspphot, radial_velocity, ruwe";

#[derive(Debug, Clone)]
pub struct Star {
    pub source_id: Option<u64>,
    pub ra: Option<f64>,
    pub dec: Option<f64>,
    pub parallax: Option<f64>,     // mas
    pub pmra: Option<f64>,         // proper motion RA (mas/yr)
    pub pmdec: Option<f64>,        // proper motion Dec (mas/yr)
    pub phot_g_mean_mag: Option<f64>,
    pub bp_rp: Option<f64>,        // color index
    pub teff: Option<f64>,         // effective temperature (K)
    pub logg: Option<f64>,         // surface gravity
    pub distance_pc: Option<f64>,  // photo-geometric distance
    pub radial_velocity: Option<f64>,
    pub ruwe: Option<f64>,
}

impl Star {
    /// Distance in parsecs — prefer GSP-Phot distance, fallback to parallax.
    pub fn dist_pc(&self) -> Option<f64> {
        self.distance_pc
            .or_else(|| self.parallax.filter(|&p| p > 0.0).map(|p| 1000.0 / p))
    }

    /// Distance in light-years.
    pub fn dist_ly(&self) -> Option<f64> {
        self.dist_pc().map(|d| d * 3.26156)
    }

    /// Absolute G magnitude.
    pub fn abs_mag(&self) -> Option<f64> {
        match (self.phot_g_mean_mag, self.dist_pc()) {
            (Some(m), Some(d)) if d > 0.0 => Some(m - 5.0 * (d / 10.0).log10()),
            _ => None,
        }
    }

    /// Rough luminosity in solar luminosities from absolute magnitude.
    /// L/L☉ = 10^((M_sun - M_star) / 2.5), where M_sun ≈ 4.83 in G-band.
    pub fn luminosity(&self) -> Option<f64> {
        self.abs_mag().map(|m| 10.0_f64.powf((4.83 - m) / 2.5))
    }

    /// Total proper motion (mas/yr).
    pub fn total_pm(&self) -> Option<f64> {
        match (self.pmra, self.pmdec) {
            (Some(a), Some(d)) => Some((a * a + d * d).sqrt()),
            _ => None,
        }
    }

    /// Tangential velocity (km/s) from proper motion and distance.
    pub fn tangential_velocity(&self) -> Option<f64> {
        match (self.total_pm(), self.dist_pc()) {
            (Some(pm), Some(d)) => Some(4.74047 * pm * d / 1000.0), // km/s
            _ => None,
        }
    }

    /// Heliocentric Cartesian position (pc) and velocity (pc/yr).
    /// Returns `(x, y, z, vx, vy, vz)`.
    pub fn cartesian_pos_vel(&self) -> (f64, f64, f64, f64, f64, f64) {
        const MAS_TO_RAD: f64 = 4.848_136_811_095_36e-9;
        const KMS_TO_PCYR: f64 = 1.022_7e-6;

        let dist = self.dist_pc().unwrap_or(0.0);
        let ra_rad = self.ra.unwrap_or(0.0).to_radians();
        let dec_rad = self.dec.unwrap_or(0.0).to_radians();
        let (sin_ra, cos_ra) = ra_rad.sin_cos();
        let (sin_dec, cos_dec) = dec_rad.sin_cos();

        let x = dist * cos_dec * cos_ra;
        let y = dist * cos_dec * sin_ra;
        let z = dist * sin_dec;

        let v_a = self.pmra.unwrap_or(0.0) * MAS_TO_RAD * dist;
        let v_d = self.pmdec.unwrap_or(0.0) * MAS_TO_RAD * dist;
        let v_r = self.radial_velocity.unwrap_or(0.0) * KMS_TO_PCYR;

        let vx = v_r * cos_dec * cos_ra - v_a * sin_ra - v_d * sin_dec * cos_ra;
        let vy = v_r * cos_dec * sin_ra + v_a * cos_ra - v_d * sin_dec * sin_ra;
        let vz = v_r * sin_dec + v_d * cos_dec;

        (x, y, z, vx, vy, vz)
    }

    pub fn spectral_class(&self) -> &'static str {
        match self.teff {
            Some(t) if t >= 30000.0 => "O (blue)",
            Some(t) if t >= 10000.0 => "B (blue-white)",
            Some(t) if t >= 7500.0 => "A (white)",
            Some(t) if t >= 6000.0 => "F (yellow-white)",
            Some(t) if t >= 5200.0 => "G (Sun-like)",
            Some(t) if t >= 3700.0 => "K (orange)",
            Some(t) if t >= 2400.0 => "M (red dwarf)",
            Some(_) => "L/T (cool)",
            None => "?",
        }
    }
}

/// Execute ADQL query via POST and parse the JSON response.
async fn adql_query(explorer: &Explorer, query: &str) -> Result<Vec<Star>> {
    let body = explorer
        .post_form(
            TAP_URL,
            &[
                ("REQUEST", "doQuery"),
                ("LANG", "ADQL"),
                ("FORMAT", "json"),
                ("QUERY", query),
            ],
        )
        .await?;

    let raw: serde_json::Value = serde_json::from_str(&body)?;

    let columns: Vec<String> = raw["metadata"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["name"].as_str().map(|s| s.to_lowercase()))
                .collect()
        })
        .unwrap_or_default();

    let rows = raw["data"].as_array().cloned().unwrap_or_default();

    let mut stars = Vec::with_capacity(rows.len());
    for row in &rows {
        let vals = match row.as_array() {
            Some(v) => v,
            None => continue,
        };

        let f = |name: &str| -> Option<f64> {
            columns
                .iter()
                .position(|c| c == name)
                .and_then(|i| vals.get(i))
                .and_then(|v| v.as_f64())
        };
        let u = |name: &str| -> Option<u64> {
            columns
                .iter()
                .position(|c| c == name)
                .and_then(|i| vals.get(i))
                .and_then(|v| v.as_u64().or_else(|| v.as_f64().map(|x| x as u64)))
        };

        stars.push(Star {
            source_id: u("source_id"),
            ra: f("ra"),
            dec: f("dec"),
            parallax: f("parallax"),
            pmra: f("pmra"),
            pmdec: f("pmdec"),
            phot_g_mean_mag: f("phot_g_mean_mag"),
            bp_rp: f("bp_rp"),
            teff: f("teff_gspphot"),
            logg: f("logg_gspphot"),
            distance_pc: f("distance_gspphot"),
            radial_velocity: f("radial_velocity"),
            ruwe: f("ruwe"),
        });
    }

    Ok(stars)
}

// ── Query functions ───────────────────────────────────────────────

/// Nearest stars to the Sun (within ~33 ly / 10 pc).
pub async fn nearest_stars(explorer: &Explorer, limit: u32) -> Result<Vec<Star>> {
    // parallax > 100 mas = within ~10 pc. Fast because it's a small result set.
    let q = format!(
        "SELECT TOP {limit} {COLS} FROM gaiadr3.gaia_source \
         WHERE parallax > 100 \
         ORDER BY parallax DESC"
    );
    adql_query(explorer, &q).await
}

/// Stars within a given distance in parsecs.
pub async fn within_distance(explorer: &Explorer, max_pc: f64, limit: u32) -> Result<Vec<Star>> {
    let min_plx = 1000.0 / max_pc;
    let q = format!(
        "SELECT TOP {limit} {COLS} FROM gaiadr3.gaia_source \
         WHERE parallax > {min_plx} \
         ORDER BY parallax DESC"
    );
    adql_query(explorer, &q).await
}

/// Stars within distance, cursor-paginated by source_id.
pub async fn within_distance_after(
    explorer: &Explorer,
    max_pc: f64,
    limit: u32,
    after_source_id: u64,
) -> Result<Vec<Star>> {
    let min_plx = 1000.0 / max_pc;
    let q = format!(
        "SELECT TOP {limit} {COLS} FROM gaiadr3.gaia_source \
         WHERE parallax > {min_plx} AND source_id > {after_source_id} \
         ORDER BY source_id ASC"
    );
    adql_query(explorer, &q).await
}

/// Brightest stars (apparent magnitude).
pub async fn bright_stars(explorer: &Explorer, max_mag: f64, limit: u32) -> Result<Vec<Star>> {
    let q = format!(
        "SELECT TOP {limit} {COLS} FROM gaiadr3.gaia_source \
         WHERE phot_g_mean_mag < {max_mag} AND parallax > 0 \
         ORDER BY phot_g_mean_mag ASC"
    );
    adql_query(explorer, &q).await
}

/// Sun-like stars: G-type temperature, main-sequence surface gravity.
pub async fn sun_like(explorer: &Explorer, max_pc: f64, limit: u32) -> Result<Vec<Star>> {
    let min_plx = 1000.0 / max_pc;
    let q = format!(
        "SELECT TOP {limit} {COLS} FROM gaiadr3.gaia_source \
         WHERE teff_gspphot BETWEEN 5400 AND 6000 \
         AND logg_gspphot BETWEEN 4.0 AND 4.8 \
         AND parallax > {min_plx} \
         ORDER BY parallax DESC"
    );
    adql_query(explorer, &q).await
}

/// Fastest-moving nearby stars (high proper motion, within 20 pc).
pub async fn fast_movers(explorer: &Explorer, limit: u32) -> Result<Vec<Star>> {
    // Compute pm_total as a column so we can ORDER BY it
    let q = format!(
        "SELECT TOP {limit} {COLS}, \
         SQRT(POWER(pmra, 2) + POWER(pmdec, 2)) AS pm_total \
         FROM gaiadr3.gaia_source \
         WHERE parallax > 50 AND pmra IS NOT NULL AND pmdec IS NOT NULL \
         ORDER BY pm_total DESC"
    );
    adql_query(explorer, &q).await
}

/// Spectral census — random sample with temperature data within a distance.
pub async fn spectral_census(explorer: &Explorer, max_pc: f64) -> Result<Vec<Star>> {
    let min_plx = 1000.0 / max_pc;
    let q = format!(
        "SELECT TOP 2000 {COLS} FROM gaiadr3.gaia_source \
         WHERE teff_gspphot IS NOT NULL AND parallax > {min_plx} \
         ORDER BY random_index"
    );
    adql_query(explorer, &q).await
}

/// Stars within distance, filtered by RA band (for parallel/chunked fetching).
/// ra_min/ra_max in degrees [0, 360).
pub async fn within_distance_ra_band(
    explorer: &Explorer,
    max_pc: f64,
    ra_min: f64,
    ra_max: f64,
    limit: u32,
) -> Result<Vec<Star>> {
    let min_plx = 1000.0 / max_pc;
    let q = format!(
        "SELECT TOP {limit} {COLS} FROM gaiadr3.gaia_source \
         WHERE parallax > {min_plx} AND ra >= {ra_min} AND ra < {ra_max} \
         ORDER BY source_id ASC"
    );
    adql_query(explorer, &q).await
}

/// Custom ADQL query.
pub async fn custom_query(explorer: &Explorer, query: &str) -> Result<Vec<Star>> {
    adql_query(explorer, query).await
}

// ── Display ──────────────────────────────────────────────────────

pub fn print_stars(stars: &[Star], title: &str) {
    use colored::Colorize;

    println!("\n{} — {} stars\n", title.bold(), stars.len());
    println!(
        "  {:>8}  {:>8}  {:>7}  {:>6}  {:>7}  {}",
        "Dist(ly)".bold(),
        "Mag".bold(),
        "T(K)".bold(),
        "Color".bold(),
        "PM″/yr".bold(),
        "Class".bold(),
    );
    println!("  {}", "─".repeat(70));

    for s in stars {
        let class = s.spectral_class();
        let class_colored = match class.chars().next() {
            Some('O') => class.blue().bold().to_string(),
            Some('B') => class.cyan().to_string(),
            Some('A') => class.white().bold().to_string(),
            Some('F') => class.yellow().to_string(),
            Some('G') => class.yellow().bold().to_string(),
            Some('K') => class.truecolor(255, 165, 0).to_string(),
            Some('M') => class.red().to_string(),
            _ => class.dimmed().to_string(),
        };

        println!(
            "  {:>8.1}  {:>8.2}  {:>7.0}  {:>6.2}  {:>7.1}  {}",
            s.dist_ly().unwrap_or(0.0),
            s.phot_g_mean_mag.unwrap_or(0.0),
            s.teff.unwrap_or(0.0),
            s.bp_rp.unwrap_or(0.0),
            s.total_pm().unwrap_or(0.0),
            class_colored,
        );
    }

    // Summary statistics
    let dists: Vec<f64> = stars.iter().filter_map(|s| s.dist_ly()).collect();
    let temps: Vec<f64> = stars.iter().filter_map(|s| s.teff).collect();

    println!();
    if let Some(s) = crate::analysis::stats::summarize(&dists) {
        println!("  Distance (ly): {s}");
    }
    if let Some(s) = crate::analysis::stats::summarize(&temps) {
        println!("  Temperature (K): {s}");
    }
}

pub fn print_census(stars: &[Star]) {
    use colored::Colorize;
    use std::collections::BTreeMap;

    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for s in stars {
        *counts.entry(s.spectral_class()).or_insert(0) += 1;
    }

    let total = stars.len();
    println!(
        "\n{} — {} stars sampled\n",
        "Spectral Census".bold(),
        total
    );

    let order = [
        "O (blue)",
        "B (blue-white)",
        "A (white)",
        "F (yellow-white)",
        "G (Sun-like)",
        "K (orange)",
        "M (red dwarf)",
        "L/T (cool)",
    ];

    for class in &order {
        if let Some(&count) = counts.get(class) {
            let pct = (count as f64 / total as f64) * 100.0;
            let bar_len = (pct * 0.5) as usize;
            let bar = "█".repeat(bar_len);

            let colored_bar = match class.chars().next() {
                Some('O') => bar.blue().bold().to_string(),
                Some('B') => bar.cyan().to_string(),
                Some('A') => bar.white().bold().to_string(),
                Some('F') => bar.yellow().to_string(),
                Some('G') => bar.yellow().bold().to_string(),
                Some('K') => bar.truecolor(255, 165, 0).to_string(),
                Some('M') => bar.red().to_string(),
                _ => bar.dimmed().to_string(),
            };

            println!(
                "  {:>22}  {:>5} ({:>5.1}%)  {}",
                class, count, pct, colored_bar
            );
        }
    }

    let temps: Vec<f64> = stars.iter().filter_map(|s| s.teff).collect();
    if let Some(s) = crate::analysis::stats::summarize(&temps) {
        println!("\n  Temperature (K): {s}");
    }
}
