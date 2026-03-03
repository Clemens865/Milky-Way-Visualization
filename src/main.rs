mod analysis;
mod cache;
mod client;
mod sources;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use futures::stream::{self, StreamExt};

#[derive(Parser)]
#[command(name = "undiscovered", about = "Scientific data exploration toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Explore earthquake data (USGS)
    Quake {
        #[command(subcommand)]
        sub: QuakeCmd,
    },
    /// Explore exoplanet data (NASA)
    Exo {
        #[command(subcommand)]
        sub: ExoCmd,
    },
    /// Explore gravitational wave events (LIGO/GWOSC)
    Gw {
        #[command(subcommand)]
        sub: GwCmd,
    },
    /// Explore integer sequences (OEIS)
    Oeis {
        #[command(subcommand)]
        sub: OeisCmd,
    },
    /// Explore the Milky Way via Gaia DR3 (1.8B stars)
    Stars {
        #[command(subcommand)]
        sub: StarsCmd,
    },
    /// Explore drug adverse events (OpenFDA)
    Fda {
        #[command(subcommand)]
        sub: FdaCmd,
    },
    /// Cache management
    Cache {
        #[command(subcommand)]
        sub: CacheCmd,
    },
}

#[derive(Subcommand)]
enum QuakeCmd {
    /// Show recent earthquakes
    Recent {
        /// Period: hour, day, week, month
        #[arg(default_value = "day")]
        period: String,
        /// Minimum magnitude: significant, 4.5, 2.5, 1.0, all
        #[arg(short, long, default_value = "4.5")]
        min_mag: String,
    },
    /// Query earthquakes by date range
    Query {
        /// Start date (ISO 8601)
        start: String,
        /// End date (ISO 8601)
        end: String,
        /// Minimum magnitude
        #[arg(short, long, default_value_t = 4.0)]
        min_mag: f64,
        /// Max results
        #[arg(short, long, default_value_t = 50)]
        limit: u32,
    },
    /// Statistical summary of recent seismic activity
    Stats {
        /// Period: hour, day, week, month
        #[arg(default_value = "week")]
        period: String,
    },
}

#[derive(Subcommand)]
enum ExoCmd {
    /// List recently discovered exoplanets
    Recent {
        #[arg(short, long, default_value_t = 20)]
        limit: u32,
    },
    /// Find potentially habitable exoplanets
    Habitable,
    /// Search by discovery method (Transit, Radial Velocity, etc.)
    Method {
        method: String,
        #[arg(short, long, default_value_t = 20)]
        limit: u32,
    },
}

#[derive(Subcommand)]
enum GwCmd {
    /// List all gravitational wave events
    List,
    /// Show events from a specific catalog
    Catalog {
        /// e.g., GWTC-3-confident
        name: String,
    },
    /// Statistical summary of GW events
    Stats,
}

#[derive(Subcommand)]
enum OeisCmd {
    /// Search sequences by text
    Search {
        query: String,
        #[arg(short, long, default_value_t = 10)]
        limit: u32,
    },
    /// Look up a sequence by A-number
    Lookup {
        /// A-number (just the number, e.g., 45 for Fibonacci)
        number: u64,
    },
    /// Find sequences matching given values
    Values {
        /// Comma-separated values, e.g., "1,1,2,3,5,8"
        values: String,
    },
}

