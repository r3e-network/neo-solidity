//! Bounds Checking Module
//!
//! Runtime bounds checking for arrays and integers.

/// Bounds check configuration
#[derive(Debug, Clone)]
pub struct BoundsConfig {
    pub check_array_access: bool,
    pub check_integer_overflow: bool,
    pub check_division_by_zero: bool,
}

impl Default for BoundsConfig {
    fn default() -> Self {
        Self {
            check_array_access: true,
            check_integer_overflow: true,
            check_division_by_zero: true,
        }
    }
}

/// Integer overflow check result
#[derive(Debug, Clone, Copy)]
pub enum OverflowCheck {
    Safe,
    MayOverflow,
    WillOverflow,
}

/// Bounds checker
pub struct BoundsChecker {
    config: BoundsConfig,
}

impl BoundsChecker {
    pub fn new(config: BoundsConfig) -> Self {
        Self { config }
    }

    pub fn check_add_overflow(&self, a: i128, b: i128, bits: u32) -> OverflowCheck {
        if !self.config.check_integer_overflow {
            return OverflowCheck::Safe;
        }
        let max = (1i128 << (bits - 1)) - 1;
        let min = -(1i128 << (bits - 1));
        match a.checked_add(b) {
            Some(r) if r >= min && r <= max => OverflowCheck::Safe,
            Some(_) => OverflowCheck::WillOverflow,
            None => OverflowCheck::WillOverflow,
        }
    }
}

impl Default for BoundsChecker {
    fn default() -> Self {
        Self::new(BoundsConfig::default())
    }
}
