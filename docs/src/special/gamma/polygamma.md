# Polygamma

- The polygamma functions, denoted as (\\( \psi^{n}\left( z \right) \\)), are mathematical functions that are extensions of the digamma function (\\( \psi\left( z \right) \\)), also known as the psi or di-gamma function. The digamma function is the logarithmic derivative of the gamma function (\\( \Gamma\left( z \right) \\)), which is a widely used function in mathematics and has applications in various fields.

***

## Ferrate Functions:

### Digamma:

- The definition of the first polygamma function

\\[ \psi\left( z \right)=\frac{d }{dz}\Gamma\left( z \right)=\frac{\Gamma'\left( z \right)}{\Gamma\left( z \right)} \\]

#### Example:

```,norun,rust                                      
use ferrate::special::Polygamma;         
                                         
let z = 2_f64;                           
let digamma = Polygamma::digamma(z);     
                                         
assert_eq!(digamma, 0.42278438084235914);
```

### Polygamma:

- Calculates the Polygamma function with its series representation

\\[ \psi^{\left( m \right)}\left( z \right):=\frac{d^{m}}{dz^{m}}\psi\left( z \right)=\left( -1 \right)^{\left( m+1 \right)}m!\sum_{k=0}^{\infty }\frac{1}{\left( z+k \right)^{m+1}}\qquad m\gt 1 \\]

\\[ \psi^{-1}\left( z \right)=\text{ln}\Gamma\left( z \right) \\]

#### Example:

```,norun,rust                                             
use ferrate::special::Polygamma;                
                                                
let degree = 0;                                 
let z = 5_f64;                                  
                                                
let polygamma = Polygamma::polygamma(degree, z);
                                                
assert_eq!(polygamma, 1.5061177964312848_f64);  
```                                             