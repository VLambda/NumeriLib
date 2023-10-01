use numerilib::Functions;

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn constant_test() {
        let constant = |x: f64| 3_f64;
        let lower_bound = 0_f64;
        let upper_bound = 9_f64;
        let sum = Functions::summation(lower_bound, upper_bound, constant);

        assert_approx_eq!(30_f64, sum);
    }

    #[test]
    pub fn function_test() {
        let function = |x: f64| 1_f64 / x;
        let lower_bound = 4.5;
        let upper_bound = 100_f64;
        let sum = Functions::summation(lower_bound, upper_bound, function);

        assert_approx_eq!(3.2163034262223156, sum);
    }
}
