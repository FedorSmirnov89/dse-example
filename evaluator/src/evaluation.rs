use crate::types::{Configuration, EvaluationResult};

pub(crate) fn evaluate_config(config: Configuration) -> EvaluationResult {
    let cost = evaluate_cost(&config);
    let runtime = evalute_runtime(&config);
    EvaluationResult {
        cost,
        runtime_s: runtime,
    }
}

// Dummy function for "evaluating" the cost of the given configuration
fn evaluate_cost(config: &Configuration) -> f64 {
    let mut cost = match config.core_number {
        crate::types::CoreNumber::Two => 50.0,
        crate::types::CoreNumber::Four => 100.0,
        crate::types::CoreNumber::Eight => 180.0,
        crate::types::CoreNumber::Sixteen => 350.0,
    };

    cost += match config.memory_gb {
        crate::types::MemoryGb::Four => 30.0,
        crate::types::MemoryGb::Eight => 50.0,
        crate::types::MemoryGb::Sixteen => 90.0,
        crate::types::MemoryGb::ThirtyTwo => 160.0,
        crate::types::MemoryGb::SixtyFour => 300.0,
    };

    cost += match config.disk_type {
        crate::types::DiskType::HDD => 20.0,
        crate::types::DiskType::SSD => 60.0,
        crate::types::DiskType::NVMe => 120.0,
    };

    cost
}

// Dummy function for evaluating cost (just sth to get a nice pareto front - this is not a "sane" eval function in any way)
// (literally just asked ChatGpt to give me sth that will result in an interesting pareto front)
fn evalute_runtime(config: &Configuration) -> f64 {
    let mut core_factor: f64 = match config.core_number {
        crate::types::CoreNumber::Two => 2.0,
        crate::types::CoreNumber::Four => 4.0,
        crate::types::CoreNumber::Eight => 8.0,
        crate::types::CoreNumber::Sixteen => 16.0,
    };
    core_factor = core_factor.ln();

    let ram_factor = match config.memory_gb {
        crate::types::MemoryGb::Four => 0.25,
        crate::types::MemoryGb::Eight => 0.5,
        crate::types::MemoryGb::Sixteen => 1.0,
        crate::types::MemoryGb::ThirtyTwo => 1.5,
        crate::types::MemoryGb::SixtyFour => 1.86,
    };

    let disk_factor = match config.disk_type {
        crate::types::DiskType::HDD => 0.8,
        crate::types::DiskType::SSD => 1.0,
        crate::types::DiskType::NVMe => 1.2,
    };

    1000.0 / (1.0 + core_factor + ram_factor * disk_factor)
}
