use super::*;

#[test]
fn positive_sequence() {
    let v = vec![1, 2, 3, 4];

    assert_eq!(max_subsequence_sum(v), 10);
}

#[test]
fn one_negative_sequence() {
    let v = [1, 2, 3, -100, 1, 6];

    assert_eq!(max_subsequence_sum(v), 7);
}

#[test]
fn negative_sequence() {
    let v = [-2, -2, -1, -3, -4];

    assert_eq!(max_subsequence_sum(v), -1);
}

#[test]
#[should_panic]
fn empty_sequence() {
    let _: i32 = max_subsequence_sum([]);
}

#[test]
fn single_member_sequence() {
    let v = [1];

    assert_eq!(max_subsequence_sum(v), 1);
}

#[test]
fn sum_test() {
    let v = [1, 2];
    let sum: i32 = v.iter().sum();

    assert_eq!(sum, 3);
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
