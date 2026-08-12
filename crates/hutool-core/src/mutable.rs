//! Mutable value containers corresponding to Hutool's `core.lang.mutable` package.
//!
//! Rust normally prefers ordinary mutable bindings. These small wrappers are
//! useful when mutation itself must be passed around as a value, while keeping
//! ownership and borrowing explicit.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_lossless
)]

/// 对齐: `cn.hutool.core.lang.mutable.MutableObj`
/// 可变对象
use std::{
    cmp::Ordering,
    convert::Infallible,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

/// Shared contract for owned mutable value wrappers.
pub trait Mutable<T> {
    /// Borrows the current value.
    fn get(&self) -> &T;

    /// Mutably borrows the current value.
    fn get_mut(&mut self) -> &mut T;

    /// Replaces the current value.
    fn set(&mut self, value: T);
}

/// Generic owned mutable value.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutableObj<T> {
    value: T,
}

impl<T> MutableObj<T> {
    /// Creates a mutable object around `value`.
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Hutool-style factory alias.
    pub const fn of(value: T) -> Self {
        Self::new(value)
    }

    /// Borrows the wrapped value.
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Mutably borrows the wrapped value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Replaces the wrapped value.
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Consumes the wrapper and returns its value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Mutable<T> for MutableObj<T> {
    fn get(&self) -> &T {
        self.get()
    }

    fn get_mut(&mut self) -> &mut T {
        self.get_mut()
    }

    fn set(&mut self, value: T) {
        self.set(value);
    }
}

impl<T> From<T> for MutableObj<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: fmt::Display> fmt::Display for MutableObj<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

macro_rules! mutable_integer {
    ($name:ident, $primitive:ty, $java_hash:expr) => {
        #[doc = concat!("Mutable `", stringify!($primitive), "` value.")]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            value: $primitive,
        }

        impl $name {
            /// Creates a mutable numeric value.
            pub const fn new(value: $primitive) -> Self {
                Self { value }
            }

            /// Returns the current primitive value.
            pub const fn get(&self) -> $primitive {
                self.value
            }

            /// Replaces the current value.
            pub fn set(&mut self, value: $primitive) {
                self.value = value;
            }

            /// Adds one using Java-compatible wrapping integer arithmetic.
            pub fn increment(&mut self) -> &mut Self {
                self.value = self.value.wrapping_add(1);
                self
            }

            /// Subtracts one using Java-compatible wrapping integer arithmetic.
            pub fn decrement(&mut self) -> &mut Self {
                self.value = self.value.wrapping_sub(1);
                self
            }

            /// Adds an operand using Java-compatible wrapping arithmetic.
            pub fn add(&mut self, operand: $primitive) -> &mut Self {
                self.value = self.value.wrapping_add(operand);
                self
            }

            /// Subtracts an operand using Java-compatible wrapping arithmetic.
            pub fn subtract(&mut self, operand: $primitive) -> &mut Self {
                self.value = self.value.wrapping_sub(operand);
                self
            }

            /// Returns the Java `byteValue` conversion.
            pub const fn byte_value(&self) -> i8 {
                self.value as i8
            }

            /// Returns the Java `shortValue` conversion.
            pub const fn short_value(&self) -> i16 {
                self.value as i16
            }

            /// Returns the Java `intValue` conversion.
            pub const fn int_value(&self) -> i32 {
                self.value as i32
            }

            /// Returns the Java `longValue` conversion.
            pub const fn long_value(&self) -> i64 {
                self.value as i64
            }

            /// Returns the Java `floatValue` conversion.
            pub const fn float_value(&self) -> f32 {
                self.value as f32
            }

            /// Returns the Java `doubleValue` conversion.
            pub const fn double_value(&self) -> f64 {
                self.value as f64
            }

            /// Returns the exact Java wrapper hash code used by Hutool.
            pub fn java_hash_code(&self) -> i32 {
                ($java_hash)(self.value)
            }

            /// Consumes the wrapper and returns its value.
            pub const fn into_inner(self) -> $primitive {
                self.value
            }
        }

        impl Mutable<$primitive> for $name {
            fn get(&self) -> &$primitive {
                &self.value
            }

            fn get_mut(&mut self) -> &mut $primitive {
                &mut self.value
            }

            fn set(&mut self, value: $primitive) {
                self.value = value;
            }
        }

        impl From<$primitive> for $name {
            fn from(value: $primitive) -> Self {
                Self::new(value)
            }
        }

        impl FromStr for $name {
            type Err = <$primitive as FromStr>::Err;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                value.parse().map(Self::new)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.value.fmt(formatter)
            }
        }
    };
}

mutable_integer!(MutableByte, i8, i32::from);
mutable_integer!(MutableShort, i16, i32::from);
mutable_integer!(MutableInt, i32, |value| value);
mutable_integer!(MutableLong, i64, java_i64_hash);

