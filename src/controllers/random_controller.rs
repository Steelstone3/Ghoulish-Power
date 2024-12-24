use std::ops::Range;

use rand::{rngs::StdRng, Rng, SeedableRng};

#[allow(dead_code)]
pub trait RandomGenerator {
    fn random_value_i32(seed: u64, range: Range<i32>) -> i32;
    #[allow(dead_code)]
    fn generate_seed() -> u64;
}

pub struct RandomController;

impl RandomGenerator for RandomController {
    fn random_value_i32(seed: u64, range: Range<i32>) -> i32 {
        let mut rng = StdRng::seed_from_u64(seed);

        rng.gen_range(range.start..range.end)
    }

    #[allow(dead_code)]
    fn generate_seed() -> u64 {
        let mut rng = rand::thread_rng();

        rng.gen_range(u64::MIN..u64::MAX)
    }
}

#[cfg(test)]
mod controller_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case( 0, 0..1000, 801)]
    #[case( 10, 0..1000, 386)]
    #[case( 100, 0..1000, 755)]
    fn generate_seeded_random_i32(
        #[case] seed: u64,
        #[case] range: Range<i32>,
        #[case] expected_random: i32,
    ) {
        // When
        let actual_random = RandomController::random_value_i32(seed, range);

        // Then
        assert_eq!(expected_random, actual_random);
    }
}
