use cfg_expr::{
    expr::Predicate,
    targets::{get_builtin_target_by_triple, ALL_BUILTINS as all},
    Expression,
};

struct Target {
    builtin: &'static cfg_expr::targets::TargetInfo<'static>,
    #[cfg(feature = "targets")]
    lexicon: target_lexicon::Triple,
}

impl Target {
    fn make(s: &str) -> Self {
        Self {
            builtin: get_builtin_target_by_triple(s).unwrap(),
            #[cfg(feature = "targets")]
            lexicon: {
                // Hack to workaround the addition in 1.48.0 of this weird, non-conformant
                // target triple, until https://github.com/bytecodealliance/target-lexicon/issues/63 is
                // resolved in a satisfactory manner, not really concerned about
                // the presence of this triple in most normal cases
                use target_lexicon as tl;
                match s {
                    "avr-unknown-gnu-atmega328" => tl::Triple {
                        architecture: tl::Architecture::Avr,
                        vendor: tl::Vendor::Unknown,
                        operating_system: tl::OperatingSystem::Unknown,
                        environment: tl::Environment::Unknown,
                        binary_format: tl::BinaryFormat::Unknown,
                    },
                    triple => match triple.parse() {
                        Ok(l) => l,
                        Err(e) => panic!("failed to parse '{}': {:?}", triple, e),
                    },
                }
            },
        }
    }
}

macro_rules! tg_match {
    ($pred:expr, $target:expr) => {
        match $pred {
            Predicate::Target(tg) => {
                let tinfo = tg.matches($target.builtin);

                #[cfg(feature = "targets")]
                {
                    let linfo = tg.matches(&$target.lexicon);
                    assert_eq!(
                        tinfo, linfo,
                        "{:#?} builtin didn't match lexicon {:#?} for predicate {:#?}",
                        $target.builtin, $target.lexicon, tg,
                    );

                    return linfo;
                }

                #[cfg(not(feature = "targets"))]
                tinfo
            }
            _ => panic!("not a target predicate"),
        }
    };

    ($pred:expr, $target:expr, $feats:expr) => {
        match $pred {
            Predicate::Target(tg) => {
                let tinfo = tg.matches($target.builtin);

                #[cfg(feature = "targets")]
                {
                    let linfo = tg.matches(&$target.lexicon);
                    assert_eq!(
                        tinfo, linfo,
                        "{:#?} builtin didn't match lexicon {:#?} for predicate {:#?}",
                        $target.builtin, $target.lexicon, tg,
                    );

                    return linfo;
                }

                #[cfg(not(feature = "targets"))]
                tinfo
            }
            Predicate::TargetFeature(feat) => $feats.iter().find(|f| *f == feat).is_some(),
            _ => panic!("not a target predicate"),
        }
    };
}

#[test]
fn target_family() {
    let matches_any_family = Expression::parse("any(unix, target_family = \"windows\")").unwrap();
    let impossible = Expression::parse("all(windows, target_family = \"unix\")").unwrap();

    for target in all {
        let target = Target::make(target.triple);
        match target.builtin.family {
            Some(_) => {
                assert!(matches_any_family.eval(|pred| { tg_match!(pred, target) }));
                assert!(!impossible.eval(|pred| { tg_match!(pred, target) }));
            }
            None => {
                assert!(!matches_any_family.eval(|pred| { tg_match!(pred, target) }));
                assert!(!impossible.eval(|pred| { tg_match!(pred, target) }));
            }
        }
    }
}

#[test]
fn tiny() {
    assert!(Expression::parse("all()").unwrap().eval(|_| false));
    assert!(!Expression::parse("any()").unwrap().eval(|_| true));
    assert!(!Expression::parse("not(all())").unwrap().eval(|_| false));
    assert!(Expression::parse("not(any())").unwrap().eval(|_| true));
    assert!(Expression::parse("all(not(blah))").unwrap().eval(|_| false));
    assert!(!Expression::parse("any(not(blah))").unwrap().eval(|_| true));
}

