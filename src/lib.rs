//! widom: Monte Carlo adsorption simulation for gases in porous crystals.
//!
//! Nothing is implemented yet. See README.md for the plan.

/// Boltzmann constant in J/K (exact, SI 2019).
pub const BOLTZMANN_J_PER_K: f64 = 1.380_649e-23;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boltzmann_constant_is_si_exact() {
        assert_eq!(BOLTZMANN_J_PER_K, 1.380_649e-23);
    }
}
