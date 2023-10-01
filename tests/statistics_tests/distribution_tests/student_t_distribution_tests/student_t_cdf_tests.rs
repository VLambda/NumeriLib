use numerilib::stats::distr::Student;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn student_t_cdf_test1() {
        let bound = -10_f64;
        let df = 7_f64;

        let cdf = Student::cdf(bound, df);

        assert_approx_eq!(1.0697101445421829e-5, cdf);
    }

    #[test]
    fn student_t_cdf_test2() {
        let bound = 13_f64;
        let df = 8_f64;

        let cdf = Student::cdf(bound, df);

        assert_approx_eq!(0.9999994188321667, cdf);
    }

    #[test]
    fn student_t_cdf_test3() {
        let bound = 0_f64;
        let df = 9_f64;

        let cdf = Student::cdf(bound, df);

        assert_approx_eq!(0_f64, cdf);
    }

    #[test]
    fn student_t_cdf_test4() {
        let bound = 8_f64;
        let df = 63_f64;

        let cdf = Student::cdf(bound, df);

        assert_approx_eq!(0.9999999998130576, cdf);
    }
}
