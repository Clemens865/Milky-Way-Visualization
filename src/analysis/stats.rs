//! Basic statistical utilities for exploring data.

/// Descriptive statistics for a slice of f64 values.
#[derive(Debug)]
pub struct Summary {
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub p25: f64,
    pub p75: f64,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "n={}, min={:.3}, p25={:.3}, median={:.3}, mean={:.3}, p75={:.3}, max={:.3}, std={:.3}",
            self.count, self.min, self.p25, self.median, self.mean, self.p75, self.max, self.std_dev
        )
    }
}

pub fn summarize(data: &[f64]) -> Option<Summary> {
    if data.is_empty() {
        return None;
    }

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let n = sorted.len();
    let sum: f64 = sorted.iter().sum();
    let mean = sum / n as f64;

    let variance: f64 = sorted.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;

    Some(Summary {
        count: n,
        min: sorted[0],
        max: sorted[n - 1],
        mean,
        median: percentile(&sorted, 50.0),
        std_dev: variance.sqrt(),
        p25: percentile(&sorted, 25.0),
        p75: percentile(&sorted, 75.0),
    })
}

fn percentile(sorted: &[f64], pct: f64) -> f64 {
    let idx = (pct / 100.0) * (sorted.len() - 1) as f64;
    let lower = idx.floor() as usize;
    let upper = idx.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let frac = idx - lower as f64;
        sorted[lower] * (1.0 - frac) + sorted[upper] * frac
    }
}

/// Find statistical outliers using IQR method.
/// Returns indices of outlier values.
pub fn outliers(data: &[f64], iqr_factor: f64) -> Vec<usize> {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let q1 = percentile(&sorted, 25.0);
    let q3 = percentile(&sorted, 75.0);
    let iqr = q3 - q1;
    let lower = q1 - iqr_factor * iqr;
    let upper = q3 + iqr_factor * iqr;

    data.iter()
        .enumerate()
        .filter(|(_, v)| **v < lower || **v > upper)
        .map(|(i, _)| i)
        .collect()
}