fn java_i64_hash(value: i64) -> i32 {
    let bits = u64::from_ne_bytes(value.to_ne_bytes());
    let folded = bits ^ (bits >> 32);
    let bytes = folded.to_le_bytes();
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn canonical_f32_bits(value: f32) -> u32 {
    if value.is_nan() {
        0x7fc0_0000
    } else {
        value.to_bits()
    }
}

fn canonical_f64_bits(value: f64) -> u64 {
    if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}

fn java_f32_cmp(lhs: f32, rhs: f32) -> Ordering {
    lhs.partial_cmp(&rhs)
        .unwrap_or_else(|| {
            i32::from_ne_bytes(canonical_f32_bits(lhs).to_ne_bytes())
                .cmp(&i32::from_ne_bytes(canonical_f32_bits(rhs).to_ne_bytes()))
        })
        .then_with(|| {
            i32::from_ne_bytes(canonical_f32_bits(lhs).to_ne_bytes())
                .cmp(&i32::from_ne_bytes(canonical_f32_bits(rhs).to_ne_bytes()))
        })
}

fn java_f64_cmp(lhs: f64, rhs: f64) -> Ordering {
    lhs.partial_cmp(&rhs)
        .unwrap_or_else(|| {
            i64::from_ne_bytes(canonical_f64_bits(lhs).to_ne_bytes())
                .cmp(&i64::from_ne_bytes(canonical_f64_bits(rhs).to_ne_bytes()))
        })
        .then_with(|| {
            i64::from_ne_bytes(canonical_f64_bits(lhs).to_ne_bytes())
                .cmp(&i64::from_ne_bytes(canonical_f64_bits(rhs).to_ne_bytes()))
        })
}

macro_rules! mutable_float {
    ($name:ident, $primitive:ty, $bits:ty, $canonical:ident, $compare:ident) => {
        #[doc = concat!("Mutable `", stringify!($primitive), "` value with Java equality semantics.")]
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $name {
            value: $primitive,
        }

        impl $name {
            /// Creates a mutable floating-point value.
            pub const fn new(value: $primitive) -> Self {
                Self { value }
            }

            /// Returns the current primitive value.
            pub const fn get(&self) -> $primitive {
                self.value
            }

            /// Replaces the current value.
            pub fn set(&mut self, value: $primitive) {
                self.value = value;
            }

            /// Adds one.
            pub fn increment(&mut self) -> &mut Self {
                self.value += 1.0;
                self
            }

            /// Subtracts one.
            pub fn decrement(&mut self) -> &mut Self {
                self.value -= 1.0;
                self
            }

            /// Adds an operand.
            pub fn add(&mut self, operand: $primitive) -> &mut Self {
                self.value += operand;
                self
            }

            /// Subtracts an operand.
            pub fn subtract(&mut self, operand: $primitive) -> &mut Self {
                self.value -= operand;
                self
            }

            /// Returns the Java `intValue` conversion.
            pub fn int_value(&self) -> i32 {
                self.value as i32
            }

            /// Returns the Java `longValue` conversion.
            pub fn long_value(&self) -> i64 {
                self.value as i64
            }

            /// Returns the Java `floatValue` conversion.
            pub fn float_value(&self) -> f32 {
                self.value as f32
            }

            /// Returns the Java `doubleValue` conversion.
            pub fn double_value(&self) -> f64 {
                self.value as f64
            }

            /// Returns canonical Java floating-point bits.
            pub fn canonical_bits(&self) -> $bits {
                $canonical(self.value)
            }

            /// Consumes the wrapper and returns its value.
            pub const fn into_inner(self) -> $primitive {
                self.value
            }
        }

        impl Mutable<$primitive> for $name {
            fn get(&self) -> &$primitive {
                &self.value
            }

            fn get_mut(&mut self) -> &mut $primitive {
                &mut self.value
            }

            fn set(&mut self, value: $primitive) {
                self.value = value;
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.canonical_bits() == other.canonical_bits()
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                $compare(self.value, other.value)
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.canonical_bits().hash(state);
            }
        }

        impl From<$primitive> for $name {
            fn from(value: $primitive) -> Self {
                Self::new(value)
            }
        }

        impl FromStr for $name {
            type Err = <$primitive as FromStr>::Err;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                value.parse().map(Self::new)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.value.fmt(formatter)
            }
        }
    };
}

mutable_float!(MutableFloat, f32, u32, canonical_f32_bits, java_f32_cmp);
mutable_float!(MutableDouble, f64, u64, canonical_f64_bits, java_f64_cmp);

impl MutableFloat {
    /// Returns the exact Java `Float.hashCode` value.
    pub fn java_hash_code(&self) -> i32 {
        i32::from_ne_bytes(self.canonical_bits().to_ne_bytes())
    }
}

