pub mod ohlc {
    const BULLISH: i8 = 1;
    const BEARISH: i8 = -1;

    #[derive(Debug, PartialEq, Clone)]
    pub struct OHLC {
        pub open: f64,
        pub high: f64,
        pub low: f64,
        pub close: f64,
        pub vol: Option<f64>,
        pub ts: u64,
    }

    /// All opens from a slice of OHLC.
    pub fn opens(ohlcs: &[OHLC]) -> Vec<f64> {
        ohlcs.iter().map(|ohlc| ohlc.open).collect()
    }

    /// All highs from a slice of OHLC.
    pub fn highs(ohlcs: &[OHLC]) -> Vec<f64> {
        ohlcs.iter().map(|ohlc| ohlc.high).collect()
    }

    /// All lows from a slice of OHLC.
    pub fn lows(ohlcs: &[OHLC]) -> Vec<f64> {
        ohlcs.iter().map(|ohlc| ohlc.low).collect()
    }

    /// All closes from a slice of OHLC.
    pub fn closes(ohlcs: &[OHLC]) -> Vec<f64> {
        ohlcs.iter().map(|ohlc| ohlc.close).collect()
    }

    /// Options for filtering OHLC slices.
    pub struct Opts {
        pub exclude_before: Option<u64>,
        pub exclude_after: Option<u64>,
    }

    /// Filters a slice of OHLC returning a Vector of OHLC matching the supplied options.
    pub fn filter(ohlcs: &[OHLC], opts: Opts) -> Vec<OHLC> {
        ohlcs
            .iter()
            .filter(|ohlc| {
                let before_ok = opts.exclude_before.map_or(true, |before| ohlc.ts >= before);
                let after_ok = opts.exclude_after.map_or(true, |after| ohlc.ts <= after);
                before_ok && after_ok
            })
            .cloned()
            .collect()
    }

    impl OHLC {
        /// Return a new OHLC.
        pub fn new(open: f64, high: f64, low: f64, close: f64, ts: u64) -> Self {
            OHLC {
                open,
                high,
                low,
                close,
                vol: None,
                ts,
            }
        }

        /// Create and validate a new OHLC.
        pub fn build(
            open: f64,
            high: f64,
            low: f64,
            close: f64,
            ts: u64,
        ) -> Result<Self, Vec<String>> {
            let ohlc = Self::new(open, high, low, close, ts);
            ohlc.validate()?;
            Ok(ohlc)
        }

        /// Specify the volume for an OHLC.
        pub fn with_volume(mut self, vol: f64) -> Self {
            self.vol = Some(vol);
            self
        }

