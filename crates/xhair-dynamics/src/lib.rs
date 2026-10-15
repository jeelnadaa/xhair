//! Dynamic reticle pure state simulation and spread curves for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// Maximum simulation tick rate in Hz.
pub const MAX_SIMULATION_HZ: u32 = 120;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_rate() {
        assert_eq!(MAX_SIMULATION_HZ, 120);
    }
}