impl MutableDouble {
    /// Returns the exact Java `Double.hashCode` value.
    pub fn java_hash_code(&self) -> i32 {
        java_i64_hash(i64::from_ne_bytes(self.canonical_bits().to_ne_bytes()))
    }
}

/// Mutable boolean value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutableBool {
    value: bool,
}

impl MutableBool {
    /// Creates a mutable boolean value.
    pub const fn new(value: bool) -> Self {
        Self { value }
    }

    /// Parses using Java `Boolean.parseBoolean` semantics.
    pub fn parse(value: &str) -> Self {
        Self::new(value.eq_ignore_ascii_case("true"))
    }

    /// Returns the current primitive value.
    pub const fn get(&self) -> bool {
        self.value
    }

    /// Replaces the current value.
    pub fn set(&mut self, value: bool) {
        self.value = value;
    }

    /// Returns Java's boolean wrapper hash code.
    pub const fn java_hash_code(&self) -> i32 {
        if self.value { 1231 } else { 1237 }
    }

    /// Consumes the wrapper and returns its value.
    pub const fn into_inner(self) -> bool {
        self.value
    }
}

impl Mutable<bool> for MutableBool {
    fn get(&self) -> &bool {
        &self.value
    }

    fn get_mut(&mut self) -> &mut bool {
        &mut self.value
    }

    fn set(&mut self, value: bool) {
        self.value = value;
    }
}

impl From<bool> for MutableBool {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl FromStr for MutableBool {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(value))
    }
}

impl fmt::Display for MutableBool {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

/// Mutable key/value pair.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutablePair<K, V> {
    pair: (K, V),
}

impl<K, V> MutablePair<K, V> {
    /// Creates a mutable pair.
    pub const fn new(key: K, value: V) -> Self {
        Self { pair: (key, value) }
    }

    /// Borrows the key.
    pub const fn key(&self) -> &K {
        &self.pair.0
    }

    /// Borrows the value.
    pub const fn value(&self) -> &V {
        &self.pair.1
    }

    /// Replaces the key and returns this pair.
    pub fn set_key(&mut self, key: K) -> &mut Self {
        self.pair.0 = key;
        self
    }

    /// Replaces the value and returns this pair.
    pub fn set_value(&mut self, value: V) -> &mut Self {
        self.pair.1 = value;
        self
    }

    /// Borrows the complete pair.
    pub const fn get(&self) -> &(K, V) {
        &self.pair
    }

    /// Mutably borrows the complete pair.
    pub fn get_mut(&mut self) -> &mut (K, V) {
        &mut self.pair
    }

    /// Replaces both pair elements.
    pub fn set(&mut self, pair: (K, V)) {
        self.pair = pair;
    }

    /// Consumes the wrapper and returns the pair.
    pub fn into_inner(self) -> (K, V) {
        self.pair
    }
}

impl<K, V> Mutable<(K, V)> for MutablePair<K, V> {
    fn get(&self) -> &(K, V) {
        self.get()
    }

    fn get_mut(&mut self) -> &mut (K, V) {
        self.get_mut()
    }

    fn set(&mut self, pair: (K, V)) {
        self.set(pair);
    }
}

