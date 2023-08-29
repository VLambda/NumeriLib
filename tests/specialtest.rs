use mathematica::special::{Beta, Error, Gamma, Hypergeometric, Polygamma, Probability};

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_beta_functions() {
        assert_approx_eq!(Beta::lnbeta(1.0, 2.0), -0.6931471805616405, 1e-6);
        assert_approx_eq!(Beta::beta(8.0, 7.0), 4.162504162405661e-5, 1e-6);
        assert_approx_eq!(
            Beta::incbeta(1_f64 / 7_f64, 0.5_f64, 3_f64),
            0.6870211373344728_f64,
            1e-6
        );
        assert_approx_eq!(
            Beta::regincbeta(1_f64 / 2_f64, 3_f64, 1_f64 / 7_f64),
            0.6440823162530317,
            1e-6
        );
        assert_approx_eq!(
            Beta::invregincbeta(1_f64, 2_f64, 0.590401_f64),
            0.3600007812492397_f64,
            1e-6
        );
    }

    #[test]
    fn test_error_functions() {
        assert_approx_eq!(Error::erf(4.0), 0.9999999845946841, 1e-6);
        assert_approx_eq!(Error::erfc(4.0), 0.000000015405315911820594, 1e-6);
        assert_approx_eq!(Error::inverf(0.975), 1.5849110680594818, 1e-6);
    }

    #[test]
    fn test_gamma_functions() {
        assert_approx_eq!(Gamma::stirling(2.0), 1.9190043514889832, 1e-6);
        assert_approx_eq!(Gamma::lanczosln(6.0), 4.787491742764145, 1e-6);
        assert_approx_eq!(Gamma::lanczos(6.0), 120_f64, 1e-6);
        assert_approx_eq!(Gamma::incgamma(3_f64, 1_f64), 0.16060279414278839_f64, 1e-6);
        assert_approx_eq!(Gamma::incgammac(3_f64, 1_f64), 1.8393972058572117_f64, 1e-6);
        assert_approx_eq!(
            Gamma::reggamma(5_f64, 2_f64),
            0.052653017343711174_f64,
            1e-6
        );
        assert_approx_eq!(Gamma::reggammac(5_f64, 2_f64), 0.9473469826562888_f64, 1e-6);
    }

    #[test]
    fn test_hypergeometric_functions() {
        assert_approx_eq!(
            Hypergeometric::gaussian(2_f64, 3_f64, 4_f64, 0.5),
            2.7289353331109574,
            1e-6
        );
        assert_approx_eq!(
            Hypergeometric::kummer(2_f64, 3_f64, 0.5),
            1.4051149171994874,
            1e-6
        );
        assert_approx_eq!(
            Hypergeometric::whittaker(2_f64, 3_f64, 0.5),
            0.07682805702800186,
            1e-6
        );
    }

    #[test]
    fn test_polygamma_functions() {
        assert_approx_eq!(Polygamma::digamma(2.0), 0.42278438084235914, 1e-6);
        assert_approx_eq!(
            Polygamma::polygamma(4, 5_f64),
            -0.014063191342111519_f64,
            1e-6
        );
        assert_approx_eq!(
            Polygamma::polygamma(-1, 5_f64),
            3.1780538303306463_f64,
            1e-6
        );
        assert_approx_eq!(Polygamma::polygamma(0, 5_f64), 1.5061177964312848_f64, 1e-6);
    }

    #[test]
    fn test_probability_functions() {
        assert_eq!(Probability::permutation(3.0, 4.0), 24.0);
        assert_eq!(Probability::combination(3.0, 4.0), 4.0);
        // Add more test cases for other Probability functions
    }
}
