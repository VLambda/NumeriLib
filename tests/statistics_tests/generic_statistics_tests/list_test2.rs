use numerilib::stats::Generic;

// Make similar test to the ones in list_test1.rs, but make the list twice as long.
const NUMBERS: [f64; 10] = [3.5, 2.1, 78.9, 2.0, 89.0, 173.5, 0.0, 9.7, 73.4, 3.33];

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn mean_test() {
        let mean = Generic::mean(&NUMBERS);
        assert_eq!(43.543, mean);
    }

    #[test]
    pub fn sum_test() {
        let sum = Generic::sum(&NUMBERS);
        assert_eq!(435.43, sum);
    }

    #[test]
    pub fn sum_sqaured_test() {
        let sum_squared = Generic::sum_squared(&NUMBERS);
        assert_eq!(49761.8589, sum_squared);
    }

    #[test]
    pub fn size_test() {
        let size = Generic::size(&NUMBERS);
        assert_eq!(10, size);
    }

    #[test]
    pub fn population_variance_test() {
        let population_variance = Generic::population_variance(&NUMBERS);
        assert_eq!(3080.193041, population_variance);
    }

    #[test]
    pub fn sample_variance_test() {
        let sample_variance = Generic::sample_variance(&NUMBERS);
        assert_eq!(3422.4367122222225, sample_variance);
    }

    #[test]
    pub fn population_standard_deviation_test() {
        let population_standard_deviation = Generic::population_sd(&NUMBERS);
        assert_eq!(55.49948685348361, population_standard_deviation);
    }

    #[test]
    pub fn sample_standard_deviation_test() {
        let sample_standard_deviation = Generic::sample_sd(&NUMBERS);
        assert_eq!(58.5015958091933, sample_standard_deviation);
    }

    #[test]
    pub fn five_number_summary_test() {
        let five_number_summary = Generic::five_number_summary(&NUMBERS);
        assert_eq!(0_f64, five_number_summary[0]);
        assert_eq!(2.1, five_number_summary[1]);
        assert_eq!(6.6, five_number_summary[2]);
        assert_eq!(78.9, five_number_summary[3]);
        assert_eq!(173.5, five_number_summary[4]);
    }
}
