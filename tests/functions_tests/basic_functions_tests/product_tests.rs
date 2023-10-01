use numerilib::Functions;

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn constant_test() {
        let constant = |x: f64| 3_f64;
        let lower_bound = 2_f64;
        let upper_bound = 7_f64;
        let product = Functions::product(lower_bound, upper_bound, constant);

        assert_approx_eq!(729_f64, product);
    }

    #[test]
    pub fn function_test() {
        let function = |x: f64| x.powi(2);
        let lower_bound = 3_f64;
        let upper_bound = 7_f64;
        let product = Functions::product(lower_bound, upper_bound, function);

        assert_approx_eq!(6350400_f64, product);
    }
}
