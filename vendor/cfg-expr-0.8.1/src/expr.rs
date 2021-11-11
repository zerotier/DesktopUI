pub mod lexer;
mod parser;

use smallvec::SmallVec;
use std::ops::Range;

/// A predicate function, used to combine 1 or more predicates
/// into a single value
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Func {
    /// `not()` with a configuration predicate. It is true if its predicate
    /// is false and false if its predicate is true.
    Not,
    /// `all()` with a comma separated list of configuration predicates. It
    /// is false if at least one predicate is false. If there are no predicates,
    /// it is true.
    ///
    /// The associated `usize` is the number of predicates inside the `all()`.
    All(usize),
    /// `any()` with a comma separated list of configuration predicates. It
    /// is true if at least one predicate is true. If there are no predicates,
    /// it is false.
    ///
    /// The associated `usize` is the number of predicates inside the `any()`.
    Any(usize),
}

use crate::targets as targ;

/// All predicates that pertains to a target, except for `target_feature`
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TargetPredicate<'a> {
    /// [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    Arch(targ::Arch<'a>),
    /// [target_endian](https://doc.rust-lang.org/reference/conditional-compilation.html#target_endian)
    Endian(targ::Endian),
    /// [target_env](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env)
    Env(targ::Env<'a>),
    /// [target_family](https://doc.rust-lang.org/reference/conditional-compilation.html#target_family)
    /// This also applies to the bare [`unix` and `windows`](https://doc.rust-lang.org/reference/conditional-compilation.html#unix-and-windows)
    /// predicates.
    Family(targ::Family),
    /// [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    Os(targ::Os<'a>),
    /// [target_pointer_width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width)
    PointerWidth(u8),
    /// [target_vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    Vendor(targ::Vendor<'a>),
}

