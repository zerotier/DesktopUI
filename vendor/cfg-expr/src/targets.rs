use crate::error::Reason;

mod builtins;

/// A list of all of the [builtin](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/spec/index.html#modules)
/// targets known to rustc, as of 1.49.0
pub use builtins::ALL_BUILTINS;

/// The "architecture" field
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Arch<'a>(pub &'a str);

/// The "vendor" field, which in practice is little more than an arbitrary modifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vendor<'a>(pub &'a str);

/// The "operating system" field, which sometimes implies an environment, and
/// sometimes isn't an actual operating system.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Os<'a>(pub &'a str);

/// The "environment" field, which specifies an ABI environment on top of the
/// operating system. In many configurations, this field is omitted, and the
/// environment is implied by the operating system.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Env<'a>(pub &'a str);

macro_rules! target_enum {
    (
        $(#[$outer:meta])*
        pub enum $kind:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $name:ident $(= $value:expr)?,
            )+
        }
    ) => {
        $(#[$outer])*
        #[allow(non_camel_case_types)]
        pub enum $kind {
            $(
                $(#[$inner $($args)*])*
                $name $(= $value)?,
            )+
        }

        impl_from_str! {
            $kind {
                $(
                    $(#[$inner $($args)*])*
                    $name $(= $value)?,
                )+
            }
        }
    };
}

macro_rules! impl_from_str {
    (
        $kind:ident {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $name:ident $(= $value:expr)?,
            )+
        }
    ) => {
        impl std::str::FromStr for $kind {
            type Err = Reason;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($name) => Ok(Self::$name),)+
                    _ => Err(Reason::Unexpected(&[$(stringify!($name),)+])),
                }
            }
        }
    };
}

target_enum! {
    /// The endian types known to rustc
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum Endian {
        big,
        little,
    }
}

target_enum! {
    /// All of the target families known to rustc
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum Family {
        /// Everything that isn't windows, and has a family!
        unix,
        /// The lone wolf of target families.
        windows,
    }
}

/// Contains information regarding a particular target known to rustc
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetInfo<'a> {
    /// The target's unique identifier
    pub triple: &'a str,
    /// The target's operating system, if any. Used by the
    /// [target_os](https://doc.rust-lang.org/reference/conditional-compilation.html#target_os)
    /// predicate.
    pub os: Option<Os<'a>>,
    /// The target's CPU architecture. Used by the
    /// [target_arch](https://doc.rust-lang.org/reference/conditional-compilation.html#target_arch)
    /// predicate.
    pub arch: Arch<'a>,
    /// The target's ABI/libc used, if any. Used by the
    /// [target_env](https://doc.rust-lang.org/reference/conditional-compilation.html#target_env)
    /// predicate.
    pub env: Option<Env<'a>>,
    /// The target's vendor, if any. Used by the
    /// [target_vendor](https://doc.rust-lang.org/reference/conditional-compilation.html#target_vendor)
    /// predicate.
    pub vendor: Option<Vendor<'a>>,
    /// The target's family, if any. Used by the
    /// [target_family](https://doc.rust-lang.org/reference/conditional-compilation.html#target_family)
    /// predicate.
    pub family: Option<Family>,
    /// The size of the target's pointer type. Used by the
    /// [target_pointer_width](https://doc.rust-lang.org/reference/conditional-compilation.html#target_pointer_width)
    /// predicate.
    pub pointer_width: u8,
    /// The target's endianness. Used by the
    /// [target_endian](https://doc.rust-lang.org/reference/conditional-compilation.html#target_endian)
    /// predicate.
    pub endian: Endian,
}

/// Attempts to find the `TargetInfo` for the specified target triple
///
/// ```
/// assert!(cfg_expr::targets::get_builtin_target_by_triple("x86_64-unknown-linux-musl").is_some());
/// ```
pub fn get_builtin_target_by_triple(triple: &str) -> Option<&'static TargetInfo<'static>> {
    ALL_BUILTINS
        .binary_search_by(|ti| ti.triple.cmp(triple))
        .map(|i| &ALL_BUILTINS[i])
        .ok()
}

/// Retrieves the version of rustc for which the built-in targets were
/// retrieved from. Targets may be added and removed between different rustc
/// versions.
///
/// ```
/// assert_eq!("1.53.0", cfg_expr::targets::rustc_version());
/// ```
pub fn rustc_version() -> &'static str {
    builtins::RUSTC_VERSION
}

#[cfg(test)]
mod test {
    use crate::targets::get_builtin_target_by_triple;
    use std::collections::{BTreeSet, HashSet};

    // rustc's target-list is currently sorted lexicographically
    // by the target-triple, so ensure that stays the case
    #[test]
    fn targets_are_sorted() {
        for window in super::ALL_BUILTINS.windows(2) {
            assert!(window[0].triple < window[1].triple);
        }
    }

    // Ensure our workaround for https://github.com/rust-lang/rust/issues/36156
    // still functions
    #[test]
    fn has_ios() {
        assert_eq!(
            8,
            super::ALL_BUILTINS
                .iter()
                .filter(|ti| ti.os == Some(super::Os::ios))
                .count()
        );
    }

    // Ensure that TargetInfo can be used as keys for btree and hash-based data structures.
    #[test]
    fn set_map_key() {
        let target_info =
            get_builtin_target_by_triple("x86_64-unknown-linux-gnu").expect("known target");

        let mut btree_set = BTreeSet::new();
        btree_set.insert(target_info);

        let mut hash_set = HashSet::new();
        hash_set.insert(target_info);
    }
}
