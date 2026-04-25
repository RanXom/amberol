// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Default)]
pub struct LrcLine {
    pub time_ms: u64,
    pub text: String,
}

static LRC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[(\d{2,}):(\d{2}(?:\.\d+)?)\](.*)$").unwrap());

pub fn parse_lrc_file(path: &Path) -> Option<Vec<LrcLine>> {
    let content = fs::read_to_string(path).ok()?;
    
    let mut lines = Vec::new();
    for line in content.lines() {
        if let Some(captures) = LRC_REGEX.captures(line) {
            let mins: u64 = captures.get(1).unwrap().as_str().parse().unwrap_or(0);
            let secs: f64 = captures.get(2).unwrap().as_str().parse().unwrap_or(0.0);
            let text = captures.get(3).unwrap().as_str().trim().to_string();
            
            let time_ms = (mins * 60000) + (secs * 1000.0) as u64;
            lines.push(LrcLine { time_ms, text });
        }
    }
    
    // Sort by time to ensure it's sequential
    lines.sort_by_key(|l| l.time_ms);
    
    if lines.is_empty() {
        None
    } else {
        Some(lines)
    }
}

pub fn find_lrc_for_song(song_path: &Path) -> Option<PathBuf> {
    if let Some(parent) = song_path.parent() {
        if let Some(stem) = song_path.file_stem() {
            let mut lrc_path = parent.join(stem);
            lrc_path.set_extension("lrc");
            if lrc_path.exists() {
                return Some(lrc_path);
            }
        }
    }
    None
}
