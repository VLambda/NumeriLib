use numerilib::stats::distr::Hypergeometric;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn hypergeometric_sd_test1() {
        let population = 15_f64;
        let success = 10_f64;
        let draws = 5_f64;

        let sd = Hypergeometric::sd(population, success, draws);

        assert_approx_eq!(0.8908708063747479, sd);
    }

    #[test]
    fn hypergeometric_sd_test2() {
        let population = 70_f64;
        let success = 43_f64;
        let draws = 9_f64;

        let sd = Hypergeometric::sd(population, success, draws);

        assert_approx_eq!(1.3730289575205197, sd);
    }

    #[test]
    fn hypergeometric_sd_test3() {
        let population = 101_f64;
        let success = 73_f64;
        let draws = 17_f64;

        let sd = Hypergeometric::sd(population, success, draws);

        assert_approx_eq!(1.6915435195354867, sd);
    }

    #[test]
    fn hypergeometric_sd_test4() {
        let population = 13_f64;
        let success = 3_f64;
        let draws = 12_f64;

        let sd = Hypergeometric::sd(population, success, draws);

        assert_approx_eq!(0.4213250442347432, sd);
    }
}