#[test]
fn very_specific() {
    let specific = Expression::parse(
        r#"all(
            target_os = "windows",
            target_arch = "x86",
            windows,
            target_env = "msvc",
            target_feature = "fxsr",
            target_feature = "sse",
            target_feature = "sse2",
            target_pointer_width = "32",
            target_endian = "little",
            not(target_vendor = "uwp"),
        )"#,
    )
    .unwrap();

    for target in all {
        let t = Target::make(target.triple);
        assert_eq!(
            target.triple == "i686-pc-windows-msvc" || target.triple == "i586-pc-windows-msvc",
            specific.eval(|pred| { tg_match!(pred, t, &["fxsr", "sse", "sse2"]) }),
            "expected true for i686-pc-windows-msvc, but got true for {}",
            target.triple,
        );
    }

    for target in all {
        let expr = format!(
            r#"cfg(
            all(
                target_arch = "{}",
                {}
                {}
                target_env = "{}"
            )
        )"#,
            target.arch.0,
            if let Some(v) = target.vendor {
                format!(r#"target_vendor = "{}","#, v.0)
            } else {
                "".to_owned()
            },
            if let Some(v) = target.os {
                format!(r#"target_os = "{}","#, v.0)
            } else {
                "".to_owned()
            },
            target.env.map(|e| e.0).unwrap_or_else(|| ""),
        );

        let specific = Expression::parse(&expr).unwrap();

        let t = Target::make(target.triple);
        assert!(
            specific.eval(|pred| { tg_match!(pred, t) }),
            "failed expression '{}' for {:#?}",
            expr,
            t.builtin,
        );
    }
}

#[test]
fn complex() {
    let complex = Expression::parse(r#"cfg(all(unix, not(any(target_os="macos", target_os="android", target_os="emscripten"))))"#).unwrap();

    // Should match linuxes
    let linux_gnu = Target::make("x86_64-unknown-linux-gnu");
    let linux_musl = Target::make("x86_64-unknown-linux-musl");

    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));

    // Should *not* match windows or mac or android
    let windows_msvc = Target::make("x86_64-pc-windows-msvc");
    let mac = Target::make("x86_64-apple-darwin");
    let android = Target::make("aarch64-linux-android");

    assert!(!complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(!complex.eval(|pred| tg_match!(pred, mac)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));

    let complex =
        Expression::parse(r#"all(not(target_os = "ios"), not(target_os = "android"))"#).unwrap();

    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));
    assert!(complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(complex.eval(|pred| tg_match!(pred, mac)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));

    let complex = Expression::parse(r#"all(any(unix, target_arch="x86"), not(any(target_os="android", target_os="emscripten")))"#).unwrap();

    // Should match linuxes and mac
    assert!(complex.eval(|pred| tg_match!(pred, linux_gnu)));
    assert!(complex.eval(|pred| tg_match!(pred, linux_musl)));
    assert!(complex.eval(|pred| tg_match!(pred, mac)));

    // Should *not* match x86_64 windows or android
    assert!(!complex.eval(|pred| tg_match!(pred, windows_msvc)));
    assert!(!complex.eval(|pred| tg_match!(pred, android)));
}

#[test]
fn features() {
    let enabled = ["good", "bad", "ugly"];

    let many_features = Expression::parse(
        r#"all(feature = "good", feature = "bad", feature = "ugly", not(feature = "nope"))"#,
    )
    .unwrap();

    assert!(many_features.eval(|pred| {
        match pred {
            Predicate::Feature(name) => enabled.contains(name),
            _ => false,
        }
    }));

    let feature_and_target_feature =
        Expression::parse(r#"all(feature = "make_fast", target_feature = "sse4.2")"#).unwrap();

    assert!(feature_and_target_feature.eval(|pred| {
        match pred {
            Predicate::Feature(name) => *name == "make_fast",
            Predicate::TargetFeature(feat) => *feat == "sse4.2",
            _ => false,
        }
    }));

    assert_eq!(
        feature_and_target_feature.eval(|pred| {
            match pred {
                Predicate::Feature(_) => Some(false),
                Predicate::TargetFeature(_) => None,
                _ => panic!("unexpected predicate"),
            }
        }),
        Some(false),
        "all() with Some(false) and None evaluates to Some(false)"
    );

    assert_eq!(
        feature_and_target_feature.eval(|pred| {
            match pred {
                Predicate::Feature(_) => Some(true),
                Predicate::TargetFeature(_) => None,
                _ => panic!("unexpected predicate"),
            }
        }),
        None,
        "all() with Some(true) and None evaluates to None"
    );
}
