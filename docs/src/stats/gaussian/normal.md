# Gaussian/Normal Distributions

## What is a Gaussian/Normal Distribution

- The Normal, or Gaussian, distribution is a continuous probability distribution that is characterized by its bell-shaped curve. <br>

- It is widely used in statistics and probability theory due to its prevalence in natural phenomena and its numerous applications in various fields. <br>

- The distribution is essential for modeling real-world phenomena that are influenced by multiple random factors, and it forms the basis for various statistical methods, such as hypothesis testing, confidence intervals, and regression analysis. Additionally, it plays a crucial role in fields like finance, physics, engineering, and social sciences, where understanding and modeling random variables are paramount.

***

## Ferrate Functions:

### Probability Density Function (PDF)

The PDF Formula for the Normal Distributions is: <br>

\\[ \text{NormPDF}\left( x,\mu,\sigma \right)=\frac{1}{\sigma\sqrt{2\pi}}e^{-\frac{1}{2}\left( \frac{x-\mu}{\sigma} \right)^{2}} \\]

\\( \sigma \\): Population Standard Deviation <br>
\\( \mu \\): Population Mean <br>
\\( \text{x} \\): Variable <br>

#### Example

```,norun,rust
use ferrate::stats::distr::Gaussian;

fn main() {
    let xvalue = 0.5;
    let mean = 0 as f64;
    let sd = 1 as f64;

    let normalpdf = Gaussian::pdf(xvalue, mean, sd);

    assert_eq!(normalpdf, 0.3520653267642995);
}
```

### Cumulative Density Function (CDF)

The CDF Formula for the Normal Distribution is: 

CDF:

\\[ \text{CDF}\left( x,\mu,\sigma \right) = \frac{1}{2}\left[ 1+\text{erf}\left( \frac{x-\mu}{\sigma\sqrt{2}} \right) \right] \\] <br>

Error Function \\( \text{erf}\left( z \right) \\):

\\[ \text{erf}\left( z \right) = \frac{2}{\sqrt{\pi}}\int_{0}^{z}e^{-t^{2}}dt \\] <br>

Upper and Lower Bound CDF:

\\[ \text{NormCDF}\left( l,u,\mu,\sigma \right)=\frac{\text{CDF}\left( l,\mu,\sigma \right)+\text{CDF}\left( u,\mu,\sigma \right)}{2} \\] <br>

\\( \sigma \\): Population Standard Deviation <br>
\\( \mu \\): Population Mean <br>
\\( \text{x} \\): Variable <br>
\\( \text{l} \\): Lower Bound <br>
\\( \text{u} \\): Upper Bound <br>

#### Example

```,norun,rust
use ferrate::stats::distr::Gaussian;

fn main() {
    let lower = 45 as f64;
    let upper = 56 as f64;
    let mean = 42 as f64;
    let sd = 3.6;
    
    let normalcdf = Gaussian::cdf(lower, upper, mean, sd);
    
    assert_eq!(normalcdf, 0.20227802886072038);
}
```

### Inverse CDF

A Simple Representation of the Inverse CDF function of the Normal Distribution is:

Right Tailed:

\\[ \text{Q}_{r}^{-1}\left( x,\mu,\sigma \right)=\sigma\sqrt{2}\cdot \text{erf}^{-1}\left[ 1-2x \right]+\mu \\] <br>

Left Tailed:

\\[ \text{Q}_{l}^{-1}\left( x,\mu,\sigma \right)=-\left(  \sigma\sqrt{2}\cdot \text{erf}^{-1}\left[ 1-2x \right] \right)+\mu \\] <br>

Definition of \\(  \text{erf}^{-1}\left( z \right) \\):

\\[ w ==\text{erf}^{-1}\left( z \right) /; z== \text{erf}\left( w \right) \\] <br>

\\( \sigma \\): Population Standard Deviation <br>
\\( \mu \\): Population Mean <br>
\\( \text{x} \\): Variable <br>

#### Example #1: Right Side Non-Standard Normal Distribution

```,norun,rust
use ferrate::stats::distr::Gaussian;

fn main() {
    let area = 0.589255651;
    let mean = 42;
    let sd = 3.6;
    let tail = "Right";
    
    let invnorm = Gaussian::inv(area, mean, sd, tail).unwrap();
    assert_eq!(invnorm, 41.187729649603824);
}
```

#### Example #2: Left Side Standard Normal Distribution

```,norun,rust
use ferrate::stats::distr::Gaussian;

fn main() {
    let area = 0.975;
    let mean = 0;
    let sd = 1;
    let tail = "Left";
    
    let invnorm = Gaussian::inv(area, mean, sd, tail).unwrap();
    assert_eq!(invnorm, 1.9599639845401289);
}
```