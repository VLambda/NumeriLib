use crate::Functions;

pub struct Integration;

impl Integration {
    pub fn right_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        let deltax = (upper_limit - lower_limit) / intervals;

        let result = Functions::summation(1_f64, intervals, |x: f64| {
            function(x * deltax + lower_limit)
        });

        deltax * result
    }

    pub fn left_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        let deltax = (upper_limit - lower_limit) / intervals;

        let result = Functions::summation(0_f64, intervals - 1_f64, |x: f64| {
            function(x * deltax + lower_limit)
        });

        deltax * result
    }

    pub fn midpoint_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        let deltax = (upper_limit - lower_limit) / intervals;

        let result = Functions::summation(0_f64, intervals - 1_f64, |x: f64| {
            function((x + 0.5) * deltax + lower_limit)
        });

        deltax * result
    }

    pub fn trapezoid<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        let deltax = (upper_limit - lower_limit) / intervals;
        let func = |x: f64| {
            function(lower_limit + deltax * x) + function(lower_limit + deltax * (x + 1_f64))
        };

        let result = Functions::summation(0_f64, intervals - 1_f64, func);

        deltax * result * (1_f64 / 2_f64)
    }

    pub fn simpson<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        intervals: f64,
    ) -> f64 {
        let deltax = (upper_limit - lower_limit) / intervals;

        let func1 = |x: f64| function(deltax * (2_f64 * x - 1_f64) + lower_limit);
        let func2 = |x: f64| function(deltax * (2_f64 * x) + lower_limit);

        let sum1 = 4_f64 * Functions::summation(1_f64, intervals / 2_f64, func1);
        let sum2 = 2_f64 * Functions::summation(1_f64, intervals / 2_f64 - 1_f64, func2);

        (1_f64 / 3_f64) * deltax * (function(lower_limit) + sum1 + sum2 + function(upper_limit))
    }

    pub fn boole_rule<F: Fn(f64) -> f64>(function: F, lower_limit: f64, upper_limit: f64) -> f64 {
        let deltax = (upper_limit - lower_limit) / 4.0;
        let lower_limit_fx = function(lower_limit);
        let upper_limit_fx = function(upper_limit);

        (2.0 / 45.0)
            * deltax
            * (7.0 * lower_limit_fx
                + 32.0 * function(lower_limit + deltax)
                + 12.0 * function(lower_limit + 2.0 * deltax)
                + 32.0 * function(lower_limit + 3.0 * deltax)
                + 7.0 * upper_limit_fx)
    }

    pub fn adaptive_quadrature<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        tolerance: f64,
    ) -> f64 {
        fn adaptive_quadrature_recursive<F>(
            f: &F,
            a: f64,
            b: f64,
            fa: f64,
            fb: f64,
            fab: f64,
            tolerance: f64,
        ) -> f64
        where
            F: Fn(f64) -> f64,
        {
            let mid = (a + b) / 2.0;
            let h = (b - a) / 2.0;
            let fmid = f(mid);
            let left_area = h * (fa + 4.0 * fmid + fb) / 6.0;
            let right_area = h * (fa + 4.0 * fab + fb) / 6.0;
            let error = (left_area - right_area).abs();

            if error <= tolerance {
                left_area + right_area
            } else {
                let left_result = adaptive_quadrature_recursive(
                    f,
                    a,
                    mid,
                    fa,
                    fmid,
                    (fa + fmid) / 2.0,
                    tolerance,
                );
                let right_result = adaptive_quadrature_recursive(
                    f,
                    mid,
                    b,
                    fmid,
                    fb,
                    (fmid + fb) / 2.0,
                    tolerance,
                );
                left_result + right_result
            }
        }

        let fa = function(lower_limit);
        let fb = function(upper_limit);
        let fab = (fa + fb) / 2.0;

        adaptive_quadrature_recursive(&function, lower_limit, upper_limit, fa, fb, fab, tolerance)
    }
}