#[derive(Subcommand)]
enum StarsCmd {
    /// Nearest stars to the Sun
    Nearest {
        #[arg(short, long, default_value_t = 30)]
        limit: u32,
    },
    /// Stars within a given distance (parsecs)
    Within {
        /// Maximum distance in parsecs (1 pc = 3.26 ly)
        distance_pc: f64,
        #[arg(short, long, default_value_t = 50)]
        limit: u32,
    },
    /// Brightest stars visible to the naked eye
    Bright {
        /// Maximum apparent magnitude (6.0 = naked eye limit)
        #[arg(short, long, default_value_t = 4.0)]
        max_mag: f64,
        #[arg(short, long, default_value_t = 50)]
        limit: u32,
    },
    /// Sun-like stars (G-type, similar luminosity)
    SunLike {
        /// Search radius in parsecs
        #[arg(short, long, default_value_t = 100.0)]
        distance_pc: f64,
        #[arg(short, long, default_value_t = 30)]
        limit: u32,
    },
    /// Fastest-moving stars in the solar neighborhood
    Fast {
        #[arg(short, long, default_value_t = 20)]
        limit: u32,
    },
    /// Census of spectral types within a distance
    Census {
        /// Distance in parsecs
        #[arg(short, long, default_value_t = 100.0)]
        distance_pc: f64,
    },
    /// Custom ADQL query (power user)
    Query {
        /// ADQL query string
        adql: String,
    },
    /// Export star data as JSON for visualization
    Export {
        /// Max distance in parsecs
        #[arg(short, long, default_value_t = 100.0)]
        distance_pc: f64,
        /// Max number of stars
        #[arg(short, long, default_value_t = 5000)]
        limit: u32,
        /// Output file
        #[arg(short, long, default_value = "web/stars.json")]
        output: String,
    },
    /// Analyze star data from binary files (KD-tree, clustering, streams)
    Analyze {
        #[command(subcommand)]
        sub: AnalyzeCmd,
    },
}

#[derive(Subcommand)]
enum AnalyzeCmd {
    /// Find k nearest neighbors to the Sun (origin)
    Neighbors {
        /// Input binary file (.bin or .cbin)
        #[arg(short, long)]
        input: String,
        /// Number of neighbors
        #[arg(short, long, default_value_t = 20)]
        k: usize,
    },
    /// Find all stars within a radius of the Sun
    Within {
        /// Input binary file
        #[arg(short, long)]
        input: String,
        /// Radius in parsecs
        #[arg(short, long, default_value_t = 5.0)]
        radius: f64,
    },
    /// DBSCAN clustering on star positions
    Clusters {
        /// Input binary file
        #[arg(short, long)]
        input: String,
        /// Neighborhood radius (parsecs)
        #[arg(short, long, default_value_t = 5.0)]
        eps: f64,
        /// Minimum points to form a cluster
        #[arg(short, long, default_value_t = 10)]
        min_points: usize,
    },
    /// Detect co-moving stellar streams (velocity-space clustering)
    Streams {
        /// Input binary file
        #[arg(short, long)]
        input: String,
        /// Velocity-space eps (km/s)
        #[arg(short, long, default_value_t = 2.0)]
        velocity_eps_kms: f64,
        /// Minimum members for a stream
        #[arg(short, long, default_value_t = 20)]
        min_members: usize,
    },
    /// Compute local stellar density around each star
    Density {
        /// Input binary file
        #[arg(short, long)]
        input: String,
        /// Radius for density calculation (parsecs)
        #[arg(short, long, default_value_t = 10.0)]
        radius: f64,
        /// Show top N densest stars
        #[arg(short, long, default_value_t = 30)]
        top: usize,
    },
}

#[derive(Subcommand)]
enum FdaCmd {
    /// Top adverse reactions for a drug
    Reactions {
        drug: String,
        #[arg(short, long, default_value_t = 25)]
        limit: u32,
    },
    /// Most reported drugs overall
    TopDrugs {
        #[arg(short, long, default_value_t = 25)]
        limit: u32,
    },
}

#[derive(Subcommand)]
enum CacheCmd {
    /// Show cache statistics
    Stats,
    /// Clear all cached responses
    Clear,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();

    let cli = Cli::parse();
    let explorer = client::Explorer::new();

