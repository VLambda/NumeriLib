/*  I gotta give credit where credit is due. I took the Lanczos Approximation from
    Axect's Lanczos repo at: https://github.com/Axect/Lanczos.git
 */

const G: f64 = 5f64;
const N: usize = 7;

const LG5N7: [f64; 7] = [
    1.000000000189712,
    76.18009172948503,
    -86.50532032927205,
    24.01409824118972,
    -1.2317395783752254,
    0.0012086577526594748,
    -0.00000539702438713199
];

pub struct Gamma;

impl Gamma {
    pub fn stirling(n: f64) -> f64 {
        (std::f64::consts::TAU * n).sqrt() * (n/std::f64::consts::E).powf(n)
    }

    pub fn lanczosln(z: f64) -> f64 {
        let z = z - 1f64;
        let base = z + G + 0.5;
        let mut s = 0f64;
        for i in 1 .. N {
            s += LG5N7[i] / (z + i as f64);
        }
        s += LG5N7[0];
        (2f64 * std::f64::consts::PI).sqrt().ln() + s.ln() - base + base.ln() * (z + 0.5)
    }

    pub fn lanczos(z: f64) -> f64 {
        let exp = Gamma::lanczosln(z).exp();

        if ((exp * 1e+6).ceil() - 1e-6) < (exp * 1e+6) || ((exp * 1e+6).floor() + 1e+6) > (exp * 1e+6) {
            ((exp * 1e+6).round()) / 1e+6
        } else {
            exp
        }
    }
}