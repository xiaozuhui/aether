#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecimalMode {
    /// Treat decimal-like values as fixed precision numbers.
    FixedPrecision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranspileOptions {
    /// Hard reject any `numpy` usage.
    pub reject_numpy: bool,
    /// Hard reject any filesystem/network usage.
    pub reject_io: bool,
    /// Hard reject any console IO (`print` / Aether `PRINT*` / `INPUT`).
    pub reject_console: bool,

    /// Decimal handling strategy.
    pub decimal_mode: DecimalMode,

    /// Internal calculation scale (e.g. 6).
    pub calc_scale: u32,
    /// Money output scale (e.g. 2 for cents).
    pub money_scale: u32,
}

impl Default for TranspileOptions {
    fn default() -> Self {
        Self {
            reject_numpy: true,
            reject_io: true,
            reject_console: true,
            decimal_mode: DecimalMode::FixedPrecision,
            calc_scale: 6,
            money_scale: 2,
        }
    }
}
