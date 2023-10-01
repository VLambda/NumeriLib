use numerilib::stats::distr::Hypergeometric;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn hypergeometric_skewness_test1() {
        let population = 15_f64;
        let success = 10_f64;
        let draws = 5_f64;

        let skewness = Hypergeometric::skewness(population, success, draws);

        assert_approx_eq!(-0.1175019408963036, skewness);
    }

    #[test]
    fn hypergeometric_skewness_test2() {
        let population = 70_f64;
        let success = 43_f64;
        let draws = 9_f64;

        let skewness = Hypergeometric::skewness(population, success, draws);

        assert_approx_eq!(-0.09977503404417985, skewness);
    }

    #[test]
    fn hypergeometric_skewness_test3() {
        let population = 101_f64;
        let success = 73_f64;
        let draws = 17_f64;

        let skewness = Hypergeometric::skewness(population, success, draws);

        assert_approx_eq!(-0.15154733919045774, skewness);
    }

    #[test]
    fn hypergeometric_skewness_test4() {
        let population = 13_f64;
        let success = 3_f64;
        let draws = 12_f64;

        let skewness = Hypergeometric::skewness(population, success, draws);

        assert_approx_eq!(-0.6139406135149204, skewness);
    }
}
