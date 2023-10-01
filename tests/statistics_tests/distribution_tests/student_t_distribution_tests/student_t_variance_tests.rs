use numerilib::stats::distr::Student;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn student_t_variance_test1() {
        let df = 6_f64;

        let x = Student::variance(df);

        assert_approx_eq!(1.5, x);
    }

    #[test]
    fn student_t_variance_test2() {
        let df = 63_f64;

        let x = Student::variance(df);

        assert_approx_eq!(63_f64 / 61_f64, x);
    }

    #[test]
    fn student_t_variance_test3() {
        let df = 1_f64;

        let x = Student::variance(df);

        assert_eq!(f64::INFINITY, x);
    }

    #[test]
    fn student_t_variance_test4() {
        let df = 30_f64;

        let x = Student::variance(df);

        assert_approx_eq!(30_f64 / 28_f64, x);
    }
}
