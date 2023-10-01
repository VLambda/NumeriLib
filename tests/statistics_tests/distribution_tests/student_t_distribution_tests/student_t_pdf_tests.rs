use numerilib::stats::distr::Student;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn student_t_pdf_test1() {
        let x = 8_f64;
        let df = 7_f64;

        let pdf = Student::pdf(x, df);

        assert_approx_eq!(3.6375573636838696e-5, pdf);
    }

    #[test]
    fn student_t_pdf_test2() {
        let x = 13_f64;
        let df = 8_f64;

        let pdf = Student::pdf(x, df);

        assert_approx_eq!(3.430823019560095e-7, pdf);
    }

    #[test]
    fn student_t_pdf_test3() {
        let x = 0_f64;
        let df = 9_f64;

        let pdf = Student::pdf(x, df);

        assert_approx_eq!(0.38803490887312847, pdf);
    }

    #[test]
    fn student_t_pdf_test4() {
        let x = 8_f64;
        let df = 63_f64;

        let pdf = Student::pdf(x, df);

        assert_approx_eq!(7.183982393517857e-11, pdf);
    }
}