    match cli.command {
        Command::Quake { sub } => match sub {
            QuakeCmd::Recent { period, min_mag } => {
                let data = sources::earthquakes::recent(&explorer, &period, &min_mag).await?;
                sources::earthquakes::print_summary(&data);
            }
            QuakeCmd::Query {
                start,
                end,
                min_mag,
                limit,
            } => {
                let data =
                    sources::earthquakes::query(&explorer, &start, &end, min_mag, None, limit)
                        .await?;
                sources::earthquakes::print_summary(&data);
            }
            QuakeCmd::Stats { period } => {
                let data = sources::earthquakes::recent(&explorer, &period, "all").await?;
                let mags: Vec<f64> = data
                    .features
                    .iter()
                    .filter_map(|f| f.properties.mag)
                    .collect();
                let depths: Vec<f64> = data.features.iter().map(|f| f.depth_km()).collect();

                println!("\n{}\n", "Seismic Activity Statistics".bold());
                println!("  Total events: {}", data.metadata.count);

                if let Some(s) = analysis::stats::summarize(&mags) {
                    println!("  Magnitude: {s}");
                }
                if let Some(s) = analysis::stats::summarize(&depths) {
                    println!("  Depth (km): {s}");
                }

                let outlier_idx = analysis::stats::outliers(&mags, 3.0);
                if !outlier_idx.is_empty() {
                    println!("\n  {} (IQR x3.0):", "Magnitude Outliers".yellow());
                    for idx in &outlier_idx {
                        let f = &data.features[*idx];
                        println!(
                            "    M{:.1} — {}",
                            f.properties.mag.unwrap_or(0.0),
                            f.properties.place.as_deref().unwrap_or("?")
                        );
                    }
                }
            }
        },

        Command::Exo { sub } => match sub {
            ExoCmd::Recent { limit } => {
                let planets = sources::exoplanets::confirmed(&explorer, limit).await?;
                sources::exoplanets::print_summary(&planets);
            }
            ExoCmd::Habitable => {
                let planets = sources::exoplanets::habitable_candidates(&explorer).await?;
                sources::exoplanets::print_summary(&planets);
            }
            ExoCmd::Method { method, limit } => {
                let planets = sources::exoplanets::by_method(&explorer, &method, limit).await?;
                sources::exoplanets::print_summary(&planets);
            }
        },

        Command::Gw { sub } => match sub {
            GwCmd::List => {
                let events = sources::gwosc::all_confident_events(&explorer).await?;
                sources::gwosc::print_summary(&events);
            }
            GwCmd::Catalog { name } => {
                let events = sources::gwosc::catalog_events(&explorer, &name).await?;
                sources::gwosc::print_summary(&events);
            }
            GwCmd::Stats => {
                let events = sources::gwosc::all_confident_events(&explorer).await?;
                let masses: Vec<f64> =
                    events.events.values().filter_map(|e| e.mass_1_source).collect();
                let distances: Vec<f64> =
                    events.events.values().filter_map(|e| e.luminosity_distance).collect();

                println!("\n{}\n", "Gravitational Wave Statistics".bold());
                println!("  Total events: {}", events.events.len());

                if let Some(s) = analysis::stats::summarize(&masses) {
                    println!("  Primary mass (M☉): {s}");
                }
                if let Some(s) = analysis::stats::summarize(&distances) {
                    println!("  Distance (Mpc): {s}");
                }

                if let Some(heaviest) = events
                    .events
                    .values()
                    .filter(|e| e.mass_1_source.is_some())
                    .max_by(|a, b| {
                        a.mass_1_source
                            .partial_cmp(&b.mass_1_source)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                {
                    println!(
                        "\n  {} {} — M1={:.1} M☉, M2={:.1} M☉, distance={:.0} Mpc",
                        "Heaviest merger:".yellow(),
                        heaviest.common_name.as_deref().unwrap_or("?"),
                        heaviest.mass_1_source.unwrap_or(0.0),
                        heaviest.mass_2_source.unwrap_or(0.0),
                        heaviest.luminosity_distance.unwrap_or(0.0),
                    );
                }
            }
        },

        Command::Oeis { sub } => match sub {
            OeisCmd::Search { query, limit } => {
                let result = sources::oeis::search(&explorer, &query, limit).await?;
                sources::oeis::print_summary(&result);
            }
            OeisCmd::Lookup { number } => {
                let result = sources::oeis::sequence(&explorer, number).await?;
                sources::oeis::print_summary(&result);
            }
            OeisCmd::Values { values } => {
                let parsed: Vec<i64> = values
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                let result = sources::oeis::search_by_values(&explorer, &parsed).await?;
                sources::oeis::print_summary(&result);
            }
        },

        Command::Stars { sub } => match sub {
            StarsCmd::Nearest { limit } => {
                let stars = sources::gaia::nearest_stars(&explorer, limit).await?;
                sources::gaia::print_stars(&stars, "Nearest Stars to the Sun");
            }
            StarsCmd::Within { distance_pc, limit } => {
                let stars =
                    sources::gaia::within_distance(&explorer, distance_pc, limit).await?;
                let title = format!("Stars within {distance_pc} pc ({:.1} ly)", distance_pc * 3.26156);
                sources::gaia::print_stars(&stars, &title);
            }
            StarsCmd::Bright { max_mag, limit } => {
                let stars = sources::gaia::bright_stars(&explorer, max_mag, limit).await?;
                sources::gaia::print_stars(&stars, &format!("Brightest Stars (mag < {max_mag})"));
            }
            StarsCmd::SunLike { distance_pc, limit } => {
                let stars =
                    sources::gaia::sun_like(&explorer, distance_pc, limit).await?;
                sources::gaia::print_stars(
                    &stars,
                    &format!("Sun-like Stars within {distance_pc} pc"),
                );
            }
            StarsCmd::Fast { limit } => {
                let stars = sources::gaia::fast_movers(&explorer, limit).await?;
                sources::gaia::print_stars(&stars, "Fastest-Moving Stars");
            }
            StarsCmd::Census { distance_pc } => {
                let stars =
                    sources::gaia::spectral_census(&explorer, distance_pc).await?;
                sources::gaia::print_census(&stars);
            }
            StarsCmd::Query { adql } => {
                let stars = sources::gaia::custom_query(&explorer, &adql).await?;
                sources::gaia::print_stars(&stars, "Custom Query Results");
            }
            StarsCmd::Export {
                distance_pc,
                limit,
                output,
            } => {
                use indicatif::{ProgressBar, ProgressStyle};

                let chunk_size = 50000u32;
                let mut all_stars = Vec::new();

                if limit <= chunk_size {
                    println!("Fetching stars within {} pc...", distance_pc);
                    all_stars =
                        sources::gaia::within_distance(&explorer, distance_pc, limit).await?;
                } else {
                    // Auto-select band count based on target size
                    let num_bands: u32 = if limit > 5_000_000 { 72 }
                        else if limit > 100_000 { 36 }
                        else { 18 };
                    let band_width = 360.0 / num_bands as f64;
                    let per_band_limit = (limit as f64 / num_bands as f64 * 2.0) as u32;

                    let pb = ProgressBar::new(num_bands as u64);
                    pb.set_style(
                        ProgressStyle::with_template(
                            "  Fetching [{bar:30}] band {pos}/{len} ({msg})",
                        )
                        .unwrap(),
                    );

                    // Concurrent fetching: up to 10 RA bands in parallel
                    let pb_ref = &pb;
                    let explorer_ref = &explorer;
                    let results: Vec<_> = stream::iter(0..num_bands)
                        .map(|band| {
                            let ra_min = band as f64 * band_width;
                            let ra_max = ra_min + band_width;
                            async move {
                                let result = sources::gaia::within_distance_ra_band(
                                    explorer_ref, distance_pc, ra_min, ra_max, per_band_limit,
                                )
                                .await;
                                (ra_min, ra_max, result)
                            }
                        })
                        .buffer_unordered(10)
                        .collect()
                        .await;

                    for (ra_min, ra_max, result) in results {
                        match result {
                            Ok(chunk) => all_stars.extend(chunk),
                            Err(e) => eprintln!(
                                "  Warning: RA band {ra_min:.0}–{ra_max:.0} failed: {e}"
                            ),
                        }
                        pb_ref.inc(1);
                        pb_ref.set_message(format!("{} stars", all_stars.len()));

                        if all_stars.len() >= limit as usize {
                            break;
                        }
                    }
                    pb.finish_with_message(format!("{} stars fetched", all_stars.len()));
                }

                // Filter to strict distance limit
                let before = all_stars.len();
                all_stars.retain(|s| {
                    s.dist_pc()
                        .map(|d| d <= distance_pc)
                        .unwrap_or(false)
                });
                if all_stars.len() < before {
                    println!(
                        "Got {} stars, filtered to {} within {} pc",
                        before, all_stars.len(), distance_pc
                    );
                } else {
                    println!("Got {} stars", all_stars.len());
                }
                // Truncate to limit
                if all_stars.len() > limit as usize {
                    all_stars.truncate(limit as usize);
                    println!("Truncated to {} stars", all_stars.len());
                }

                if let Some(parent) = std::path::Path::new(&output).parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Helper: compute position and velocity for a star
                let compute_pos_vel = |s: &sources::gaia::Star| {
                    let (x, y, z, vx, vy, vz) = s.cartesian_pos_vel();
                    let dist = s.dist_pc().unwrap_or(0.0);
                    (x, y, z, vx, vy, vz, dist)
                };

                if output.ends_with(".cbin") {
                    // Compact binary STR3: 16 bytes/star (for large datasets)
                    // Header: "STR3" + u32 count + f32 pos_scale + f32 vel_scale = 16 bytes
                    // Per star: x,y,z as i16 (6B) + vx,vy,vz as i16 (6B) + temp u16 (2B) + mag*100 i16 (2B) = 16B

                    // First pass: find max coordinate and max velocity for scaling
                    let mut max_coord: f64 = 1.0;
                    let mut max_vel: f64 = 1e-10;
                    for s in &all_stars {
                        let (x, y, z, vx, vy, vz, _) = compute_pos_vel(s);
                        max_coord = max_coord.max(x.abs()).max(y.abs()).max(z.abs());
                        max_vel = max_vel.max(vx.abs()).max(vy.abs()).max(vz.abs());
                    }

                    let count = all_stars.len() as u32;
                    let mut buf = Vec::with_capacity(all_stars.len() * 16 + 16);
                    buf.extend_from_slice(b"STR3");
                    buf.extend_from_slice(&count.to_le_bytes());
                    buf.extend_from_slice(&(max_coord as f32).to_le_bytes());
                    buf.extend_from_slice(&(max_vel as f32).to_le_bytes());

                    let to_i16 = |val: f64, scale: f64| -> i16 {
                        (val / scale * 32767.0).clamp(-32767.0, 32767.0) as i16
                    };

                    for s in &all_stars {
                        let (x, y, z, vx, vy, vz, _) = compute_pos_vel(s);
                        buf.extend_from_slice(&to_i16(x, max_coord).to_le_bytes());
                        buf.extend_from_slice(&to_i16(y, max_coord).to_le_bytes());
                        buf.extend_from_slice(&to_i16(z, max_coord).to_le_bytes());
                        buf.extend_from_slice(&to_i16(vx, max_vel).to_le_bytes());
                        buf.extend_from_slice(&to_i16(vy, max_vel).to_le_bytes());
                        buf.extend_from_slice(&to_i16(vz, max_vel).to_le_bytes());
                        buf.extend_from_slice(&(s.teff.unwrap_or(0.0) as u16).to_le_bytes());
                        let mag_i16 = ((s.phot_g_mean_mag.unwrap_or(0.0) * 100.0)
                            .clamp(-32767.0, 32767.0)) as i16;
                        buf.extend_from_slice(&mag_i16.to_le_bytes());
                    }

                    std::fs::write(&output, &buf)?;
                    println!(
                        "Exported {} stars to {output} ({:.1} MB, compact binary)\n  pos_scale={max_coord:.1} vel_scale={max_vel:.2e}",
                        all_stars.len(), buf.len() as f64 / (1024.0 * 1024.0)
                    );
                } else if output.ends_with(".bin") {
                    // STR2 format: 10 x f32 per star = 40 bytes
                    let mut buf = Vec::with_capacity(all_stars.len() * 40 + 8);
                    buf.extend_from_slice(b"STR2");
                    buf.extend_from_slice(&(all_stars.len() as u32).to_le_bytes());

                    for s in &all_stars {
                        let (x, y, z, vx, vy, vz, dist) = compute_pos_vel(s);
                        for val in [x, y, z, vx, vy, vz,
                                    s.teff.unwrap_or(0.0),
                                    s.phot_g_mean_mag.unwrap_or(0.0),
                                    s.bp_rp.unwrap_or(0.0), dist] {
                            buf.extend_from_slice(&(val as f32).to_le_bytes());
                        }
                    }

                    std::fs::write(&output, &buf)?;
                    println!(
                        "Exported {} stars to {output} ({:.1} MB, binary)",
                        all_stars.len(), buf.len() as f64 / (1024.0 * 1024.0)
                    );
                } else {
                    let export: Vec<serde_json::Value> = all_stars
                        .iter()
                        .map(|s| {
                            let dist = s.dist_pc().unwrap_or(0.0);
                            let ra_rad = s.ra.unwrap_or(0.0).to_radians();
                            let dec_rad = s.dec.unwrap_or(0.0).to_radians();
                            serde_json::json!({
                                "x": dist * dec_rad.cos() * ra_rad.cos(),
                                "y": dist * dec_rad.cos() * ra_rad.sin(),
                                "z": dist * dec_rad.sin(),
                                "dist_pc": dist,
                                "dist_ly": dist * 3.26156,
                                "ra": s.ra,
                                "dec": s.dec,
                                "mag": s.phot_g_mean_mag,
                                "temp": s.teff,
                                "bp_rp": s.bp_rp,
                                "class": s.spectral_class(),
                                "lum": s.luminosity(),
                            })
                        })
                        .collect();

                    let json = serde_json::to_string(&export)?;
                    std::fs::write(&output, &json)?;
                    println!(
                        "Exported {} stars to {output} ({:.1} MB, JSON)",
                        export.len(),
                        json.len() as f64 / (1024.0 * 1024.0)
                    );
                }
            }
            StarsCmd::Analyze { sub } => {
                use std::time::Instant;

                match sub {
                    AnalyzeCmd::Neighbors { input, k } => {
                        let t0 = Instant::now();
                        let path = std::path::Path::new(&input);
                        let stars = analysis::loader::load_stars(path)?;
                        println!(
                            "Loaded {} stars from {} ({:.2}s)",
                            stars.len(),
                            input,
                            t0.elapsed().as_secs_f64()
                        );

                        let t1 = Instant::now();
                        let tree = analysis::spatial::build_kdtree(&stars);
                        println!(
                            "Built KD-tree ({:.2}s)",
                            t1.elapsed().as_secs_f64()
                        );

                        let origin = [0.0, 0.0, 0.0]; // Sun
                        let neighbors = analysis::spatial::nearest_neighbors(&tree, &origin, k);

                        println!(
                            "\n{}\n",
                            format!("{k} Nearest Stars to the Sun").bold()
                        );
                        println!(
                            "  {:>4}  {:>10}  {:>10}  {:>7}  {:>10}",
                            "#".bold(),
                            "Dist(pc)".bold(),
                            "Dist(ly)".bold(),
                            "Temp(K)".bold(),
                            "Mag".bold(),
                        );
                        println!("  {}", "─".repeat(50));

                        for (rank, nb) in neighbors.iter().enumerate() {
                            let s = &stars[nb.index];
                            println!(
                                "  {:>4}  {:>10.3}  {:>10.3}  {:>7.0}  {:>10.2}",
                                rank + 1,
                                nb.distance_pc,
                                nb.distance_pc * 3.26156,
                                s.temp,
                                s.mag,
                            );
                        }
                        println!("\n  Total time: {:.2}s", t0.elapsed().as_secs_f64());
                    }

                    AnalyzeCmd::Within { input, radius } => {
                        let t0 = Instant::now();
                        let path = std::path::Path::new(&input);
                        let stars = analysis::loader::load_stars(path)?;
                        println!(
                            "Loaded {} stars ({:.2}s)",
                            stars.len(),
                            t0.elapsed().as_secs_f64()
                        );

                        let t1 = Instant::now();
                        let tree = analysis::spatial::build_kdtree(&stars);
                        println!("Built KD-tree ({:.2}s)", t1.elapsed().as_secs_f64());

                        let origin = [0.0, 0.0, 0.0];
                        let mut found =
                            analysis::spatial::within_radius(&tree, &origin, radius);
                        found.sort_by(|a, b| {
                            a.distance_pc
                                .partial_cmp(&b.distance_pc)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });

                        println!(
                            "\n{}\n",
                            format!(
                                "{} stars within {radius} pc ({:.1} ly) of the Sun",
                                found.len(),
                                radius * 3.26156
                            )
                            .bold()
                        );

                        for (i, nb) in found.iter().take(100).enumerate() {
                            let s = &stars[nb.index];
                            println!(
                                "  {:>4}. {:>8.3} pc  {:>7.0} K  mag {:>5.2}",
                                i + 1,
                                nb.distance_pc,
                                s.temp,
                                s.mag,
                            );
                        }
                        if found.len() > 100 {
                            println!("  ... and {} more", found.len() - 100);
                        }
                        println!("\n  Total time: {:.2}s", t0.elapsed().as_secs_f64());
                    }

                    AnalyzeCmd::Clusters {
                        input,
                        eps,
                        min_points,
                    } => {
                        let t0 = Instant::now();
                        let path = std::path::Path::new(&input);
                        let stars = analysis::loader::load_stars(path)?;
                        println!(
                            "Loaded {} stars ({:.2}s)",
                            stars.len(),
                            t0.elapsed().as_secs_f64()
                        );

                        let t1 = Instant::now();
                        let tree = analysis::spatial::build_kdtree(&stars);
                        println!("Built KD-tree ({:.2}s)", t1.elapsed().as_secs_f64());

                        let t2 = Instant::now();
                        let result =
                            analysis::cluster::dbscan(&tree, &stars, eps, min_points);
                        println!(
                            "DBSCAN complete ({:.2}s): {} clusters, {} noise",
                            t2.elapsed().as_secs_f64(),
                            result.num_clusters,
                            result.noise_count(),
                        );

                        let summaries = result.summaries(&stars);
                        println!(
                            "\n{}\n",
                            format!(
                                "Clusters (eps={eps} pc, min_points={min_points})"
                            )
                            .bold()
                        );
                        println!(
                            "  {:>4}  {:>6}  {:>28}  {:>10}",
                            "ID".bold(),
                            "Size".bold(),
                            "Centroid (x, y, z) pc".bold(),
                            "Spread(pc)".bold(),
                        );
                        println!("  {}", "─".repeat(55));

                        for cs in summaries.iter().take(30) {
                            println!(
                                "  {:>4}  {:>6}  ({:>7.1}, {:>7.1}, {:>7.1})  {:>10.2}",
                                cs.id,
                                cs.size,
                                cs.centroid[0],
                                cs.centroid[1],
                                cs.centroid[2],
                                cs.spread_pc,
                            );
                        }
                        if summaries.len() > 30 {
                            println!("  ... and {} more clusters", summaries.len() - 30);
                        }
                        println!("\n  Total time: {:.2}s", t0.elapsed().as_secs_f64());
                    }

                    AnalyzeCmd::Streams {
                        input,
                        velocity_eps_kms,
                        min_members,
                    } => {
                        let t0 = Instant::now();
                        let path = std::path::Path::new(&input);
                        let stars = analysis::loader::load_stars(path)?;
                        println!(
                            "Loaded {} stars ({:.2}s)",
                            stars.len(),
                            t0.elapsed().as_secs_f64()
                        );

                        let t1 = Instant::now();
                        let streams = analysis::streams::detect_streams(
                            &stars,
                            velocity_eps_kms,
                            min_members,
                        );
                        println!(
                            "Stream detection complete ({:.2}s): {} streams found",
                            t1.elapsed().as_secs_f64(),
                            streams.len(),
                        );

                        println!(
                            "\n{}\n",
                            "Co-moving Stellar Streams".bold()
                        );
                        println!(
                            "  {:>4}  {:>6}  {:>34}  {:>10}  {:>10}",
                            "ID".bold(),
                            "Stars".bold(),
                            "Mean velocity (vx,vy,vz) km/s".bold(),
                            "Vel disp".bold(),
                            "Spread(pc)".bold(),
                        );
                        println!("  {}", "─".repeat(72));

                        for st in streams.iter().take(30) {
                            println!(
                                "  {:>4}  {:>6}  ({:>8.1}, {:>8.1}, {:>8.1})  {:>8.2}  {:>10.2}",
                                st.id,
                                st.members.len(),
                                st.mean_vel_kms[0],
                                st.mean_vel_kms[1],
                                st.mean_vel_kms[2],
                                st.vel_dispersion_kms,
                                st.spatial_spread_pc,
                            );
                        }
                        if streams.len() > 30 {
                            println!("  ... and {} more streams", streams.len() - 30);
                        }
                        println!("\n  Total time: {:.2}s", t0.elapsed().as_secs_f64());
                    }

                    AnalyzeCmd::Density {
                        input,
                        radius,
                        top,
                    } => {
                        let t0 = Instant::now();
                        let path = std::path::Path::new(&input);
                        let stars = analysis::loader::load_stars(path)?;
                        println!(
                            "Loaded {} stars ({:.2}s)",
                            stars.len(),
                            t0.elapsed().as_secs_f64()
                        );

                        let t1 = Instant::now();
                        let tree = analysis::spatial::build_kdtree(&stars);
                        println!("Built KD-tree ({:.2}s)", t1.elapsed().as_secs_f64());

                        let t2 = Instant::now();
                        let densities =
                            analysis::spatial::local_density(&tree, &stars, radius);
                        println!(
                            "Density computed ({:.2}s)",
                            t2.elapsed().as_secs_f64()
                        );

                        // Sort by density descending
                        let mut indexed: Vec<(usize, u32)> =
                            densities.iter().copied().enumerate().collect();
                        indexed.sort_by(|a, b| b.1.cmp(&a.1));

                        println!(
                            "\n{}\n",
                            format!("Top {top} Densest Regions (r={radius} pc)").bold()
                        );
                        println!(
                            "  {:>4}  {:>10}  {:>28}  {:>7}  {:>6}",
                            "#".bold(),
                            "Neighbors".bold(),
                            "Position (x, y, z) pc".bold(),
                            "Temp(K)".bold(),
                            "Mag".bold(),
                        );
                        println!("  {}", "─".repeat(62));

                        for (rank, &(i, count)) in indexed.iter().take(top).enumerate() {
                            let s = &stars[i];
                            println!(
                                "  {:>4}  {:>10}  ({:>7.1}, {:>7.1}, {:>7.1})  {:>7.0}  {:>6.2}",
                                rank + 1,
                                count,
                                s.x,
                                s.y,
                                s.z,
                                s.temp,
                                s.mag,
                            );
                        }

                        // Stats
                        let density_vals: Vec<f64> =
                            densities.iter().map(|&d| d as f64).collect();
                        if let Some(s) = analysis::stats::summarize(&density_vals) {
                            println!("\n  Density stats: {s}");
                        }
                        println!("  Total time: {:.2}s", t0.elapsed().as_secs_f64());
                    }
                }
            }
        },

        Command::Fda { sub } => match sub {
            FdaCmd::Reactions { drug, limit } => {
                let result = sources::openfda::reaction_counts(&explorer, &drug, limit).await?;
                sources::openfda::print_reactions(&drug, &result);
            }
            FdaCmd::TopDrugs { limit } => {
                let result = sources::openfda::top_drugs(&explorer, limit).await?;

                println!("\n{}\n", "Most Reported Drugs (Adverse Events)".bold());
                if let Some(ref results) = result.results {
                    for (i, r) in results.iter().enumerate() {
                        println!("  {:>3}. {:>8} reports — {}", i + 1, r.count, r.term);
                    }
                }
            }
        },

        Command::Cache { sub } => match sub {
            CacheCmd::Stats => {
                let stats = explorer.cache().stats()?;
                println!("\n{}\n", "Cache Statistics".bold());
                println!("  {stats}");
            }
            CacheCmd::Clear => {
                let removed = explorer.cache().clear()?;
                println!("Cleared {removed} cached responses.");
            }
        },
    }

    Ok(())
}
