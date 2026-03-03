//! Load stars from STR2/STR3 binary files into analysis-ready structs.

use anyhow::{bail, Result};
use std::path::Path;

/// A star with 3D position, velocity, temperature, and magnitude.
#[derive(Debug, Clone, Copy)]
pub struct StarPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub temp: f32,
    pub mag: f32,
}

impl StarPoint {
    /// Velocity magnitude in pc/yr.
    pub fn speed(&self) -> f64 {
        (self.vx * self.vx + self.vy * self.vy + self.vz * self.vz).sqrt()
    }

    /// Velocity in km/s (1 pc/yr ≈ 977,780 km/s... but the stored values are
    /// already tiny fractions of pc/yr, so we use the standard conversion).
    pub fn speed_kms(&self) -> f64 {
        self.speed() / 1.022_7e-6
    }

    /// Distance from origin (Sun) in parsecs.
    pub fn dist_pc(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Position as `[x, y, z]` for KD-tree indexing.
    pub fn pos(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// Velocity as `[vx, vy, vz]` for velocity-space indexing.
    pub fn vel_kms(&self) -> [f64; 3] {
        let s = 1.0 / 1.022_7e-6; // pc/yr → km/s
        [self.vx * s, self.vy * s, self.vz * s]
    }
}

/// Load stars from a `.bin` (STR2) or `.cbin` (STR3) file.
pub fn load_stars(path: &Path) -> Result<Vec<StarPoint>> {
    let data = std::fs::read(path)?;

    if data.len() < 8 {
        bail!("File too small ({} bytes)", data.len());
    }

    let magic = &data[0..4];
    match magic {
        b"STR3" => load_str3(&data),
        b"STR2" => load_str2(&data),
        _ => bail!(
            "Unknown format (magic: {:?}). Expected STR2 or STR3.",
            std::str::from_utf8(magic).unwrap_or("???")
        ),
    }
}

/// STR2: 8-byte header + 40 bytes/star (10 x f32).
fn load_str2(data: &[u8]) -> Result<Vec<StarPoint>> {
    let count = u32::from_le_bytes(data[4..8].try_into()?) as usize;
    let expected = 8 + count * 40;
    if data.len() < expected {
        bail!("STR2: expected {expected} bytes for {count} stars, got {}", data.len());
    }

    let mut stars = Vec::with_capacity(count);
    for i in 0..count {
        let off = 8 + i * 40;
        let f = |j: usize| -> f32 {
            f32::from_le_bytes(data[off + j * 4..off + j * 4 + 4].try_into().unwrap())
        };
        stars.push(StarPoint {
            x: f(0) as f64,
            y: f(1) as f64,
            z: f(2) as f64,
            vx: f(3) as f64,
            vy: f(4) as f64,
            vz: f(5) as f64,
            temp: f(6),
            mag: f(7),
            // f(8) = bp_rp, f(9) = dist — not needed for analysis
        });
    }
    Ok(stars)
}

/// STR3: 16-byte header + 16 bytes/star (quantized i16/u16).
fn load_str3(data: &[u8]) -> Result<Vec<StarPoint>> {
    if data.len() < 16 {
        bail!("STR3: header too short");
    }
    let count = u32::from_le_bytes(data[4..8].try_into()?) as usize;
    let pos_scale = f32::from_le_bytes(data[8..12].try_into()?) as f64;
    let vel_scale = f32::from_le_bytes(data[12..16].try_into()?) as f64;

    let expected = 16 + count * 16;
    if data.len() < expected {
        bail!("STR3: expected {expected} bytes for {count} stars, got {}", data.len());
    }

    let mut stars = Vec::with_capacity(count);
    for i in 0..count {
        let off = 16 + i * 16;
        let i16_at = |j: usize| -> f64 {
            let v = i16::from_le_bytes(data[off + j * 2..off + j * 2 + 2].try_into().unwrap());
            v as f64 / 32767.0
        };

        stars.push(StarPoint {
            x: i16_at(0) * pos_scale,
            y: i16_at(1) * pos_scale,
            z: i16_at(2) * pos_scale,
            vx: i16_at(3) * vel_scale,
            vy: i16_at(4) * vel_scale,
            vz: i16_at(5) * vel_scale,
            temp: u16::from_le_bytes(data[off + 12..off + 14].try_into().unwrap()) as f32,
            mag: i16::from_le_bytes(data[off + 14..off + 16].try_into().unwrap()) as f32 / 100.0,
        });
    }
    Ok(stars)
}
