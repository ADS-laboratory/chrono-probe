use time_complexity_plot::algorithms::Algorithm;

/// The algorithms to be tested
const ALGORITHMS: [Algorithm; 3] = [
    time_complexity_plot::algorithms::PERIOD_NAIVE1,
    time_complexity_plot::algorithms::PERIOD_NAIVE2,
    time_complexity_plot::algorithms::PERIOD_SMART,
];

/// Test the given period finding algorithms
/// 
/// # Arguments
/// 
/// * `input` - The string to be analyzed
/// * `expected` - The expected period
/// * `algorithms` - The algorithms to be tested
fn test_algorithms(input: &str, expected: usize, algorithms: &[Algorithm]) {
    for algorithm in algorithms {
        let actual = (algorithm.function)(input.as_bytes());
        assert_eq!(expected, actual);
    }
}

/// Test all the period finding algorithms
/// 
/// # Arguments
/// 
/// * `input` - The string to be analyzed
/// * `expected` - The expected period
fn test(input: &str, expected: usize) {
    test_algorithms(input, expected, &ALGORITHMS);
}

#[test]
fn test_1() {
    let input = "abcabcab";
    let expected = 3;
    test(input, expected);
}

#[test]
fn test_2() {
    let input = "aba";
    let expected = 2;
    test(input, expected);
}

#[test]
fn test_3() {
    let input = "abca";
    let expected = 3;
    test(input, expected);
}

// TODO: import VPL tests