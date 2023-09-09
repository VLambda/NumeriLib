macro_rules! define_gaussian_tests {
    ($module:ident) => {
        use assert_approx_eq::assert_approx_eq;
        use mathematica::stats::distr::$module as Gaussian;

        #[test]
        fn pdf_test() {
            let x_value = 0.5;
            let mean = 0.0;
            let sd = 1.0;

            let normalpdf = Gaussian::pdf(x_value, mean, sd);

            assert_approx_eq!(normalpdf, 0.3520653267642995);
        }

        #[test]
        fn cdf_test() {
            let bound = 1.96;
            let mean = 0.0;
            let sd = 1.0;

            let normalcdf = Gaussian::cdf(bound, mean, sd);

            assert_approx_eq!(normalcdf, 0.024997895148220428);
        }

        #[test]
        fn tailcdf_test() {
            let lower = 45.0;
            let upper = 56.0;
            let mean = 42.0;
            let sd = 3.6;

            let normalcdf = Gaussian::tailcdf(lower, upper, mean, sd);

            assert_approx_eq!(normalcdf, 0.2023787330665709);
        }

        #[test]
        fn inv_test_right() {
            let area = 0.589255651;
            let mean = 42.0;
            let sd = 3.6;
            let tail = "Right";

            let invnorm = Gaussian::inv(area, mean, sd, tail).unwrap();
            assert_approx_eq!(invnorm, 41.18772964960383);
        }

        #[test]
        fn inv_test_left() {
            let area = 0.975;
            let mean = 0.0;
            let sd = 1.0;
            let tail = "Left";

            let invnorm = Gaussian::inv(area, mean, sd, tail).unwrap();
            assert_approx_eq!(invnorm, 1.9599639845400547);
        }

        #[test]
        fn mad_test() {
            let omega = 1.0;
            let mad_result = Gaussian::mad(omega);

            assert_approx_eq!(mad_result, 0.6744897501960815);
        }
    };
}

macro_rules! define_student_tests {
    ($module:ident) => {
        use assert_approx_eq::assert_approx_eq;
        use mathematica::stats::distr::$module as Student;

        #[test]
        fn pdf_test() {
            let x_value = 0.975;
            let df = 6_f64;

            let tpdf = Student::pdf(x_value, df);

            assert_approx_eq!(tpdf, 0.22873968790971655);
        }

        #[test]
        fn cdf_test() {
            let bound = 1_f64;
            let df = 6_f64;

            let tcdf = Student::cdf(bound, df);

            assert_approx_eq!(tcdf, 0.8220411581265159);
        }

        #[test]
        fn tailcdf_test() {
            let lower = 1_f64;
            let upper = 1.96;
            let df = 6_f64;

            let tcdf = Student::tailcdf(lower, upper, df);

            assert_approx_eq!(tcdf, 0.12911126556567942);
        }

        #[test]
        fn inv_test() {
            let area = 0.025_f64;
            let df = 63_f64;

            let inverse_t = Student::inv(area, df);

            assert_approx_eq!(inverse_t, -1.9978075067095558);
        }

        #[test]
        fn variance_test() {
            let df = 6_f64;

            let variance = Student::variance(df);

            assert_approx_eq!(variance, 1.5);
        }

        #[test]
        fn sd_test() {
            let df = 6_f64;

            let sd = Student::sd(df);

            assert_approx_eq!(sd, 1.224744871391589);
        }

        #[test]
        fn skewness_test() {
            let df = 2_f64;

            let skewness = Student::skewness(df);

            assert!(skewness.is_nan());
        }

        #[test]
        fn kurtosis_test() {
            let df = 1_f64;

            let kurtosis = Student::kurtosis(df);

            assert!(kurtosis.is_nan());
        }
    };
}

