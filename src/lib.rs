#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn positive_sequence() {
        let v = vec![1.0, 2.0, 3.0, 4.0];

        assert_eq!(max_subsequence_sum(&v[..]), 10.0);
    }

    #[test]
    fn one_negative_sequence() {
        let v = [1.0, 2.0, 3.0, -100.0, 1.0, 6.0];

        assert_eq!(max_subsequence_sum(&v), 7.0);
    }

    #[test]
    fn negative_sequence() {
        let v = [-1.0, -2.0, -0.5, -3.0, -1.0];

        assert_eq!(max_subsequence_sum(&v), -0.5);
    }

    #[test]
    #[should_panic]
    fn empty_sequence() {
        max_subsequence_sum(&[]);
    }

    #[test]
    fn single_member_sequence() {
        let v = [1.0];

        assert_eq!(max_subsequence_sum(&v), 1.0);
    }

    #[test]
    fn sum_test() {
        let v = [1.0, 2.0];
        let sum: f64 = v.iter().sum();

        assert_eq!(sum, 3.0);
    }

    #[test]
    fn equal_average_join() {
        let a1 = Average {
            average: 1.0,
            elements_count: 1,
        };

        let a2 = Average {
            average: 1.0,
            elements_count: 1,
        };

        assert_eq!(
            a1.merge(a2),
            Average {
                average: 1.0,
                elements_count: 2
            }
        );
    }
}

use std::error::Error;

extern crate rand;
use rand::distributions::Uniform;
use rand::Rng;

fn max_subsequence_sum(v: &[f64]) -> f64 {
    if v.len() < 1 {
        panic!("Could not get max subsequence sum from empty slice");
    }

    let mut max_sum = v[0];

    let mut current_sum = max_sum;

    for val in v[1..].iter() {
        if current_sum < 0.0 {
            current_sum = *val;
        } else {
            current_sum = current_sum + *val;
        }

        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}

#[derive(Debug)]
pub struct Config {
    pub samples: usize,
    pub sample_size: usize,
}

#[derive(Debug, PartialEq)]
struct Average {
    average: f64,
    elements_count: usize,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let rng = rand::thread_rng();
    let mut average = Average {
        average: 0.0,
        elements_count: 0,
    };

    for _ in 0..config.samples {
        let sample: Vec<f64> = rng
            .sample_iter(Uniform::new_inclusive(-1.0, 1.0))
            .take(config.sample_size)
            .collect();
        let sum = max_subsequence_sum(&sample);
        average.elements_count += 1;
        average.average += (sum - average.average) / average.elements_count as f64;
    }

    println!("{}", average.average);
    Ok(())
}
