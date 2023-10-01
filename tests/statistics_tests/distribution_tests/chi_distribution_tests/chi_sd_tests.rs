use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_sd_test1() {
        let df = 5.0;

        let sd = Chi::sd(df);

        assert_approx_eq!(0.687696199769981, sd);
    }

    #[test]
    fn chi_sd_test2() {
        let df = 7.0;

        let sd = Chi::sd(df);

        assert_approx_eq!(0.6935513902864908, sd);
    }

    #[test]
    fn chi_sd_test3() {
        let df = 9.0;

        let sd = Chi::sd(df);

        assert_approx_eq!(0.6967104335347779, sd);
    }

    #[test]
    fn chi_sd_test4() {
        let df = 11.0;

        let sd = Chi::sd(df);

        assert_approx_eq!(0.6986807923645283, sd);
    }
}
