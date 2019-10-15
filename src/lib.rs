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
use std::thread;

use rand;
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
    pub thread_num: usize,
    pub samples: usize,
    pub sample_size: usize,
}

#[derive(Debug, PartialEq)]
struct Average {
    average: f64,
    elements_count: usize,
}

impl Average {
    fn merge(self, other: Average) -> Average {
        assert!(
            self.elements_count != 0 || other.elements_count != 0,
            "Both averages have elements_count equal to zero"
        );
        let new_average = (self.average * self.elements_count as f64
            + other.average * other.elements_count as f64)
            / (self.elements_count + other.elements_count) as f64;
        Average {
            average: new_average,
            elements_count: self.elements_count + other.elements_count,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut handles = Vec::new();

    for _ in 0..config.thread_num {
        let samples = config.samples;
        let sample_size = config.sample_size;
        handles.push(thread::spawn(move || {
            let rng = rand::thread_rng();
            let mut average = Average {
                average: 0.0,
                elements_count: 0,
            };
            for _ in 0..samples {
                let sample: Vec<f64> = rng
                    .sample_iter(Uniform::new_inclusive(-1.0, 1.0))
                    .take(sample_size)
                    .collect();
                let sum = max_subsequence_sum(&sample);
                average.elements_count += 1;
                average.average += (sum - average.average) / average.elements_count as f64;
            }
            average
        }));
    }

    let mut average = Average {
        average: 0.0,
        elements_count: 0,
    };

    for handle in handles {
        average = average.merge(handle.join().unwrap());
    }

    println!("{}", average.average);
    Ok(())
}
