use numerilib::stats::distr::Student;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn student_t_cdf_test1() {
        let lower = 10_f64;
        let upper = 13_f64;
        let df = 7_f64;

        let cdf = Student::tailcdf(lower, upper, df);

        assert_approx_eq!(8.8423622974787e-6, cdf);
    }

    #[test]
    fn student_t_cdf_test2() {
        let lower = -10_f64;
        let upper = 10_f64;
        let df = 8_f64;

        let cdf = Student::tailcdf(lower, upper, df);

        assert_approx_eq!(0.9999915118184723, cdf);
    }

    #[test]
    fn student_t_cdf_test3() {
        let lower = -1e99;
        let upper = 1_f64;
        let df = 4_f64;

        let cdf = Student::tailcdf(lower, upper, df);

        assert_approx_eq!(0.8130495168560655, cdf);
    }

    #[test]
    fn student_t_cdf_test4() {
        let lower = -1e99;
        let upper = 1e99;
        let df = 4_f64;

        let cdf = Student::tailcdf(lower, upper, df);

        assert_approx_eq!(1_f64, cdf);
    }
}
