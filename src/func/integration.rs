pub struct Integration;

impl Integration {
    pub fn right_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        let n = ((upper_limit - lower_limit) / interval_size).ceil();
        let t = (upper_limit - lower_limit) / n;
        let mut result = 0_f64;

        for i in 1..(n + 1_f64) as u64 {
            result += function(i as f64 * t + lower_limit)
        }

        interval_size * result
    }

    pub fn left_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        let n = ((upper_limit - lower_limit) / interval_size).ceil();
        let t = (upper_limit - lower_limit) / n;
        let mut result = 0_f64;

        for i in 0..n as u64 {
            result += function(i as f64 * t + lower_limit)
        }

        interval_size * result
    }

    pub fn midpoint_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        let n = ((upper_limit - lower_limit) / interval_size).ceil();
        let t = (upper_limit - lower_limit) / (n / interval_size);
        let mut result = 0_f64;

        for i in 0..n as u64 {
            result += function((i as f64 + (1_f64 / 2_f64)) * t + lower_limit)
        }

        interval_size * result
    }

    pub fn trapezoid<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        let n = ((upper_limit - lower_limit) / interval_size).ceil();
        let t = (upper_limit - lower_limit) / n;
        let mut result = 0_f64;

        for i in 0..n as u64 {
            result += function(t * i as f64) + function(t * (i as f64 + 1_f64))
        }

        t * result * (1_f64 / 2_f64)
    }

    pub fn simpson<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        let n = ((upper_limit - lower_limit) / interval_size).ceil();
        let h = (upper_limit - lower_limit) / (2_f64 * n);
        let mut result1 = 0_f64;
        let mut result2 = 0_f64;

        for i in 1..n as u64 {
            result1 += 2_f64 * function(2_f64 * h * i as f64 + lower_limit)
        }

        for i in 1..(n + 1_f64) as u64 {
            result2 += 4_f64 * function(h * (-1_f64 + 2_f64 * i as f64) + lower_limit)
        }

        (1_f64 / 3_f64) * h * (function(lower_limit) + result1 + result2 + function(upper_limit))
    }
}