impl<K, V> From<(K, V)> for MutablePair<K, V> {
    fn from((key, value): (K, V)) -> Self {
        Self::new(key, value)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    fn trait_round_trip<T: Clone + PartialEq + fmt::Debug>(value: &mut impl Mutable<T>, next: &T) {
        value.set(next.clone());
        assert_eq!(value.get(), next);
        assert_eq!(&*value.get_mut(), next);
    }

    macro_rules! assert_integer_mutable {
        ($type:ident, $primitive:ty, $min:expr, $max:expr, $hash:expr) => {{
            let mut value = $type::default();
            assert_eq!(value.get(), 0);
            value.set(2);
            assert_eq!(value.increment().decrement().add(3).subtract(1).get(), 4);
            assert_eq!(value.byte_value(), 4_i8);
            assert_eq!(value.short_value(), 4_i16);
            assert_eq!(value.int_value(), 4_i32);
            assert_eq!(value.long_value(), 4_i64);
            assert_eq!(value.float_value(), 4_f32);
            assert_eq!(value.double_value(), 4_f64);
            assert_eq!(value.java_hash_code(), $hash);
            assert_eq!(value.to_string(), "4");
            assert_eq!($type::from(4 as $primitive), value);
            assert_eq!("4".parse::<$type>().unwrap(), value);
            assert!("bad".parse::<$type>().is_err());
            assert_eq!(value.into_inner(), 4);

            let mut wrapping = $type::new($max);
            assert_eq!(wrapping.increment().get(), $min);
            assert_eq!(wrapping.decrement().get(), $max);
            assert_eq!(wrapping.add(1).get(), $min);
            assert_eq!(wrapping.subtract(1).get(), $max);
            trait_round_trip(&mut wrapping, &(7 as $primitive));
            assert_eq!(*Mutable::get(&wrapping), 7);
        }};
    }

    #[test]
    fn integer_mutables_cover_wrapping_parsing_conversion_and_hashing() {
        assert_integer_mutable!(MutableByte, i8, i8::MIN, i8::MAX, 4);
        assert_integer_mutable!(MutableShort, i16, i16::MIN, i16::MAX, 4);
        assert_integer_mutable!(MutableInt, i32, i32::MIN, i32::MAX, 4);
        assert_integer_mutable!(MutableLong, i64, i64::MIN, i64::MAX, 4);
        assert_eq!(MutableLong::new(i64::MIN).java_hash_code(), i32::MIN);
    }

    macro_rules! assert_float_mutable {
        ($type:ident, $primitive:ty) => {{
            let mut value = $type::default();
            value.set(2.0);
            assert_eq!(
                value.increment().decrement().add(3.0).subtract(1.0).get(),
                4.0
            );
            assert_eq!(value.int_value(), 4);
            assert_eq!(value.long_value(), 4);
            assert_eq!(value.float_value(), 4.0);
            assert_eq!(value.double_value(), 4.0);
            assert_eq!(value.to_string(), "4");
            assert_eq!($type::from(4.0), value);
            assert_eq!("4".parse::<$type>().unwrap(), value);
            assert!("bad".parse::<$type>().is_err());
            assert_eq!(value.into_inner(), 4.0);
            trait_round_trip(&mut value, &(7.0 as $primitive));
            assert_eq!(*Mutable::get(&value), 7.0);

            let nan = $type::new(<$primitive>::NAN);
            let other_nan = $type::new(-<$primitive>::NAN);
            assert_eq!(nan, other_nan);
            assert_eq!(nan.cmp(&other_nan), Ordering::Equal);
            assert_eq!(nan.partial_cmp(&other_nan), Some(Ordering::Equal));
            assert_ne!($type::new(-0.0), $type::new(0.0));
            assert_eq!($type::new(-0.0).cmp(&$type::new(0.0)), Ordering::Less);
            assert_eq!($type::new(1.0).cmp(&$type::new(2.0)), Ordering::Less);
            assert_eq!(nan.cmp(&$type::new(2.0)), Ordering::Greater);

            let mut lhs = DefaultHasher::new();
            let mut rhs = DefaultHasher::new();
            nan.hash(&mut lhs);
            other_nan.hash(&mut rhs);
            assert_eq!(lhs.finish(), rhs.finish());
            assert_ne!(nan.java_hash_code(), 0);
        }};
    }

    #[test]
    fn floating_mutables_use_java_nan_signed_zero_comparison_and_hashing() {
        assert_float_mutable!(MutableFloat, f32);
        assert_float_mutable!(MutableDouble, f64);
    }

    #[test]
    fn boolean_and_object_mutables_are_owned_standard_value_wrappers() {
        let mut boolean = MutableBool::default();
        assert!(!boolean.get());
        boolean.set(true);
        assert_eq!(boolean.java_hash_code(), 1231);
        assert_eq!(MutableBool::new(false).java_hash_code(), 1237);
        assert!(MutableBool::parse("TrUe").get());
        assert!(!MutableBool::parse(" true ").get());
        assert!("true".parse::<MutableBool>().unwrap().into_inner());
        assert_eq!(MutableBool::from(false).to_string(), "false");
        trait_round_trip(&mut boolean, &false);

        let mut object = MutableObj::of(String::from("first"));
        assert_eq!(object.get(), "first");
        object.get_mut().push('!');
        object.set(String::from("second"));
        assert_eq!(object.to_string(), "second");
        assert_eq!(MutableObj::from(String::from("second")), object);
        trait_round_trip(&mut object, &String::from("third"));
        assert_eq!(object.into_inner(), "third");
        assert_eq!(MutableObj::<Option<i32>>::default().get(), &None);
    }

    #[test]
    fn mutable_pair_supports_individual_and_atomic_replacement() {
        let mut pair = MutablePair::new("first", 1);
        assert_eq!(pair.key(), &"first");
        assert_eq!(pair.value(), &1);
        pair.set_key("second").set_value(2);
        assert_eq!(pair.get(), &("second", 2));
        pair.get_mut().1 = 3;
        pair.set(("third", 4));
        assert_eq!(pair.get(), &("third", 4));
        trait_round_trip(&mut pair, &("fourth", 5));
        assert_eq!(MutablePair::from(("fourth", 5)), pair);
        assert_eq!(pair.into_inner(), ("fourth", 5));
        assert_eq!(
            MutablePair::<String, i32>::default().get(),
            &(String::new(), 0)
        );
    }
}
