use std::{collections::HashMap, hash::Hash, iter::Iterator};

// Finds values very late in patterns, assuming that they at some point repeat.
// Future work: instead of ignore_first, wait for pattern to repeat more than once. Panic after timeout.
pub fn calc_big_pattern_index<I, T>(
  mut state_and_values: I, // Iterator which generates T (representing the state to indicate a repitition) and i64, the value of the pattern
  goal_index: u64,         // Index of the value to calculate
  ignore_first: u64, // Sometimes patterns take a little while to settle in. This allows the user to specify how much to ignore before looking for the pattern.
) -> i64
where
  I: Iterator<Item = (T, i64)>,
  T: Hash,
  T: Eq,
{
  let mut seen: HashMap<T, (u64, i64)> = HashMap::new();
  let mut index_to_value: HashMap<u64, i64> = HashMap::new();
  let mut i = 0;
  loop {
    let (state, value) = state_and_values.next().unwrap();
    if i >= ignore_first && seen.contains_key(&state) {
      let (old_i, old_value) = seen[&state];
      let delta_i = i - old_i; // Period of the pattern
      let delta_value = value - old_value; // How much the value changes over that period
      let offset = goal_index % delta_i;
      // Find index that is the same mod delta_i as the goal_index, and between old_i and i:
      let mut base_index = old_i - old_i % delta_i + offset;
      if base_index < old_i {
        base_index += delta_i;
      }
      let base_value = index_to_value[&base_index];
      let rest_value = i64::try_from((goal_index - base_index) / delta_i).unwrap() * delta_value;
      return base_value + rest_value;
    } else {
      seen.insert(state, (i, value));
      index_to_value.insert(i, value);
    }
    i += 1;
  }
}
