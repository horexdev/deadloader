//! Safety limits for one archive (decision 127), enforced on bytes as they are unpacked.
//!
//! Sizes declared in archive headers are written by whoever made the file and can lie, so nothing here trusts
//! them: the [`Meter`] is asked before every chunk is written and refuses the chunk that would cross a limit.

use crate::error::LimitExceeded;

/// Configurable in Advanced settings; [`Limits::default`] is decision 127.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    /// Absolute cap on bytes unpacked from one archive.
    pub max_total_bytes: u64,
    /// The cap is also `max_total_factor ×` the archive's size, whichever is smaller.
    pub max_total_factor: u64,
    /// More `.vpk` files than this is a pack, not a mod.
    pub max_vpk_entries: usize,
    /// An entry may not unpack to this many times its compressed size or more (real VPKs ≈ 2–3 : 1).
    pub max_entry_ratio: u64,
    /// The ratio is not checked until an entry has unpacked this much: tiny files compress oddly and a
    /// bomb this small is harmless.
    pub ratio_floor_bytes: u64,
    /// The largest 7z decoding dictionary allowed; the decoder allocates it up front.
    pub max_dictionary_bytes: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_total_bytes: 2 << 30,
            max_total_factor: 20,
            max_vpk_entries: 256,
            max_entry_ratio: 100,
            ratio_floor_bytes: 1 << 20,
            // 7-Zip's "Ultra" preset uses 64 MB; larger needs a manual choice.
            max_dictionary_bytes: 256 << 20,
        }
    }
}

impl Limits {
    /// The total cap for an archive of `archive_size` bytes.
    pub fn total_for(&self, archive_size: u64) -> u64 {
        self.max_total_bytes
            .min(self.max_total_factor.saturating_mul(archive_size))
    }
}

/// Counts unpacked bytes for the whole archive.
#[derive(Debug)]
pub(crate) struct Meter {
    total_limit: u64,
    ratio: u64,
    ratio_floor: u64,
    total: u64,
}

/// What the ratio is measured on: one zip entry, or one 7z block (7z compresses blocks, not files).
#[derive(Debug)]
pub(crate) struct Unit {
    compressed: u64,
    unpacked: u64,
}

impl Unit {
    pub(crate) fn new(compressed: u64) -> Self {
        Self {
            compressed,
            unpacked: 0,
        }
    }
}

impl Meter {
    pub(crate) fn new(limits: &Limits, archive_size: u64) -> Self {
        Self {
            total_limit: limits.total_for(archive_size),
            ratio: limits.max_entry_ratio,
            ratio_floor: limits.ratio_floor_bytes,
            total: 0,
        }
    }

    /// Accounts for `n` more unpacked bytes of `entry`, or refuses them; refused bytes must not be written.
    pub(crate) fn admit(
        &mut self,
        unit: &mut Unit,
        n: u64,
        entry: &str,
    ) -> Result<(), LimitExceeded> {
        let total = self.total.saturating_add(n);
        if total > self.total_limit {
            return Err(LimitExceeded::TotalSize {
                limit: self.total_limit,
                entry: entry.to_owned(),
            });
        }
        let unpacked = unit.unpacked.saturating_add(n);
        if unpacked > self.ratio_floor && unpacked >= self.ratio.saturating_mul(unit.compressed) {
            return Err(LimitExceeded::Ratio {
                limit: self.ratio,
                entry: entry.to_owned(),
            });
        }
        self.total = total;
        unit.unpacked = unpacked;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn total(&self) -> u64 {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits(total: u64, factor: u64, ratio: u64, floor: u64) -> Limits {
        Limits {
            max_total_bytes: total,
            max_total_factor: factor,
            max_entry_ratio: ratio,
            ratio_floor_bytes: floor,
            ..Limits::default()
        }
    }

    #[test]
    fn defaults_are_decision_127() {
        let l = Limits::default();
        assert_eq!(l.max_total_bytes, 2 * 1024 * 1024 * 1024);
        assert_eq!(l.max_total_factor, 20);
        assert_eq!(l.max_vpk_entries, 256);
        assert_eq!(l.max_entry_ratio, 100);
    }

    #[test]
    fn the_total_cap_is_the_smaller_of_absolute_and_relative() {
        let l = Limits::default();
        assert_eq!(l.total_for(1_000), 20_000);
        assert_eq!(l.total_for(370 << 20), 2 << 30);
        assert_eq!(l.total_for(u64::MAX), 2 << 30, "no overflow");
    }

    #[test]
    fn bytes_up_to_the_total_are_admitted_and_one_more_is_refused() {
        let mut m = Meter::new(&limits(100, 1000, 1000, 0), 1);
        let mut unit = Unit::new(1_000_000);
        m.admit(&mut unit, 60, "a.vpk").expect("within");
        m.admit(&mut unit, 40, "a.vpk")
            .expect("exactly at the limit");
        let err = m.admit(&mut unit, 1, "b.vpk").expect_err("over");
        assert_eq!(
            err,
            LimitExceeded::TotalSize {
                limit: 100,
                entry: "b.vpk".into()
            }
        );
        assert_eq!(m.total(), 100, "refused bytes are not counted");
    }

    #[test]
    fn the_ratio_is_checked_per_unit_once_past_the_floor() {
        let mut m = Meter::new(&limits(u64::MAX, u64::MAX, 100, 1000), 1);
        let mut small = Unit::new(1);
        m.admit(&mut small, 1000, "tiny.vpk")
            .expect("below the floor any ratio is fine");

        let mut unit = Unit::new(100);
        m.admit(&mut unit, 9_999, "a.vpk").expect("99.99 : 1");
        let err = m.admit(&mut unit, 1, "a.vpk").expect_err("100 : 1");
        assert_eq!(
            err,
            LimitExceeded::Ratio {
                limit: 100,
                entry: "a.vpk".into()
            }
        );

        let mut other = Unit::new(100);
        m.admit(&mut other, 5_000, "b.vpk")
            .expect("a new unit starts from zero");
    }

    #[test]
    fn a_unit_with_no_compressed_bytes_cannot_unpack_past_the_floor() {
        let mut m = Meter::new(&limits(u64::MAX, u64::MAX, 100, 10), 1);
        let mut unit = Unit::new(0);
        m.admit(&mut unit, 10, "a.vpk").expect("floor");
        assert!(m.admit(&mut unit, 1, "a.vpk").is_err());
    }
}