pub trait TargetMatcher {
    fn matches(&self, tp: TargetPredicate<'_>) -> bool;
}

impl<'a> TargetMatcher for targ::TargetInfo<'a> {
    fn matches(&self, tp: TargetPredicate<'_>) -> bool {
        use TargetPredicate::{Arch, Endian, Env, Family, Os, PointerWidth, Vendor};

        match tp {
            Arch(a) => a == self.arch,
            Endian(end) => end == self.endian,
            // The environment is allowed to be an empty string
            Env(env) => match self.env {
                Some(e) => env == e,
                None => env.0.is_empty(),
            },
            Family(fam) => Some(fam) == self.family,
            Os(os) => Some(os) == self.os,
            PointerWidth(w) => w == self.pointer_width,
            Vendor(ven) => match self.vendor {
                Some(v) => ven == v,
                None => ven.0 == "unknown",
            },
        }
    }
}

#[cfg(feature = "targets")]
impl TargetMatcher for target_lexicon::Triple {
    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::match_same_arms)]
    fn matches(&self, tp: TargetPredicate<'_>) -> bool {
        use target_lexicon::*;
        use TargetPredicate::{Arch, Endian, Env, Family, Os, PointerWidth, Vendor};

        match tp {
            Arch(arch) => {
                if arch.0 == "x86" {
                    matches!(self.architecture, Architecture::X86_32(_))
                } else if arch.0 == "wasm32" {
                    self.architecture == Architecture::Wasm32
                        || self.architecture == Architecture::Asmjs
                } else if arch.0 == "arm" {
                    matches!(self.architecture, Architecture::Arm(_))
                } else {
                    match arch.0.parse::<Architecture>() {
                        Ok(a) => match (self.architecture, a) {
                            (Architecture::Aarch64(_), Architecture::Aarch64(_))
                            | (Architecture::Mips32(_), Architecture::Mips32(_))
                            | (Architecture::Mips64(_), Architecture::Mips64(_))
                            | (Architecture::Powerpc64le, Architecture::Powerpc64)
                            | (Architecture::Riscv32(_), Architecture::Riscv32(_))
                            | (Architecture::Riscv64(_), Architecture::Riscv64(_))
                            | (Architecture::Sparcv9, Architecture::Sparc64) => true,
                            (a, b) => a == b,
                        },
                        Err(_) => false,
                    }
                }
            }
            Endian(end) => match self.architecture.endianness() {
                Ok(endian) => matches!(
                    (end, endian),
                    (crate::targets::Endian::little, Endianness::Little)
                        | (crate::targets::Endian::big, Endianness::Big)
                ),

                Err(_) => false,
            },
            Env(env) => {
                // The environment is implied by some operating systems
                match self.operating_system {
                    OperatingSystem::Redox => env.0 == "relibc",
                    OperatingSystem::VxWorks => env.0 == "gnu",
                    OperatingSystem::Freebsd => match self.architecture {
                        Architecture::Arm(ArmArchitecture::Armv6)
                        | Architecture::Arm(ArmArchitecture::Armv7) => env.0 == "gnueabihf",
                        _ => env.0.is_empty(),
                    },
                    OperatingSystem::Netbsd => match self.architecture {
                        Architecture::Arm(ArmArchitecture::Armv6)
                        | Architecture::Arm(ArmArchitecture::Armv7) => env.0 == "eabihf",
                        _ => env.0.is_empty(),
                    },
                    OperatingSystem::None_
                    | OperatingSystem::Cloudabi
                    | OperatingSystem::Hermit
                    | OperatingSystem::Ios => match self.environment {
                        Environment::LinuxKernel => env.0 == "gnu",
                        _ => env.0.is_empty(),
                    },
                    _ => {
                        if env.0.is_empty() {
                            matches!(
                                self.environment,
                                Environment::Unknown
                                    | Environment::Android
                                    | Environment::Softfloat
                                    | Environment::Androideabi
                                    | Environment::Eabi
                            )
                        } else {
                            match env.0.parse::<Environment>() {
                                Ok(e) => {
                                    // Rustc shortens multiple "gnu*" environments to just "gnu"
                                    if env.0 == "gnu" {
                                        match self.environment {
                                            Environment::Gnu
                                            | Environment::Gnuabi64
                                            | Environment::Gnueabi
                                            | Environment::Gnuspe
                                            | Environment::Gnux32
                                            | Environment::GnuIlp32
                                            | Environment::Gnueabihf => true,
                                            // Rust 1.49.0 changed all android targets to have the
                                            // gnu environment
                                            Environment::Android | Environment::Androideabi
                                                if self.operating_system
                                                    == OperatingSystem::Linux =>
                                            {
                                                true
                                            }
                                            Environment::Kernel => {
                                                self.operating_system == OperatingSystem::Linux
                                            }
                                            _ => false,
                                        }
                                    } else if env.0 == "musl" {
                                        matches!(
                                            self.environment,
                                            Environment::Musl
                                                | Environment::Musleabi
                                                | Environment::Musleabihf
                                                | Environment::Muslabi64
                                        )
                                    } else if env.0 == "uclibc" {
                                        matches!(
                                            self.environment,
                                            Environment::Uclibc | Environment::Uclibceabi
                                        )
                                    } else {
                                        self.environment == e
                                    }
                                }
                                Err(_) => false,
                            }
                        }
                    }
                }
            }
            Family(fam) => {
                use target_lexicon::OperatingSystem::{
                    AmdHsa, Bitrig, Cloudabi, Cuda, Darwin, Dragonfly, Emscripten, Freebsd,
                    Fuchsia, Haiku, Hermit, Illumos, Ios, L4re, Linux, MacOSX, Nebulet, Netbsd,
                    None_, Openbsd, Redox, Solaris, Tvos, Uefi, Unknown, VxWorks, Wasi, Windows,
                };
                Some(fam)
                    == match self.operating_system {
                        Unknown | AmdHsa | Bitrig | Cloudabi | Cuda | Hermit | Nebulet | None_
                        | Uefi | Wasi => None,
                        Darwin
                        | Dragonfly
                        | Emscripten
                        | Freebsd
                        | Fuchsia
                        | Haiku
                        | Illumos
                        | Ios
                        | L4re
                        | MacOSX { .. }
                        | Netbsd
                        | Openbsd
                        | Redox
                        | Solaris
                        | Tvos
                        | VxWorks => Some(crate::targets::Family::unix),
                        Linux => {
                            // The 'kernel' environment is treated specially as not-unix
                            if self.environment != Environment::Kernel {
                                Some(crate::targets::Family::unix)
                            } else {
                                None
                            }
                        }
                        Windows => Some(crate::targets::Family::windows),
                        // I really dislike non-exhaustive :(
                        _ => None,
                    }
            }
            Os(os) => match os.0.parse::<OperatingSystem>() {
                Ok(o) => match self.environment {
                    Environment::HermitKernel => os.0 == "hermit",
                    _ => self.operating_system == o,
                },
                Err(_) => {
                    // Handle special case for darwin/macos, where the triple is
                    // "darwin", but rustc identifies the OS as "macos"
                    if os.0 == "macos" && self.operating_system == OperatingSystem::Darwin {
                        true
                    } else {
                        // For android, the os is still linux, but the environment is android
                        os.0 == "android"
                            && self.operating_system == OperatingSystem::Linux
                            && (self.environment == Environment::Android
                                || self.environment == Environment::Androideabi)
                    }
                }
            },
            Vendor(ven) => match ven.0.parse::<target_lexicon::Vendor>() {
                Ok(v) => self.vendor == v,
                Err(_) => false,
            },
            PointerWidth(pw) => {
                // The gnux32 environment is a special case, where it has an
                // x86_64 architecture, but a 32-bit pointer width
                if !matches!(
                    self.environment,
                    Environment::Gnux32 | Environment::GnuIlp32
                ) {
                    pw == match self.pointer_width() {
                        Ok(pw) => pw.bits(),
                        Err(_) => return false,
                    }
                } else {
                    pw == 32
                }
            }
        }
    }
}

