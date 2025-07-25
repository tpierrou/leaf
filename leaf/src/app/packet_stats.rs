use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use tracing::{debug};

use serde::{Deserialize, Serialize};

pub const STATS_FILE: &str = "/data/data/com.guardsolutions.guard/files/packet_stats.json";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PacketStats {
    // Map from hour timestamp (seconds since epoch, truncated to hour) -> count
    hourly_counts: BTreeMap<u64, u64>,
}

impl PacketStats {
    fn new() -> Self {
        PacketStats {
            hourly_counts: BTreeMap::new(),
        }
    }

    pub fn increment_current_hour(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let hour = now.as_secs() / 3600 * 3600; // truncate to the current hour
        *self.hourly_counts.entry(hour).or_insert(0) += 1;
        self.trim_to_last_24_hours();
    }

    fn trim_to_last_24_hours(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cutoff = now - 24 * 3600;
        self.hourly_counts.retain(|&ts, _| ts >= cutoff / 3600 * 3600);
    }

    pub fn save_atomically(&self, path: &str) -> std::io::Result<()> {
        let tmp_path = format!("{}.tmp", path);
        let json = serde_json::to_string(&self)?;
        let mut tmp_file = File::create(&tmp_path)?;
        tmp_file.write_all(json.as_bytes())?;
        tmp_file.sync_all()?; // ensure flushed
        fs::rename(&tmp_path, path)?; // atomic swap
        Ok(())
    }

    pub fn load_or_default(path: &str) -> Self {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(parsed) = serde_json::from_str::<PacketStats>(&data) {
                return parsed;
            }
        }
        PacketStats::new()
    }
}