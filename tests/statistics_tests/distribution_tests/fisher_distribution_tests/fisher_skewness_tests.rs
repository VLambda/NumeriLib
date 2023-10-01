use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_skewness_test1() {
        let d1 = 4.0;
        let d2 = 8.0;

        let skewness = Fisher::skewness(d1, d2);

        assert_approx_eq!(6.260990336999411, skewness);
    }

    #[test]
    pub fn fisher_skewness_test2() {
        let d1 = 8.0;
        let d2 = 10.0;

        let skewness = Fisher::skewness(d1, d2);

        assert_approx_eq!(3.674234614174767, skewness);
    }

    #[test]
    pub fn fisher_skewness_test3() {
        let d1 = 7.0;
        let d2 = 10.0;

        let skewness = Fisher::skewness(d1, d2);

        assert_approx_eq!(3.718678720805473, skewness);
    }

    #[test]
    pub fn fisher_skewness_test4() {
        let d1 = 16.0;
        let d2 = 17.0;

        let skewness = Fisher::skewness(d1, d2);

        assert_approx_eq!(1.9565051895283998, skewness);
    }
}