impl<'a> TargetPredicate<'a> {
    /// Returns true of the predicate matches the specified target
    ///
    /// ```
    /// use cfg_expr::{targets::*, expr::TargetPredicate as tp};
    /// let win = get_builtin_target_by_triple("x86_64-pc-windows-msvc").unwrap();
    ///
    /// assert!(
    ///     tp::Arch(Arch::x86_64).matches(win) &&
    ///     tp::Endian(Endian::little).matches(win) &&
    ///     tp::Env(Env::msvc).matches(win) &&
    ///     tp::Family(Family::windows).matches(win) &&
    ///     tp::Os(Os::windows).matches(win) &&
    ///     tp::PointerWidth(64).matches(win) &&
    ///     tp::Vendor(Vendor::pc).matches(win)
    /// );
    /// ```
    pub fn matches<T>(self, target: &T) -> bool
    where
        T: TargetMatcher,
    {
        target.matches(self)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Which {
    Arch,
    Endian(targ::Endian),
    Env,
    Family(targ::Family),
    Os,
    PointerWidth(u8),
    Vendor,
}

#[derive(Clone, Debug)]
pub(crate) struct InnerTarget {
    which: Which,
    span: Option<Range<usize>>,
}

/// A single predicate in a `cfg()` expression
#[derive(Debug, PartialEq)]
pub enum Predicate<'a> {
    /// A target predicate, with the `target_` prefix
    Target(TargetPredicate<'a>),
    /// Whether rustc's test harness is [enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#test)
    Test,
    /// [Enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions)
    /// when compiling without optimizations.
    DebugAssertions,
    /// [Enabled](https://doc.rust-lang.org/reference/conditional-compilation.html#proc_macro) for
    /// crates of the proc_macro type.
    ProcMacro,
    /// A [`feature = "<name>"`](https://doc.rust-lang.org/nightly/cargo/reference/features.html)
    Feature(&'a str),
    /// [target_feature](https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature)
    TargetFeature(&'a str),
    /// A generic bare predicate key that doesn't match one of the known options, eg `cfg(bare)`
    Flag(&'a str),
    /// A generic key = "value" predicate that doesn't match one of the known options, eg `cfg(foo = "bar")`
    KeyValue { key: &'a str, val: &'a str },
}

#[derive(Clone, Debug)]
pub(crate) enum InnerPredicate {
    Target(InnerTarget),
    Test,
    DebugAssertions,
    ProcMacro,
    Feature(Range<usize>),
    TargetFeature(Range<usize>),
    Other {
        identifier: Range<usize>,
        value: Option<Range<usize>>,
    },
}

impl InnerPredicate {
    fn to_pred<'a>(&self, s: &'a str) -> Predicate<'a> {
        use InnerPredicate as IP;
        use Predicate::{
            DebugAssertions, Feature, Flag, KeyValue, ProcMacro, Target, TargetFeature, Test,
        };

        match self {
            IP::Target(it) => match &it.which {
                Which::Arch => Target(TargetPredicate::Arch(targ::Arch(
                    &s[it.span.clone().unwrap()],
                ))),
                Which::Os => Target(TargetPredicate::Os(targ::Os(&s[it.span.clone().unwrap()]))),
                Which::Vendor => Target(TargetPredicate::Vendor(targ::Vendor(
                    &s[it.span.clone().unwrap()],
                ))),
                Which::Env => Target(TargetPredicate::Env(targ::Env(
                    &s[it.span.clone().unwrap()],
                ))),
                Which::Endian(end) => Target(TargetPredicate::Endian(*end)),
                Which::Family(fam) => Target(TargetPredicate::Family(*fam)),
                Which::PointerWidth(pw) => Target(TargetPredicate::PointerWidth(*pw)),
            },
            IP::Test => Test,
            IP::DebugAssertions => DebugAssertions,
            IP::ProcMacro => ProcMacro,
            IP::Feature(rng) => Feature(&s[rng.clone()]),
            IP::TargetFeature(rng) => TargetFeature(&s[rng.clone()]),
            IP::Other { identifier, value } => match value {
                Some(vs) => KeyValue {
                    key: &s[identifier.clone()],
                    val: &s[vs.clone()],
                },
                None => Flag(&s[identifier.clone()]),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ExprNode {
    Fn(Func),
    Predicate(InnerPredicate),
}

/// A parsed `cfg()` expression that can evaluated
#[derive(Clone, Debug)]
pub struct Expression {
    pub(crate) expr: SmallVec<[ExprNode; 5]>,
    // We keep the original string around for providing the arbitrary
    // strings that can make up an expression
    pub(crate) original: String,
}

impl Expression {
    /// An iterator over each predicate in the expression
    pub fn predicates(&self) -> impl Iterator<Item = Predicate<'_>> {
        self.expr.iter().filter_map(move |item| match item {
            ExprNode::Predicate(pred) => {
                let pred = pred.clone().to_pred(&self.original);
                Some(pred)
            }
            ExprNode::Fn(_) => None,
        })
    }

    /// Evaluates the expression, using the provided closure to determine the value of
    /// each predicate, which are then combined into a final result depending on the
    /// functions not(), all(), or any() in the expression.
    ///
    /// `eval_predicate` typically returns `bool`, but may return any type that implements
    /// the `Logic` trait.
    ///
    /// ## Examples
    ///
    /// ```
    /// use cfg_expr::{targets::*, Expression, Predicate};
    ///
    /// let linux_musl = get_builtin_target_by_triple("x86_64-unknown-linux-musl").unwrap();
    ///
    /// let expr = Expression::parse(r#"all(not(windows), target_env = "musl", any(target_arch = "x86", target_arch = "x86_64"))"#).unwrap();
    ///
    /// assert!(expr.eval(|pred| {
    ///     match pred {
    ///         Predicate::Target(tp) => tp.matches(linux_musl),
    ///         _ => false,
    ///     }
    /// }));
    /// ```
    ///
    /// Returning `Option<bool>`, where `None` indicates the result is unknown:
    ///
    /// ```
    /// use cfg_expr::{targets::*, Expression, Predicate};
    ///
    /// let expr = Expression::parse(r#"any(target_feature = "sse2", target_env = "musl")"#).unwrap();
    ///
    /// let linux_gnu = get_builtin_target_by_triple("x86_64-unknown-linux-gnu").unwrap();
    /// let linux_musl = get_builtin_target_by_triple("x86_64-unknown-linux-musl").unwrap();
    ///
    /// fn eval(expr: &Expression, target: &TargetInfo) -> Option<bool> {
    ///     expr.eval(|pred| {
    ///         match pred {
    ///             Predicate::Target(tp) => Some(tp.matches(target)),
    ///             Predicate::TargetFeature(_) => None,
    ///             _ => panic!("unexpected predicate"),
    ///         }
    ///     })
    /// }
    ///
    /// // Whether the target feature is present is unknown, so the whole expression evaluates to
    /// // None (unknown).
    /// assert_eq!(eval(&expr, linux_gnu), None);
    ///
    /// // Whether the target feature is present is irrelevant for musl, since the any() always
    /// // evaluates to true.
    /// assert_eq!(eval(&expr, linux_musl), Some(true));
    /// ```
    pub fn eval<EP, T>(&self, mut eval_predicate: EP) -> T
    where
        EP: FnMut(&Predicate<'_>) -> T,
        T: Logic + std::fmt::Debug,
    {
        let mut result_stack = SmallVec::<[T; 8]>::new();

        // We store the expression as postfix, so just evaluate each license
        // requirement in the order it comes, and then combining the previous
        // results according to each operator as it comes
        for node in self.expr.iter() {
            match node {
                ExprNode::Predicate(pred) => {
                    let pred = pred.to_pred(&self.original);

                    result_stack.push(eval_predicate(&pred));
                }
                ExprNode::Fn(Func::All(count)) => {
                    // all() with a comma separated list of configuration predicates.
                    let mut result = T::top();

                    for _ in 0..*count {
                        let r = result_stack.pop().unwrap();
                        result = result.and(r);
                    }

                    result_stack.push(result);
                }
                ExprNode::Fn(Func::Any(count)) => {
                    // any() with a comma separated list of configuration predicates.
                    let mut result = T::bottom();

                    for _ in 0..*count {
                        let r = result_stack.pop().unwrap();
                        result = result.or(r);
                    }

                    result_stack.push(result);
                }
                ExprNode::Fn(Func::Not) => {
                    // not() with a configuration predicate.
                    // It is true if its predicate is false
                    // and false if its predicate is true.
                    let r = result_stack.pop().unwrap();
                    result_stack.push(r.not());
                }
            }
        }

        result_stack.pop().unwrap()
    }

    /// The original string which has been parsed to produce this ['Expression`].
    ///
    /// ```
    /// use cfg_expr::Expression;
    ///
    /// assert_eq!(
    ///     Expression::parse("any()").unwrap().original(),
    ///     "any()"
    /// );
    /// ```
    #[inline]
    pub fn original(&self) -> &str {
        &self.original
    }
}

/// [`PartialEq`] will do a **syntactical** comparaison, so will just check if both
/// expressions have been parsed from the same string, **not** if they are semantically
/// equivalent.
///
/// ```
/// use cfg_expr::Expression;
///
/// assert_eq!(
///     Expression::parse("any()").unwrap(),
///     Expression::parse("any()").unwrap()
/// );
/// assert_ne!(
///     Expression::parse("any()").unwrap(),
///     Expression::parse("unix").unwrap()
/// );
/// ```
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.original.eq(&other.original)
    }
}

/// A propositional logic used to evaluate `Expression` instances.
///
/// An `Expression` consists of some predicates and the `any`, `all` and `not` operators. An
/// implementation of `Logic` defines how the `any`, `all` and `not` operators should be evaluated.
pub trait Logic {
    /// The result of an `all` operation with no operands, akin to Boolean `true`.
    fn top() -> Self;

    /// The result of an `any` operation with no operands, akin to Boolean `false`.
    fn bottom() -> Self;

    /// `AND`, which corresponds to the `all` operator.
    fn and(self, other: Self) -> Self;

    /// `OR`, which corresponds to the `any` operator.
    fn or(self, other: Self) -> Self;

    /// `NOT`, which corresponds to the `not` operator.
    fn not(self) -> Self;
}

/// A boolean logic.
impl Logic for bool {
    #[inline]
    fn top() -> Self {
        true
    }

    #[inline]
    fn bottom() -> Self {
        false
    }

    #[inline]
    fn and(self, other: Self) -> Self {
        self && other
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        self || other
    }

    #[inline]
    fn not(self) -> Self {
        !self
    }
}

/// A three-valued logic -- `None` stands for the value being unknown.
///
/// The truth tables for this logic are described on
/// [Wikipedia](https://en.wikipedia.org/wiki/Three-valued_logic#Kleene_and_Priest_logics).
impl Logic for Option<bool> {
    #[inline]
    fn top() -> Self {
        Some(true)
    }

    #[inline]
    fn bottom() -> Self {
        Some(false)
    }

    #[inline]
    fn and(self, other: Self) -> Self {
        match (self, other) {
            // If either is false, the expression is false.
            (Some(false), _) | (_, Some(false)) => Some(false),
            // If both are true, the expression is true.
            (Some(true), Some(true)) => Some(true),
            // One or both are unknown -- the result is unknown.
            _ => None,
        }
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        match (self, other) {
            // If either is true, the expression is true.
            (Some(true), _) | (_, Some(true)) => Some(true),
            // If both are false, the expression is false.
            (Some(false), Some(false)) => Some(false),
            // One or both are unknown -- the result is unknown.
            _ => None,
        }
    }

    #[inline]
    fn not(self) -> Self {
        self.map(|v| !v)
    }
}
