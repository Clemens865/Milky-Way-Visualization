//! OEIS — Online Encyclopedia of Integer Sequences.
//! No auth required. JSON search API.
//! https://oeis.org/

use crate::client::Explorer;
use anyhow::Result;
use serde::Deserialize;

const BASE: &str = "https://oeis.org";

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub count: Option<u32>,
    pub results: Option<Vec<Sequence>>,
}

#[derive(Debug, Deserialize)]
pub struct Sequence {
    pub number: u64,        // A-number (without the A)
    pub name: String,
    pub data: String,       // comma-separated values
    pub keyword: String,    // keywords like "nonn,easy,nice"
    pub author: Option<String>,
    pub formula: Option<Vec<String>>,
    pub comment: Option<Vec<String>>,
    pub xref: Option<Vec<String>>, // cross-references to other sequences
}

impl Sequence {
    /// Parse the comma-separated data string into integers.
    pub fn values(&self) -> Vec<i64> {
        self.data
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    }

    /// A-number string (e.g., "A000045" for Fibonacci).
    pub fn a_number(&self) -> String {
        format!("A{:06}", self.number)
    }
}

/// Parse OEIS response — it returns a JSON array of sequences directly.
fn parse_oeis_response(body: &str) -> Result<SearchResult> {
    // OEIS returns a bare JSON array of sequence objects
    let sequences: Vec<Sequence> = serde_json::from_str(body)?;
    let count = sequences.len() as u32;
    Ok(SearchResult {
        count: Some(count),
        results: if sequences.is_empty() { None } else { Some(sequences) },
    })
}

/// Search sequences by text query.
pub async fn search(explorer: &Explorer, query: &str, limit: u32) -> Result<SearchResult> {
    let encoded = query.replace(' ', "+");
    let url = format!("{BASE}/search?q={encoded}&fmt=json&start=0&count={limit}");
    let body = explorer.fetch(&url).await?;
    parse_oeis_response(&body)
}

/// Fetch a specific sequence by A-number (e.g., 45 for A000045).
pub async fn sequence(explorer: &Explorer, number: u64) -> Result<SearchResult> {
    let url = format!("{BASE}/search?q=id:A{number:06}&fmt=json");
    let body = explorer.fetch(&url).await?;
    parse_oeis_response(&body)
}

/// Search by the first few values of a sequence.
pub async fn search_by_values(explorer: &Explorer, values: &[i64]) -> Result<SearchResult> {
    let query: Vec<String> = values.iter().map(|v| v.to_string()).collect();
    let encoded = query.join(",");
    let url = format!("{BASE}/search?q={encoded}&fmt=json");
    let body = explorer.fetch(&url).await?;
    parse_oeis_response(&body)
}

pub fn print_summary(result: &SearchResult) {
    use colored::Colorize;

    let count = result.count.unwrap_or(0);
    println!("\n{} — {} matches\n", "OEIS Search".bold(), count);

    if let Some(ref results) = result.results {
        for seq in results {
            let values = seq.values();
            let preview: Vec<String> = values.iter().take(10).map(|v| v.to_string()).collect();

            println!("  {} {}", seq.a_number().cyan().bold(), seq.name);
            println!("    Values: {}", preview.join(", "));
            if !seq.keyword.is_empty() {
                println!("    Keywords: {}", seq.keyword.dimmed());
            }
            if let Some(ref xrefs) = seq.xref {
                if !xrefs.is_empty() {
                    let first_refs: Vec<&str> = xrefs.iter().take(3).map(|s| s.as_str()).collect();
                    println!("    Refs: {}", first_refs.join("; ").dimmed());
                }
            }
            println!();
        }
    }
}