        /// Validate an OHLC.
        pub fn validate(&self) -> Result<(), Vec<String>> {
            let mut errors = Vec::new();

            if !self.open.is_finite() {
                errors.push("Open price must be finite".to_string());
            }
            if !self.high.is_finite() {
                errors.push("High price must be finite".to_string());
            }
            if !self.low.is_finite() {
                errors.push("Low price must be finite".to_string());
            }
            if !self.close.is_finite() {
                errors.push("Close price must be finite".to_string());
            }

            if self.high < self.low {
                errors.push("High price must be greater than or equal to low price".to_string());
            }

            if let Some(vol) = self.vol {
                if !vol.is_finite() || vol < 0.0 {
                    errors.push("Volume must be non-negative and finite".to_string());
                }
            }

            if self.ts == 0 {
                errors.push("Timestamp must be non-zero".to_string());
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }

        /// Returns the direction of the OHLC.
        ///
        /// Will return a zero if the open and close values are equal.
        pub fn direction(&self) -> i8 {
            if self.close > self.open {
                BULLISH
            } else if self.open > self.close {
                BEARISH
            } else {
                0
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let open = 100.0;
            let high = 110.0;
            let low = 95.0;
            let close = 105.0;
            let ts = 1625097600;

            let ohlc = OHLC::new(open, high, low, close, ts);
            let expected = OHLC {
                open,
                high,
                low,
                close,
                vol: None,
                ts,
            };

            assert_eq!(ohlc, expected, "OHLC struct should match expected values");
        }

        #[test]
        fn test_with_volume() {
            let open = 100.0;
            let high = 110.0;
            let low = 95.0;
            let close = 105.0;
            let vol = 12345.0;
            let ts = 1625097600;

            let ohlc = OHLC::new(open, high, low, close, ts).with_volume(vol);
            let expected = OHLC {
                open,
                high,
                low,
                close,
                vol: Some(vol),
                ts,
            };

            assert_eq!(ohlc, expected, "OHLC struct should match expected values");
        }

        #[test]
        fn test_opens() {
            let ohlcs = vec![
                OHLC::new(100.0, 110.0, 95.0, 105.0, 1625097600000),
                OHLC::new(200.0, 210.0, 190.0, 205.0, 1625097601000).with_volume(1000.0),
                OHLC::new(300.0, 310.0, 290.0, 305.0, 1625097602000),
            ];

            let opens = opens(&ohlcs);
            let expected = vec![100.0, 200.0, 300.0];

            assert_eq!(opens, expected, "Opens should match expected values");
        }

        #[test]
        fn test_opens_empty() {
            let ohlcs: Vec<OHLC> = vec![];
            let opens = opens(&ohlcs);
            let expected: Vec<f64> = vec![];

            assert_eq!(opens, expected, "Opens of empty slice should be empty");
        }

        #[test]
        fn test_highs() {
            let ohlcs = vec![
                OHLC::new(100.0, 110.0, 95.0, 105.0, 1625097600000),
                OHLC::new(200.0, 210.0, 190.0, 205.0, 1625097601000).with_volume(1000.0),
                OHLC::new(300.0, 310.0, 290.0, 305.0, 1625097602000),
            ];

            let highs = highs(&ohlcs);
            let expected = vec![110.0, 210.0, 310.0];

            assert_eq!(highs, expected, "Highs should match expected values");
        }

        #[test]
        fn test_highs_empty() {
            let ohlcs: Vec<OHLC> = vec![];
            let highs = highs(&ohlcs);
            let expected: Vec<f64> = vec![];

            assert_eq!(highs, expected, "Highs of empty slice should be empty");
        }

        #[test]
        fn test_lows() {
            let ohlcs = vec![
                OHLC::new(100.0, 110.0, 95.0, 105.0, 1625097600000),
                OHLC::new(200.0, 210.0, 190.0, 205.0, 1625097601000).with_volume(1000.0),
                OHLC::new(300.0, 310.0, 290.0, 305.0, 1625097602000),
            ];

            let lows = lows(&ohlcs);
            let expected = vec![95.0, 190.0, 290.0];

            assert_eq!(lows, expected, "Lows should match expected values");
        }

        #[test]
        fn test_lows_empty() {
            let ohlcs: Vec<OHLC> = vec![];
            let lows = lows(&ohlcs);
            let expected: Vec<f64> = vec![];

            assert_eq!(lows, expected, "Lows of empty slice should be empty");
        }

        #[test]
        fn test_closes() {
            let ohlcs = vec![
                OHLC::new(100.0, 110.0, 95.0, 105.0, 1625097600000),
                OHLC::new(200.0, 210.0, 190.0, 205.0, 1625097601000).with_volume(1000.0),
                OHLC::new(300.0, 310.0, 290.0, 305.0, 1625097602000),
            ];

            let closes = closes(&ohlcs);
            let expected = vec![105.0, 205.0, 305.0];

            assert_eq!(closes, expected, "Closes should match expected values");
        }

        #[test]
        fn test_closes_empty() {
            let ohlcs: Vec<OHLC> = vec![];
            let closes = closes(&ohlcs);
            let expected: Vec<f64> = vec![];

            assert_eq!(closes, expected, "Closes of empty slice should be empty");
        }

        // Helper function to create an OHLC with minimal fields for testing
        fn fake_ohlc(ts: u64) -> OHLC {
            OHLC {
                open: 100.0,
                high: 110.0,
                low: 90.0,
                close: 105.0,
                vol: Some(1000.0),
                ts,
            }
        }

        #[test]
        fn test_filter() {
            // Define test cases as a vector of (name, input_ohlcs, opts, expected_output)
            let test_cases = vec![
                (
                    "both_bounds_set",
                    vec![
                        fake_ohlc(1000),
                        fake_ohlc(2000),
                        fake_ohlc(3000),
                        fake_ohlc(4000),
                    ],
                    Opts {
                        exclude_before: Some(1500),
                        exclude_after: Some(3500),
                    },
                    vec![fake_ohlc(2000), fake_ohlc(3000)],
                ),
                (
                    "only_exclude_before",
                    vec![
                        fake_ohlc(1000),
                        fake_ohlc(2000),
                        fake_ohlc(3000),
                        fake_ohlc(4000),
                    ],
                    Opts {
                        exclude_before: Some(2000),
                        exclude_after: None,
                    },
                    vec![fake_ohlc(2000), fake_ohlc(3000), fake_ohlc(4000)],
                ),
                (
                    "only_exclude_after",
                    vec![
                        fake_ohlc(1000),
                        fake_ohlc(2000),
                        fake_ohlc(3000),
                        fake_ohlc(4000),
                    ],
                    Opts {
                        exclude_before: None,
                        exclude_after: Some(2500),
                    },
                    vec![fake_ohlc(1000), fake_ohlc(2000)],
                ),
                (
                    "no_bounds",
                    vec![
                        fake_ohlc(1000),
                        fake_ohlc(2000),
                        fake_ohlc(3000),
                        fake_ohlc(4000),
                    ],
                    Opts {
                        exclude_before: None,
                        exclude_after: None,
                    },
                    vec![
                        fake_ohlc(1000),
                        fake_ohlc(2000),
                        fake_ohlc(3000),
                        fake_ohlc(4000),
                    ],
                ),
                (
                    "exclude_before_zero",
                    vec![
                        fake_ohlc(1000),
                        fake_ohlc(2000),
                        fake_ohlc(3000),
                        fake_ohlc(4000),
                    ],
                    Opts {
                        exclude_before: Some(0),
                        exclude_after: Some(3500),
                    },
                    vec![fake_ohlc(1000), fake_ohlc(2000), fake_ohlc(3000)],
                ),
                (
                    "exclude_after_zero",
                    vec![
                        fake_ohlc(1000),
                        fake_ohlc(2000),
                        fake_ohlc(3000),
                        fake_ohlc(4000),
                    ],
                    Opts {
                        exclude_before: Some(1500),
                        exclude_after: Some(0),
                    },
                    vec![],
                ),
                (
                    "empty_input",
                    vec![],
                    Opts {
                        exclude_before: Some(1500),
                        exclude_after: Some(3500),
                    },
                    vec![],
                ),
                (
                    "single_element_within_bounds",
                    vec![fake_ohlc(2000)],
                    Opts {
                        exclude_before: Some(1500),
                        exclude_after: Some(2500),
                    },
                    vec![fake_ohlc(2000)],
                ),
                (
                    "single_element_outside_bounds",
                    vec![fake_ohlc(1000)],
                    Opts {
                        exclude_before: Some(1500),
                        exclude_after: Some(2500),
                    },
                    vec![],
                ),
            ];

            for (name, input, opts, expected) in test_cases {
                let result = filter(&input, opts);

                assert_eq!(result, expected, "Test case '{}' failed", name);
            }
        }

        #[test]
        fn test_direction() {
            struct TestCase {
                name: &'static str,
                open: f64,
                high: f64,
                low: f64,
                close: f64,
                vol: Option<f64>,
                ts: u64,
                expected_ohlc: OHLC,
                expected_direction: i8,
            }

            let test_cases = vec![
                TestCase {
                    name: "bullish direction",
                    open: 100.0,
                    high: 110.0,
                    low: 95.0,
                    close: 105.0,
                    vol: Some(12345.0),
                    ts: 1625097600,
                    expected_ohlc: OHLC {
                        open: 100.0,
                        high: 110.0,
                        low: 95.0,
                        close: 105.0,
                        vol: Some(12345.0),
                        ts: 1625097600,
                    },
                    expected_direction: BULLISH,
                },
                TestCase {
                    name: "bearish direction",
                    open: 110.0,
                    high: 110.0,
                    low: 95.0,
                    close: 100.0,
                    vol: Some(12345.0),
                    ts: 1625097600,
                    expected_ohlc: OHLC {
                        open: 110.0,
                        high: 110.0,
                        low: 95.0,
                        close: 100.0,
                        vol: Some(12345.0),
                        ts: 1625097600,
                    },
                    expected_direction: BEARISH,
                },
                TestCase {
                    name: "neutral direction",
                    open: 100.0,
                    high: 110.0,
                    low: 95.0,
                    close: 100.0,
                    vol: Some(12345.0),
                    ts: 1625097600,
                    expected_ohlc: OHLC {
                        open: 100.0,
                        high: 110.0,
                        low: 95.0,
                        close: 100.0,
                        vol: Some(12345.0),
                        ts: 1625097600,
                    },
                    expected_direction: 0,
                },
                TestCase {
                    name: "bullish with no volume",
                    open: 100.0,
                    high: 110.0,
                    low: 95.0,
                    close: 105.0,
                    vol: None,
                    ts: 1625097600,
                    expected_ohlc: OHLC {
                        open: 100.0,
                        high: 110.0,
                        low: 95.0,
                        close: 105.0,
                        vol: None,
                        ts: 1625097600,
                    },
                    expected_direction: BULLISH,
                },
                TestCase {
                    name: "edge case: zero prices",
                    open: 0.0,
                    high: 0.0,
                    low: 0.0,
                    close: 1.0,
                    vol: Some(12345.0),
                    ts: 1625097600,
                    expected_ohlc: OHLC {
                        open: 0.0,
                        high: 0.0,
                        low: 0.0,
                        close: 1.0,
                        vol: Some(12345.0),
                        ts: 1625097600,
                    },
                    expected_direction: BULLISH,
                },
                TestCase {
                    name: "edge case: equal open and close with zero",
                    open: 0.0,
                    high: 0.0,
                    low: 0.0,
                    close: 0.0,
                    vol: None,
                    ts: 1625097600,
                    expected_ohlc: OHLC {
                        open: 0.0,
                        high: 0.0,
                        low: 0.0,
                        close: 0.0,
                        vol: None,
                        ts: 1625097600,
                    },
                    expected_direction: 0,
                },
            ];

            for case in test_cases {
                let mut ohlc = OHLC::new(case.open, case.high, case.low, case.close, case.ts);
                if let Some(vol) = case.vol {
                    ohlc = ohlc.with_volume(vol);
                }
                assert_eq!(
                    ohlc, case.expected_ohlc,
                    "Test failed for '{}': OHLC struct does not match expected values",
                    case.name
                );
                assert_eq!(
                    ohlc.direction(),
                    case.expected_direction,
                    "Test failed for '{}': expected direction {}, got {}",
                    case.name,
                    case.expected_direction,
                    ohlc.direction()
                );
            }
        }

        #[test]
        fn test_build() {
            struct TestCase {
                name: &'static str,
                open: f64,
                high: f64,
                low: f64,
                close: f64,
                vol: Option<f64>,
                ts: u64,
                expected: Result<OHLC, Vec<String>>,
            }

            let test_cases = vec![
                TestCase {
                    name: "valid case without volume",
                    open: 100.0,
                    high: 110.0,
                    low: 95.0,
                    close: 105.0,
                    vol: None,
                    ts: 1625097600000,
                    expected: Ok(OHLC {
                        open: 100.0,
                        high: 110.0,
                        low: 95.0,
                        close: 105.0,
                        vol: None,
                        ts: 1625097600000,
                    }),
                },
                TestCase {
                    name: "valid case with volume",
                    open: 200.0,
                    high: 210.0,
                    low: 190.0,
                    close: 205.0,
                    vol: Some(1000.0),
                    ts: 1625097601000,
                    expected: Ok(OHLC {
                        open: 200.0,
                        high: 210.0,
                        low: 190.0,
                        close: 205.0,
                        vol: Some(1000.0),
                        ts: 1625097601000,
                    }),
                },
                TestCase {
                    name: "invalid high < low",
                    open: 100.0,
                    high: 90.0,
                    low: 95.0,
                    close: 105.0,
                    vol: None,
                    ts: 1625097600000,
                    expected: Err(vec![
                        "High price must be greater than or equal to low price".to_string(),
                    ]),
                },
                TestCase {
                    name: "invalid negative volume",
                    open: 100.0,
                    high: 110.0,
                    low: 95.0,
                    close: 105.0,
                    vol: Some(-1000.0),
                    ts: 1625097600000,
                    expected: Err(vec!["Volume must be non-negative and finite".to_string()]),
                },
                TestCase {
                    name: "invalid zero timestamp",
                    open: 100.0,
                    high: 110.0,
                    low: 95.0,
                    close: 105.0,
                    vol: None,
                    ts: 0,
                    expected: Err(vec!["Timestamp must be non-zero".to_string()]),
                },
                TestCase {
                    name: "multiple invalid fields",
                    open: f64::NAN,
                    high: 90.0,
                    low: 95.0,
                    close: f64::INFINITY,
                    vol: Some(-1000.0),
                    ts: 0,
                    expected: Err(vec![
                        "Open price must be finite".to_string(),
                        "Close price must be finite".to_string(),
                        "High price must be greater than or equal to low price".to_string(),
                        "Volume must be non-negative and finite".to_string(),
                        "Timestamp must be non-zero".to_string(),
                    ]),
                },
            ];

            for case in test_cases {
                // Create OHLC with new and apply volume if supplied
                let mut ohlc = OHLC::new(case.open, case.high, case.low, case.close, case.ts);

                if let Some(vol) = case.vol {
                    ohlc = ohlc.with_volume(vol);
                }

                // Validate the final OHLC instance
                let result = ohlc.validate().map(|_| ohlc);

                match (result, case.expected) {
                    (Ok(result), Ok(expected)) => {
                        assert_eq!(
                            result, expected,
                            "Test failed for '{}': expected Ok({:?}), got Ok({:?})",
                            case.name, expected, result
                        );
                    }
                    (Err(errors), Err(expected_errors)) => {
                        assert_eq!(
                            errors, expected_errors,
                            "Test failed for '{}': expected Err({:?}), got Err({:?})",
                            case.name, expected_errors, errors
                        );
                    }
                    (Ok(_), Err(expected_errors)) => {
                        panic!(
                            "Test failed for '{}': expected Err({:?}), got Ok",
                            case.name, expected_errors
                        );
                    }
                    (Err(errors), Ok(expected)) => {
                        panic!(
                            "Test failed for '{}': expected Ok({:?}), got Err({:?})",
                            case.name, expected, errors
                        );
                    }
                }
            }
        }
    }
}
