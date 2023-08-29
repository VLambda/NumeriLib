pub struct Generic;

impl Generic {
    pub fn mean(list: &[f64]) -> f64 {
        let sum: f64 = list.iter().sum();
        let count = list.len() as f64;

        if count > 0.0 {
            sum / count
        } else {
            panic!("Cannot calculate mean for an empty list.");
        }
    }

    pub fn sum(list: &[f64]) -> f64 {
        list.iter().sum()
    }

    pub fn sum_squared(list: &[f64]) -> f64 {
        list.iter().map(|&x| x * x).sum()
    }

    pub fn size(list: &[f64]) -> usize {
        list.len()
    }

    pub fn population_variance(list: &[f64]) -> f64 {
        let mu = Self::mean(list);
        let squared_deviations_sum = list.iter().map(|x| (x - mu).powi(2)).sum::<f64>();
        squared_deviations_sum / Self::size(list) as f64
    }

    pub fn sample_variance(list: &[f64]) -> f64 {
        let mean = Self::mean(list);
        let squared_deviations_sum = list.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
        squared_deviations_sum / (Self::size(list) - 1) as f64
    }

    pub fn population_sd(list: &[f64]) -> f64 {
        Self::population_variance(list).sqrt()
    }

    pub fn sample_sd(list: &[f64]) -> f64 {
        Self::sample_variance(list).sqrt()
    }

    pub fn five_number_summary(list: &[f64]) -> Vec<f64> {
        let mut sorted_list = list.to_vec();
        sorted_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = sorted_list[0];
        let max = sorted_list[sorted_list.len() - 1];

        let median_index = (sorted_list.len() as f64 / 2.0).floor() as usize;
        let median = if sorted_list.len() % 2 == 0 {
            (sorted_list[median_index - 1] + sorted_list[median_index]) / 2.0
        } else {
            sorted_list[median_index]
        };

        let lower_half = &sorted_list[..median_index];
        let upper_half = &sorted_list[median_index + sorted_list.len() % 2..];

        let q1_index = (lower_half.len() as f64 / 2.0).floor() as usize;
        let q1 = if lower_half.len() % 2 == 0 {
            (lower_half[q1_index - 1] + lower_half[q1_index]) / 2.0
        } else {
            lower_half[q1_index]
        };

        let q3_index = (upper_half.len() as f64 / 2.0).floor() as usize;
        let q3 = if upper_half.len() % 2 == 0 {
            (upper_half[q3_index - 1] + upper_half[q3_index]) / 2.0
        } else {
            upper_half[q3_index]
        };

        vec![min, q1, median, q3, max]
    }
}
