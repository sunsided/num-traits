use core::ops::{Add, Mul};
use core::num::Wrapping;

/// Defines an additive identity element for `Self`.
pub trait Zero: Sized + Add<Self, Output = Self> {
    /// Returns the additive identity element of `Self`, `0`.
    ///
    /// # Laws
    ///
    /// ```{.text}
    /// a + 0 = a       ∀ a ∈ Self
    /// 0 + a = a       ∀ a ∈ Self
    /// ```
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // FIXME (#5527): This should be an associated constant
    fn zero() -> Self;

    /// Returns `true` if `self` is equal to the additive identity.
    #[inline]
    fn is_zero(&self) -> bool;
}

macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t { $v }
            #[inline]
            fn is_zero(&self) -> bool { *self == $v }
        }
    }
}

zero_impl!(usize, 0);
zero_impl!(u8,    0);
zero_impl!(u16,   0);
zero_impl!(u32,   0);
zero_impl!(u64,   0);
#[cfg(feature = "i128")]
zero_impl!(u128,  0);

zero_impl!(isize, 0);
zero_impl!(i8,    0);
zero_impl!(i16,   0);
zero_impl!(i32,   0);
zero_impl!(i64,   0);
#[cfg(feature = "i128")]
zero_impl!(i128,  0);

zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);

impl<T: Zero> Zero for Wrapping<T> where Wrapping<T>: Add<Output=Wrapping<T>> {
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn zero() -> Self {
        Wrapping(T::zero())
    }
}


/// Defines a multiplicative identity element for `Self`.
pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the multiplicative identity element of `Self`, `1`.
    ///
    /// # Laws
    ///
    /// ```{.text}
    /// a * 1 = a       ∀ a ∈ Self
    /// 1 * a = a       ∀ a ∈ Self
    /// ```
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // FIXME (#5527): This should be an associated constant
    fn one() -> Self;

    /// Returns `true` if `self` is equal to the multiplicative identity.
    ///
    /// For performance reasons, it's best to implement this manually.
    /// After a semver bump, this method will be required, and the
    /// `where Self: PartialEq` bound will be removed.
    #[inline]
    fn is_one(&self) -> bool where Self: PartialEq {
        *self == Self::one()
    }
}

macro_rules! one_impl {
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t { $v }
        }
    }
}

one_impl!(usize, 1);
one_impl!(u8,    1);
one_impl!(u16,   1);
one_impl!(u32,   1);
one_impl!(u64,   1);
#[cfg(feature = "i128")]
one_impl!(u128,   1);

one_impl!(isize, 1);
one_impl!(i8,    1);
one_impl!(i16,   1);
one_impl!(i32,   1);
one_impl!(i64,   1);
#[cfg(feature = "i128")]
one_impl!(i128,   1);

one_impl!(f32, 1.0);
one_impl!(f64, 1.0);

impl<T: One> One for Wrapping<T> where Wrapping<T>: Mul<Output=Wrapping<T>> {
    fn one() -> Self {
        Wrapping(T::one())
    }
}

// Some helper functions provided for backwards compatibility.

/// Returns the additive identity, `0`.
#[inline(always)] pub fn zero<T: Zero>() -> T { Zero::zero() }

/// Returns the multiplicative identity, `1`.
#[inline(always)] pub fn one<T: One>() -> T { One::one() }


#[test]
fn wrapping_identities() {
    macro_rules! test_wrapping_identities {
        ($($t:ty)+) => {
            $(
                assert_eq!(zero::<$t>(), zero::<Wrapping<$t>>().0);
                assert_eq!(one::<$t>(), one::<Wrapping<$t>>().0);
                assert_eq!((0 as $t).is_zero(), Wrapping(0 as $t).is_zero());
                assert_eq!((1 as $t).is_zero(), Wrapping(1 as $t).is_zero());
            )+
        };
    }

    test_wrapping_identities!(isize i8 i16 i32 i64 usize u8 u16 u32 u64);
}

#[test]
fn wrapping_is_zero() {
    fn require_zero<T: Zero>(_: &T) {}
    require_zero(&Wrapping(42));
}
#[test]
fn wrapping_is_one() {
    fn require_one<T: One>(_: &T) {}
    require_one(&Wrapping(42));
}
