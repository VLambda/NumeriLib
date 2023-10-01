use numerilib::stats::Generic;

const NUMBERS: [f64; 5] = [3.0, 5.0, 8.0, 12.0, 15.0];

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn mean_test() {
        let mean = Generic::mean(&NUMBERS);
        assert_approx_eq!(8.6, mean);
    }

    #[test]
    pub fn sum_test() {
        let sum = Generic::sum(&NUMBERS);
        assert_approx_eq!(43.0, sum);
    }

    #[test]
    pub fn sum_sqaured_test() {
        let sum_squared = Generic::sum_squared(&NUMBERS);
        assert_approx_eq!(467_f64, sum_squared);
    }

    #[test]
    pub fn size_test() {
        let size = Generic::size(&NUMBERS);
        assert_eq!(5, size);
    }

    #[test]
    pub fn population_variance_test() {
        let population_variance = Generic::population_variance(&NUMBERS);
        assert_approx_eq!(19.44, population_variance);
    }

    #[test]
    pub fn sample_variance_test() {
        let sample_variance = Generic::sample_variance(&NUMBERS);
        assert_approx_eq!(24.3, sample_variance);
    }

    #[test]
    pub fn population_standard_deviation_test() {
        let population_standard_deviation = Generic::population_sd(&NUMBERS);
        assert_approx_eq!(4.409081537009721, population_standard_deviation);
    }

    #[test]
    pub fn sample_standard_deviation_test() {
        let sample_standard_deviation = Generic::sample_sd(&NUMBERS);
        assert_approx_eq!(4.929503017546495, sample_standard_deviation);
    }

    #[test]
    pub fn five_number_summary_test() {
        let five_number_summary = Generic::five_number_summary(&NUMBERS);
        assert_approx_eq!(3_f64, five_number_summary[0]);
        assert_approx_eq!(4_f64, five_number_summary[1]);
        assert_approx_eq!(8_f64, five_number_summary[2]);
        assert_approx_eq!(13.5, five_number_summary[3]);
        assert_approx_eq!(15_f64, five_number_summary[4]);
    }
}
