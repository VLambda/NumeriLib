use mathematica::Functions;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn derivative_test() {
        let function = |x: f64| x.powf(2_f64);
        let derivative = Functions::derivative(function, 2_f64);
        assert_approx_eq!(4_f64, derivative, 1e-2)
    }

    #[test]
    fn right_riemann() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let interval_size = 1_f64;
        let function = |x: f64| x.powf(2_f64);

        let integral = Functions::right_riemann(function, lower_bound, upper_bound, interval_size);

        assert_approx_eq!(91_f64, integral, 1_f64);
    }

    #[test]
    fn left_riemann() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let interval_size = 1_f64;
        let function = |x: f64| x.powi(2);

        let integral = Functions::left_riemann(function, lower_bound, upper_bound, interval_size);

        assert_approx_eq!(55_f64, integral, 1_f64);
    }

    #[test]
    fn midpoint_riemann() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let interval_size = 1_f64;
        let function = |x: f64| x.powi(2);

        let integral =
            Functions::midpoint_riemann(function, lower_bound, upper_bound, interval_size);

        assert_approx_eq!(71.5_f64, integral, 1_f64);
    }

    #[test]
    fn trapezoid() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let interval_size = 1_f64;
        let function = |x: f64| x.powi(2);

        let integral = Functions::trapezoid(function, lower_bound, upper_bound, interval_size);

        assert_approx_eq!(73_f64, integral, 1_f64);
    }

    #[test]
    fn simpson() {
        let lower_bound = 0_f64;
        let upper_bound = 6_f64;
        let interval_size = 1_f64;
        let function = |x: f64| x.powi(2);

        let integral = Functions::simpson(function, lower_bound, upper_bound, interval_size);

        assert_approx_eq!(72_f64, integral, 1_f64);
    }

    #[test]
    fn factorial_whole() {
        let x = 6_f64;

        let fact = Functions::factorial(x);

        assert_approx_eq!(720_f64, fact, 1e-6)
    }

    #[test]
    fn factorial_frac() {
        let x = 6.6_f64;

        let fact = Functions::factorial(x);

        assert_approx_eq!(2275.032699_f64, fact, 1e-2)
    }

    #[warn(unused_variables)]
    #[test]
    fn summation_constant_test() {
        let start = 0;
        let limit = 9;
        let function = |x: f64| 3_f64;

        let summation = Functions::summation(start, limit, function);

        assert_eq!(30_f64, summation);
    }

    #[test]
    fn summation_function_test() {
        let start = 4.5;
        let limit = 100;
        let function = |x: f64| 1_f64 / x;

        let summation = Functions::summation(start, limit, function);

        assert_approx_eq!(3.104044_f64, summation, 1e-2);
    }

    #[test]
    #[warn(unused_variables)]
    fn product_constant_test() {
        let start = 2_f64;
        let limit = 7_f64;
        let function = |x: f64| 3_f64;

        let product_series = Functions::product(start, limit, function);

        assert_approx_eq!(729_f64, product_series, 1e-1);
    }

    #[test]
    fn product_function_test() {
        let start = 1_f64;
        let limit = 6_f64;
        let function = |x: f64| x.powi(2);

        let product_series = Functions::product(start, limit, function);

        assert_approx_eq!(518400_f64, product_series, 1e-1);
    }

    #[test]
    fn newton_raphson_test() {
        let x = 1.5_f64;
        let function = |x: f64| x.powi(2) - 2_f64;
        let newton = Functions::newmet(x, function);

        assert_approx_eq!(std::f64::consts::SQRT_2, newton, 1e-10);
    }

    #[test]
    fn pochhammer_test() {
        let x = 10_f64;
        let n = 6_f64;
        let poch = Functions::pochhammer(x, n);

        assert_approx_eq!(3603600_f64, poch, 1e-1);
    }

    #[test]
    fn fallfactorial_test() {
        let x = 3_f64;
        let n = 2_f64;
        let fall = Functions::fallfactorial(x, n);

        assert_approx_eq!(6_f64, fall, 1e-10);
    }
}
