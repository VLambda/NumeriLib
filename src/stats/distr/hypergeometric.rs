use crate::special::Probability;

pub struct Hypergeometric;

impl Hypergeometric {
    pub fn pmf(population: f64, success: f64, draws: f64, observed: f64) -> f64 {
        (Probability::combination(success, observed)
            * Probability::combination(population - success, draws - observed))
            / Probability::combination(population, observed)
    }

    pub fn cdf(population: f64, success: f64, draws: f64, observed: f64) -> f64 {
        let mut result = 0_f64;
        let x = population as i32;

        for i in 0..x {
            result += (Probability::combination(draws, i as f64)
                * Probability::combination(success - draws, observed - i as f64))
                / Probability::combination(success, observed)
        }

        result
    }

    pub fn mean(population: f64, success: f64, draws: f64) -> f64 {
        draws * (success / population)
    }

    pub fn mode(population: f64, success: f64, draws: f64) -> f64 {
        (((draws + 1_f64) * (success + 1_f64)) / (population + 2_f64)).floor()
    }

    pub fn variance(population: f64, success: f64, draws: f64) -> f64 {
        draws
            * (success / population)
            * ((population - success) / population)
            * ((population - draws) / (population - 1_f64))
    }

    pub fn sd(population: f64, success: f64, draws: f64) -> f64 {
        Self::variance(population, success, draws).sqrt()
    }

    pub fn skewness(population: f64, success: f64, draws: f64) -> f64 {
        ((population - (2_f64 * success))
            * (population - 1_f64).sqrt()
            * (population - (2_f64 * draws)))
            / ((draws * population * (population - success) * (population - draws)).sqrt()
                * (population - 2_f64))
    }

    pub fn kurtosis(population: f64, success: f64, draws: f64) -> f64 {
        let p1 = 1_f64
            / (draws
                * success
                * (population - success)
                * (population - draws)
                * (population - 2_f64)
                * (population - 3_f64));
        let p2 = (population - 1_f64)
            * population.powi(2)
            * (population * (population + 1_f64)
                - (6_f64 * success) * (population - success)
                - (6_f64 * draws) * (population - draws))
            + (6_f64 * draws * success)
                * (population - success)
                * (population - draws)
                * ((5_f64 * population) - 6_f64);
        p1 * p2
    }
}
