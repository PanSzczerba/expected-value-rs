#[cfg(test)]
mod test;

use std::error::Error;
use std::thread;

use rand::{Rng, distributions::Uniform};
use std::ops::Add;
use std::convert::From;

fn max_subsequence_sum<T>(i: impl IntoIterator<Item=T>) -> T
where
    T: Copy,
    T: Add<Output = T> + PartialEq + PartialOrd,
    T: From<i32>,
{
    let mut it = i.into_iter();
    let mut max_sum = if let Some(e) = it.next() {
        e
    } else {
        panic!("Could not get max subsequence sum from empty slice");
    };

    let mut current_sum: T = max_sum;

    for val in it {
        if current_sum < T::from(0) {
            current_sum = val;
        } else {
            current_sum = current_sum + val;
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
                let sample = rng
                    .sample_iter(Uniform::new_inclusive(-1.0, 1.0))
                    .take(sample_size);
                let sum = max_subsequence_sum(sample);
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
