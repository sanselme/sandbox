// SPDX-License-Identifier: GPL-3.0
// https://rust-unofficial.github.io/patterns/idioms/ctor.html

/// Time in seconds.
///
/// # Example
/// ```
/// let s = Second::new(42);
/// assert_eq!(42, s.value());
/// ```
#[derive(Default)]
pub struct Second {
    value: u64,
}

impl Second {
    // Construcs a new instance of [`Second`].
    // note: this is an associated function - no self.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

// impl Default for Second {
//     fn default() -> Self {
//         Self { value: 0 }
//     }
// }
