# NumeriLib v0.1.0 Change Log:

1. We are officially renaming Ferrate to NumeriLib. This will be the final name change for the project.

2. Moving Branches:
   * Moved `Functions` out of `special` and into the root of the crate.
      * You can access it from `numerilib::Functions` now.
   * Moved `Probability` out of `special` and into the root of the crate.
      * You can access it from `numerilib::Probability` now.
   
3. `numerilib::Functions` Changes:
    * More Numerical Integration Methods:
        * `numerilib::Functions::right_riemann`
        * `numerilib::Functions::left_riemann`
        * `numerilib::Functions::midpoint_riemann`
        * `numerilib::Functions::trapezoid`
        * `numerilib::Functions::simpson`
        * `numerilib::Functions::boole_rule`
        * `numerilib::Functions::adaptive_quadrature`
    * Moved Error Functions outside of `Functions`
      * They can now be accessed via `numerilib::special::Error`
    * Factorials and Factorial Related Functions:
      * Moved Factorials and Factorial Related Functions outside of `Functions` to `Probability`
        * They can now be accessed via `numerilib::special::Probability`
      * Added Rising Factorials `Pochammer's Symbol` and Falling Factorials
        * `numerilib::special::Probability::pochhammer`
        * `numerilib::special::Probability::falling_factorial`

4. More Special Functions:
   * The Previously Mentioned Error Functions
     * `numerilib::special::Error`
   * Hypergeomeric Functions
     * `numerilib::special::Hypergeometric`
     * A Quick Note on Hypergeometric Functions:
       * The Hypergeometric Functions are not fully implemented yet. They are still a work in progress.
       * Most Importantly the `numerilib::special::Hypergeometric::gaussian` Hypergeometric Function is fully not implemented yet. It is only accurate when 0 < `z` < 0.5
         * All the Transforms will be implemented in the next release
   * The Previously Mentioned Probability Functions
     * `numerilib::special::Probability`

5. A Much Better Matrix API:
   * The Matrix API has been completely reworked and is now much more intuitive and easy to use.
   * For now `Gaussian Elimination` has been removed and will be re-added in the next release.

6. Statistics:
   * List Based Statistics:
     * Given a list you are now able to calculate basic functions
       * `numerilib::stats::Generic::mean`
       * `numerilib::stats::Generic::five_number_summary`
       * `numerilib::stats::Generic::population_sd`
       * `numerilib::stats::Generic::population_variance`
       * `numerilib::stats::Generic::sample_sd`
       * `numerilib::stats::Generic::sample_variance`
       * `numerilib::stats::Generic::size`
       * `numerilib::stats::Generic::sum`
       * `numerilib::stats::Generic::sum_squared`
     * More Distributions & Functions:
       * The New Distributions:
         * `numerilib::stats::distr::Binomial`
         * `numerilib::stats::distr::Cauchy`
         * `numerilib::stats::distr::Chi`
         * `numerilib::stats::distr::ChiSquared`
         * `numerilib::stats::distr::Fisher`
         * `numerilib::stats::distr::GeometricFailure`
         * `numerilib::stats::distr::GeometricTrials`
         * `numerilib::stats::distr::Hypergeometric`
         * `numerilib::stats::distr::Poisson`
       * Added tailed-CDF and CDF functions
       * Added many, many functions for each distribution (too many to list)

7. Docs.rs Changed:
   * Made Docs much more fleshed out and added in-depth examples for every function.
   * removed all the Links
   * Tests will no longer be in API Docs. Rather they will be in the new `tests` Directory of the project.

8. MDBook Website is Published:
   * The MDBook Website is now published and can be found at <a href="https://vlambda.github.io/" target="_blank">vlambda.github.io</a>
   * The MDBook Website is still a work in progress and will be updated regularly. It is still missing a lot of content, but will be fully update to date by the next release.

9. Imaginary and Complex Numbers:
   * Added Imaginary and Complex Numbers
     * `numerilib::Complex`

10. A better README.md

11. The Future of `ferrate`, `vml`, and `rusty_stats`:
    * `ferrate` v0.0.4 will be the last release of `ferrate`. It will be yanked on the next release of `numerilib`.
    * `vml` and `rusty_stats` are currently yanked.
