use std::time::Duration;

use rand::rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

use crate::error::{error_response, Result};

pub(crate) fn read_json_input(config_json: ConfigJson) -> Result<Configuration> {
    let ConfigJson {
        core_number,
        memory_size_gb,
        disk_type,
    } = config_json;

    let config = Configuration {
        core_number: CoreNumber::new(core_number)?,
        disk_type: DiskType::new(disk_type)?,
        memory_gb: MemoryGb::new(memory_size_gb)?,
    };
    Ok(config)
}

/// Models the structure of the JSON we get as a request
#[derive(Debug, Deserialize)]
pub struct ConfigJson {
    core_number: u32,
    memory_size_gb: u32,
    disk_type: String, // e.g., "HDD", "SSD", "NVMe"
}

/// The type we actually use to model the configuration we evaluate
pub(crate) struct Configuration {
    pub(crate) core_number: CoreNumber,
    pub(crate) memory_gb: MemoryGb,
    pub(crate) disk_type: DiskType,
}

pub(crate) enum CoreNumber {
    Two,
    Four,
    Eight,
    Sixteen,
}

impl CoreNumber {
    fn new(core_num: u32) -> Result<Self> {
        let core_num = match core_num {
            2 => Self::Two,
            4 => Self::Four,
            8 => Self::Eight,
            16 => Self::Sixteen,
            _ => return Err(error_response("invalid core number")),
        };
        Ok(core_num)
    }
}

pub(crate) enum MemoryGb {
    Four,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

impl MemoryGb {
    fn new(memory_gb: u32) -> Result<Self> {
        let memory_gb = match memory_gb {
            4 => Self::Four,
            8 => Self::Eight,
            16 => Self::Sixteen,
            32 => Self::ThirtyTwo,
            64 => Self::SixtyFour,
            _ => return Err(error_response("invalid memory number")),
        };
        Ok(memory_gb)
    }
}

pub(crate) enum DiskType {
    HDD,
    SSD,
    NVMe,
}

impl DiskType {
    fn new(disk_type: String) -> Result<Self> {
        let disk_type = match disk_type.as_str() {
            "HDD" => Self::HDD,
            "SSD" => Self::SSD,
            "NVMe" => Self::NVMe,
            _ => return Err(error_response("invalid disk type")),
        };
        Ok(disk_type)
    }
}

/// Models the JSON structure we use as the response body
#[derive(Debug, Serialize)]
pub struct EvaluationResult {
    pub(crate) cost: f64,
    pub(crate) runtime_s: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelayConfiguration {
    mean_ms: f64,
    stddev_ms: f64,
}

impl DelayConfiguration {
    pub(crate) async fn simulate_delay(&self) -> Result<()> {
        let wait_time_ms = {
            let mut rand = rng();
            let normal = Normal::new(self.mean_ms, self.stddev_ms).map_err(|err| {
                let msg = format!("error creating normal distribution: {err}");
                error_response(msg)
            })?;
            let sample = normal.sample(&mut rand).max(0.0);
            sample as u64
        };
        tokio::time::sleep(Duration::from_millis(wait_time_ms)).await;
        Ok(())
    }
}
