use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_sd_test1() {
        let d1 = 4.0;
        let d2 = 8.0;

        let sd = Fisher::sd(d1, d2);

        assert_approx_eq!(1.4907119849998598, sd);
    }

    #[test]
    pub fn fisher_sd_test2() {
        let d1 = 8.0;
        let d2 = 8.0;

        let sd = Fisher::sd(d1, d2);

        assert_approx_eq!(1.247219128924647, sd);
    }

    #[test]
    pub fn fisher_sd_test3() {
        let d1 = 7.0;
        let d2 = 10.0;

        let sd = Fisher::sd(d1, d2);

        assert_approx_eq!(1.0564428184106458, sd);
    }

    #[test]
    pub fn fisher_sd_test4() {
        let d1 = 16.0;
        let d2 = 9.0;

        let sd = Fisher::sd(d1, d2);

        assert_approx_eq!(0.9749411285209136, sd);
    }
}
