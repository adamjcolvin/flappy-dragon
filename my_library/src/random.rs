use rand::{Rng, SeedableRng, distributions::uniform::{SampleRange, SampleUniform}};

#[cfg(all(not(feature = "pcg"), not(feature = "xorshift")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

/// `RandomNumberGenerator` holds random number generation state, and offers 
/// random number generation services to your program.
/// 
/// `RandomNumberGenerator` defaults to using the 
/// [PCG](https://crates.io/crates/rand_pcg)(1)
/// algorithm. You can specify `xorshift` as a feature flag to use it 
/// instead.
/// 
/// By default, `RandomNumberGenerator` requires mutability---it 
/// is shared in Bevy with `ResMut<RandomNumberGenerator>`. If 
/// you prefer interior mutability (and wish to use 
/// `Res<RandomNumberGenerator>` instead), specify the `locking`
/// feature flag.
/// 
/// ## Example
/// 
/// (2)
/// ```
/// use my_library::RandomNumberGenerator;
/// let mut my_rng = RandomNumberGenerator::new();
/// let random_number = my_rng.range(1..10);
/// println!("{random_number}");
/// ```
#[derive(bevy::prelude::Resource)]
pub struct RandomNumberGenerator {
  rng: RngCore,
}

impl Default for RandomNumberGenerator {
  fn default() -> Self {
    Self::new()
  }
}

impl RandomNumberGenerator {
  /// Creates a default `RandomNumberGenerator`, with a randomly 
  /// selected starting seed.
  pub fn new() -> Self {
    Self {
      rng: RngCore::from_entropy(),
    }
  }

  /// Creates a new `RandomNumberGenerator`, with a user-specified random seed.
  /// It will produce the same results each time (given the same requests).
  /// 
  /// # Arguments(3)
  /// 
  /// * `seed` - the random seed to use.
  /// 
  /// # Example(4)
  /// 
  /// ```
  /// use my_library::RandomNumberGenerator;
  /// let mut rng1 = RandomNumberGenerator::seeded(1);
  /// let mut rng2 = RandomNumberGenerator::seeded(1);
  /// let results: (u32, u32) = ( rng1.next(), rng2.next() );
  /// assert_eq!(results.0, results.1);
  /// ```
  pub fn seeded(seed: u64) -> Self {
    Self {
      rng: RngCore::seed_from_u64(seed),
    }
  }

  /// Generates a new random number of the requested type.
  pub fn next<T>(&mut self) -> T
  where rand::distributions::Standard: rand::prelude::Distribution<T>
  {
    self.rng.gen()
  }

  /// Generates a random number within the specified range.
  /// 
  /// # Arguments
  /// 
  /// * `range` - the range (inclusive or exclusive) within which to 
  /// generate a random number.
  /// 
  /// # Example
  /// 
  /// ```
  /// use my_library::RandomNumberGenerator;
  /// let mut rng = RandomNumberGenerator::new();
  /// let one_to_nine = rng.range(1..10);
  /// let one_to_ten = rng.range(1..=10);
  /// ```
  pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
  where
    T: SampleUniform + PartialOrd,
  {
    self.rng.gen_range(range)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_range_bounds() {
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..1000 {
      let n = rng.range(1..10);
      assert!(n >= 1);
      assert!(n < 10);
    }
  }

  #[test]
  fn test_inclusive_range_bounds() {
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..1000 {
      let n = rng.range(1..10);
      assert!(n >= 1);
      assert!(n <= 10);
    }
  }

  #[test]
  fn test_reproducibility() {
    let mut rng = (
      RandomNumberGenerator::seeded(1),
      RandomNumberGenerator::seeded(1),
    );
    (0..1000).for_each(|_| {
      assert_eq!(
        rng.0.range(u32::MIN..u32::MAX),
        rng.1.range(u32::MIN..u32::MAX),
      );
    });
  }

  #[test]
  fn test_next_types() {
    let mut rng = RandomNumberGenerator::new();
    let _ : i32 = rng.next();
    let _ = rng.next::<f32>();
  }

  #[test]
  fn test_float() {
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..1000 {
      // fun bug: rng.range(f32::MIN .. f32::MAX)
      // Crashes with a "range overflow" error.
      let n = rng.range(-3.40282347e+37_f32..3.40282347e+37_f32);
      assert!(n.is_finite());
      assert!(!n.is_infinite());
      assert!(!n.is_nan());
    }
  }
}

/// `Random` is a Bevy plugin that inserts a `RandomNumberGenerator` 
/// resource into your application.
/// 
/// Once you add the plugin (with `App::new().add_plugin(Random)`),
/// you can access a random number generator in systems with
/// `rng: ResMut<RandomNumberGenerator>`.
pub struct Random;

impl bevy::prelude::Plugin for Random {
  fn build(&self, app: &mut bevy::prelude::App) {
      app.insert_resource(RandomNumberGenerator::new());
  }
}