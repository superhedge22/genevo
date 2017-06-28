
use genetic::AsScalar;
use rand::Rng;


pub fn random_index<R>(rng: &mut R, length: usize) -> usize
    where R: Rng + Sized {
    random_index_from_range(rng, 0, length)
}

pub fn random_index_from_range<R>(rng: &mut R, min: usize, max: usize) -> usize
    where R: Rng + Sized {
    rng.gen_range(min, max)
}

pub fn random_cut_points<R>(rng: &mut R, length: usize) -> (usize, usize)
    where R: Rng + Sized {
    random_cut_points_from_range(rng, 0, length)
}

pub fn random_cut_points_from_range<R>(rng: &mut R, min: usize, max: usize) -> (usize, usize)
    where R: Rng + Sized {
    assert!(max >= min + 4);
    let max_slice = max - min - 2;
    loop {
        let cutpoint1 = rng.gen_range(min, max);
        let cutpoint2 = rng.gen_range(min, max);
        if cutpoint1 < cutpoint2 {
            if cutpoint2 - cutpoint1 >= max_slice {
                continue;
            }
            return (cutpoint1, cutpoint2)
        } else if cutpoint2 < cutpoint1 {
            if cutpoint1 - cutpoint2 >= max_slice {
                continue;
            }
            return (cutpoint2, cutpoint1)
        }
    }
}

pub fn random_n_cut_points<R>(rng: &mut R, n: usize, length: usize) -> Vec<usize>
    where R: Rng + Sized {
    assert!(n > 0);
    assert!(length >= 2 * n);
    let mut cutpoints = Vec::with_capacity(n);
    match n {
        1 => {
            cutpoints.push(random_index(rng, length));
        },
        2 => {
            let (cp1, cp2) = random_cut_points(rng, length);
            cutpoints.push(cp1);
            cutpoints.push(cp2);
        },
        _ => {
            let slice_len = length / n;
            let mut start = 0;
            let mut end = slice_len;
            let mut count = 1;
            loop {
                let cutpoint = random_index_from_range(rng, start, end);
                if cutpoint == 0 || cutpoint == length {
                    continue;
                }
                cutpoints.push(cutpoint);
                count += 1;
                if count > n {
                    break;
                }
                start = cutpoint;
                if count == n {
                    end = length;
                } else {
                    end += slice_len;
                }
            }
        },
    }
    cutpoints
}

pub fn random_probability<R>(rng: &mut R) -> f64
    where R: Rng + Sized {
    rng.next_f64()
}

/// The `WeightedDistribution` is used to select values proportional to their
/// weighted values.
pub struct WeightedDistribution<'a, T> where T: 'a + AsScalar {
    values: &'a [T],
    sum: f64,
    weights: Vec<f64>,
}

impl<'a, T> WeightedDistribution<'a, T> where T: 'a + AsScalar {

    pub fn from_scalar_values(values: &'a [T]) -> Self {
        let (weights, weight_sum) = calc_weights_and_sum(values);
        WeightedDistribution {
            values: values,
            weights: weights,
            sum: weight_sum,
        }
    }

    pub fn sum(&self) -> &f64 {
        &self.sum
    }

    pub fn select(&self, pointer: f64) -> usize {
        weighted_select(pointer, &self.weights)
    }

    pub fn value(&self, index: usize) -> &T {
        &self.values[index]
    }

}

/// Calculates weights and the sum for the given values.
fn calc_weights_and_sum<'a, T>(values: &'a [T]) -> (Vec<f64>, f64)
    where T: 'a + AsScalar {
    let mut weights = Vec::with_capacity(values.len());
    let mut weight_sum: f64 = 0.;
    for i in 0..values.len() {
        weights.push(values[i].as_scalar());
        weight_sum += values[i].as_scalar();
    }
    (weights, weight_sum)
}

/// Selects one index proportional to their weights.
fn weighted_select(pointer: f64, weights: &[f64]) -> usize {
    let mut delta = pointer;
    for i in 0..weights.len() {
        delta -= weights[i];
        if delta <= 0. {
            return i;
        }
    }
    // when rounding errors occur, we return the last item's index
    return weights.len() - 1;
}


#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;
    use rand::thread_rng;

    #[test]
    #[should_panic(expected = "assertion failed: max >= min + 4")]
    fn random_cut_points_from_range_0_to_3() {
        random_cut_points_from_range(&mut thread_rng(), 0, 3);
    }

    #[test]
    #[should_panic(expected = "assertion failed: max >= min + 4")]
    fn random_cut_points_from_range_4_to_4() {
        random_cut_points_from_range(&mut thread_rng(), 4, 4);
    }

    quickcheck! {

        fn in_random_cut_points_from_range_cutpoint1_is_smaller_than_cutpoint2(
            min: usize, max: usize) -> TestResult {
            if max < min + 4 { return TestResult::discard() }

            let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

            if cutpoint1 < cutpoint2 {
                TestResult::passed()
            } else {
                TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
            }
        }

        fn in_random_cut_points_from_range_delta_of_cutpoint1_and_cutpoint2_is_smaller_than_range_minus_2(
            min: usize, max: usize) -> TestResult {
            if max < min + 4 { return TestResult::discard() }

            let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

            if cutpoint2 - cutpoint1 < max - min - 2 {
                TestResult::passed()
            } else {
                TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
            }
        }

        fn in_random_cut_points_from_range_cutpoint1_is_not_smaller_than_min_of_range(
            min: usize, max: usize) -> TestResult {
            if max < min + 4 { return TestResult::discard() }

            let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

            if cutpoint1 >= min {
                TestResult::passed()
            } else {
                TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
            }
        }

        fn in_random_cut_points_from_range_cutpoint2_is_not_greater_than_max_of_range(
            min: usize, max: usize) -> TestResult {
            if max < min + 4 { return TestResult::discard() }

            let (cutpoint1, cutpoint2) = random_cut_points_from_range(&mut thread_rng(), min, max);

            if cutpoint2 <= max {
                TestResult::passed()
            } else {
                TestResult::error(format!("cut points: {}, {}", cutpoint1, cutpoint2))
            }
        }

    }

    #[test]
    fn weighted_distribution_select() {
        let mut rng = StdRng::from_seed(&[42usize]);

        let weights = vec![200, 150, 600, 50];
        let n_sum = 1_000.;

        let weighted_distribution = WeightedDistribution::from_scalar_values(&weights);

        let mut counter = vec![0, 0, 0, 0];
        for _ in 0..n_sum as usize {
            let random = rng.next_f64() * weighted_distribution.sum();
            let index = weighted_distribution.select(random);
            counter[index] += 1;
        }

        assert_that!(counter[0], is(equal_to(204)));
        assert_that!(counter[1], is(equal_to(152)));
        assert_that!(counter[2], is(equal_to(600)));
        assert_that!(counter[3], is(equal_to(44)));
    }

}
