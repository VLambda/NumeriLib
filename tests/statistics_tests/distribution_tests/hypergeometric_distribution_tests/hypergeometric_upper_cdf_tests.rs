use numerilib::stats::distr::Hypergeometric;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn hypergeometric_upper_cdf_test1() {
        let population = 15_f64;
        let success = 10_f64;
        let draws = 5_f64;
        let observed = 3_f64;

        let upper_cdf = Hypergeometric::ucdf(population, success, draws, observed);

        assert_approx_eq!(0.8458189848070801, upper_cdf);
    }

    #[test]
    fn hypergeometric_upper_cdf_test2() {
        let population = 70_f64;
        let success = 43_f64;
        let draws = 9_f64;
        let observed = 7_f64;

        let upper_cdf = Hypergeometric::ucdf(population, success, draws, observed);

        assert_approx_eq!(0.24396734652091165, upper_cdf);
    }

    #[test]
    fn hypergeometric_upper_cdf_test3() {
        let population = 101_f64;
        let success = 73_f64;
        let draws = 17_f64;
        let observed = 9_f64;

        let upper_cdf = Hypergeometric::ucdf(population, success, draws, observed);

        assert_approx_eq!(0.9854571119660213, upper_cdf);
    }

    #[test]
    fn hypergeometric_upper_cdf_test4() {
        let population = 13_f64;
        let success = 3_f64;
        let draws = 12_f64;
        let observed = 1_f64;

        let upper_cdf = Hypergeometric::ucdf(population, success, draws, observed);

        assert_approx_eq!(1_f64, upper_cdf);
    }
}
