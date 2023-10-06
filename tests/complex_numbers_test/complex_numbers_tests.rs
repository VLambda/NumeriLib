use numerilib::Complex;

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_new() {
        let z = Complex::new(1.0, 2.0);

        assert_eq!(z.real_part(), 1.0);
        assert_eq!(z.imag_part(), 2.0);
    }

    #[test]
    fn test_magnitude() {
        let z = Complex::new(1.0, 2.0);

        assert_approx_eq!(5_f64.sqrt(), z.magnitude());
    }

    #[test]
    fn test_powi() {
        let z = Complex::new(1.0, 2.0);

        let z_to_the_power_of_3 = z.powi(3);

        assert_approx_eq!(-11.0, z_to_the_power_of_3.real_part());
        assert_approx_eq!(-2.0, z_to_the_power_of_3.imag_part());
    }

    #[test]
    fn test_powf() {
        let z = Complex::new(1.0, 2.0);

        let z_to_the_power_of_3 = z.powf(2.0);

        assert_approx_eq!(-3.0, z_to_the_power_of_3.real_part());
        assert_approx_eq!(4.0, z_to_the_power_of_3.imag_part());
    }

    #[test]
    fn test_add() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);

        let z3 = z1 + z2;

        assert_approx_eq!(4.0, z3.real_part());
        assert_approx_eq!(6.0, z3.imag_part());
    }

    #[test]
    fn test_sub() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);

        let z3 = z1 - z2;

        assert_approx_eq!(-2.0, z3.real_part());
        assert_approx_eq!(-2.0, z3.imag_part());
    }

    #[test]
    fn test_mul() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);

        let z3 = z1 * z2;

        assert_approx_eq!(-5.0, z3.real_part());
        assert_approx_eq!(10.0, z3.imag_part());
    }

    #[test]
    fn test_div() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);

        let z3 = z1 / z2;

        assert_approx_eq!(0.44, z3.real_part());
        assert_approx_eq!(0.08, z3.imag_part());
    }

    #[test]
    fn test_display() {
        let z = Complex::new(1.0, 2.0);

        assert_eq!("1+2i", format!("{}", z));
    }
}