macro_rules! define_poisson_tests {
    ($module:ident) => {
        use assert_approx_eq::assert_approx_eq;
        use mathematica::stats::distr::$module as Poisson;

        #[test]
        fn pmf_test() {
            let k = 7_f64;
            let lambda = 8_f64;

            let pmf = Poisson::pmf(k, lambda);

            assert_approx_eq!(pmf, 0.13958653195059692);
        }

        #[test]
        fn cdf_test() {
            let k = 7_f64;
            let lambda = 8_f64;

            let cdf = Poisson::cdf(k, lambda);

            assert_approx_eq!(cdf, 0.4529608094815124);
        }

        #[test]
        fn median_test() {
            let lambda = 8_f64;

            let median = Poisson::median(lambda);

            assert_approx_eq!(median, 8_f64);
        }

        #[test]
        fn mode_test() {
            let lambda = 8_f64;

            let mode = Poisson::mode(lambda);

            assert_eq!(mode, 8_f64);
        }

        #[test]
        fn sd_test() {
            let lambda = 8_f64;

            let sd = Poisson::sd(lambda);

            assert_approx_eq!(sd, 2.8284271247461903);
        }

        #[test]
        fn skewness_test() {
            let lambda = 8_f64;

            let skewness = Poisson::skewness(lambda);

            assert_approx_eq!(skewness, 0.35355339059327373);
        }

        #[test]
        fn kurtosis_test() {
            let lambda = 8_f64;

            let kurtosis = Poisson::kurtosis(lambda);

            assert_approx_eq!(kurtosis, 0.125);
        }
    };
}

macro_rules! define_hypergeometric_tests {
    ($module:ident) => {
        use assert_approx_eq::assert_approx_eq;
        use mathematica::stats::distr::$module as Hypergeometric;

        #[test]
        fn pmf_test() {
            let population = 100_f64;
            let success = 30_f64;
            let draws = 20_f64;
            let observed = 5_f64;

            let pmf = Hypergeometric::pmf(population, success, draws, observed);

            assert_approx_eq!(pmf, 0.1918255924290465);
        }

        #[test]
        fn lcdf_test() {
            let population = 100_f64;
            let success = 30_f64;
            let draws = 20_f64;
            let observed = 5_f64;

            let cdf = Hypergeometric::lcdf(population, success, draws, observed);

            assert_approx_eq!(cdf, 0.4009887932548526);
        }

        #[test]
        fn ucdf_test() {
            let population = 100_f64;
            let success = 30_f64;
            let draws = 20_f64;
            let observed = 5_f64;

            let cdf = Hypergeometric::ucdf(population, success, draws, observed);

            assert_approx_eq!(cdf, 0.790836799174194);
        }

        #[test]
        fn mean_test() {
            let population = 100_f64;
            let success = 20_f64;
            let draws = 10_f64;

            let mean = Hypergeometric::mean(population, success, draws);

            assert_approx_eq!(mean, 2.0);
        }

        #[test]
        fn mode_test() {
            let population = 100_f64;
            let success = 20_f64;
            let draws = 10_f64;

            let mode = Hypergeometric::mode(population, success, draws);

            assert_approx_eq!(mode, 2.0);
        }

        #[test]
        fn variance_test() {
            let population = 100_f64;
            let success = 20_f64;
            let draws = 10_f64;

            let variance = Hypergeometric::variance(population, success, draws);

            assert_approx_eq!(variance, 1.4545454545454546);
        }

        #[test]
        fn sd_test() {
            let population = 100_f64;
            let success = 20_f64;
            let draws = 10_f64;

            let sd = Hypergeometric::sd(population, success, draws);

            assert_approx_eq!(sd, 1.2060453783110545);
        }

        #[test]
        fn skewness_test() {
            let population = 100_f64;
            let success = 20_f64;
            let draws = 10_f64;

            let skewness = Hypergeometric::skewness(population, success, draws);

            assert_approx_eq!(skewness, 0.18162118743907743);
        }

        #[test]
        fn kurtosis_test() {
            let population = 100_f64;
            let success = 20_f64;
            let draws = 10_f64;

            let kurtosis = Hypergeometric::kurtosis(population, success, draws);

            assert_approx_eq!(kurtosis, -0.04257837155480749);
        }
    };
}
#[cfg(test)]
mod tests {
    mod student {
        define_student_tests!(Student);
    }

    mod gaussian {
        define_gaussian_tests!(Gaussian);
    }

    mod poisson {
        define_poisson_tests!(Poisson);
    }

    mod hypergeometric {
        define_hypergeometric_tests!(Hypergeometric);
    }
}
