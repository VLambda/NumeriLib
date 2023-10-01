use numerilib::stats::distr::Student;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_student_t_inverse_cdf() {
        let df = 6_f64;
        let area = 0.95_f64;

        let x = Student::inv(area, df);

        assert_approx_eq!(1.9431802803927817_f64, x);
    }

    #[test]
    fn test_student_t_inverse_cdf_2() {
        let df = 63_f64;
        let area = 0.05_f64;

        let x = Student::inv(area, df);

        assert_approx_eq!(-1.6657206563868145, x);
    }

    #[test]
    fn test_student_t_inverse_cdf_3() {
        let df = 1_f64;
        let area = 0.5_f64;

        let x = Student::inv(area, df);

        assert_approx_eq!(0_f64, x);
    }

    #[test]
    fn test_student_t_inverse_cdf_4() {
        let df = 30_f64;
        let area = 0.975_f64;

        let x = Student::inv(area, df);

        assert_approx_eq!(2.0422720735586672, x);
    }
}
