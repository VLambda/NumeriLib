use numerilib::stats::distr::Student;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn student_t_kurtosis_test1() {
        let df = 6_f64;

        let x = Student::kurtosis(df);

        assert_approx_eq!(3_f64, x);
    }

    #[test]
    fn student_t_kurtosis_test2() {
        let df = 63_f64;

        let x = Student::kurtosis(df);

        assert_approx_eq!(0.1016949152542373, x);
    }

    #[test]
    fn student_t_kurtosis_test3() {
        let df = 2_f64;

        let x = Student::kurtosis(df);

        assert_eq!(f64::INFINITY, x);
    }

    #[test]
    fn student_t_kurtosis_test4() {
        let df = 30_f64;

        let x = Student::kurtosis(df);

        assert_approx_eq!(0.23076923076923078, x);
    }
}
