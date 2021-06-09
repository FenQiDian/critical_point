use super::convert;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use fixed::types::I32F32;
use num_traits::{Bounded, FromPrimitive, Num, One, Signed, Zero};
use rand::distributions::{Distribution, OpenClosed01, Standard};
use rand::Rng;
use simba::scalar::{ComplexField, Field, RealField, SubsetOf};
use simba::simd::{PrimitiveSimdValue, SimdValue};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Eq)]
pub struct Fx(pub(crate) I32F32);

impl Default for Fx {
    fn default() -> Fx {
        return Fx(I32F32::from_bits(0));
    }
}

impl Debug for Fx {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return Debug::fmt(&self.0, f);
    }
}

impl Display for Fx {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return Display::fmt(&self.0, f);
    }
}

impl Hash for Fx {
    fn hash<H: Hasher>(&self, h: &mut H) {
        return self.0.hash(h);
    }
}

impl PartialEq for Fx {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl PartialOrd for Fx {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.0.partial_cmp(&other.0);
    }
}

impl Ord for Fx {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        return self.0.cmp(&other.0);
    }
}

impl Distribution<Fx> for Standard {
    #[inline]
    fn sample<'a, G: Rng + ?Sized>(&self, rng: &mut G) -> Fx {
        let bits = rng.gen();
        return Fx(I32F32::from_bits(bits));
    }
}

impl Distribution<Fx> for OpenClosed01 {
    #[inline]
    fn sample<'a, G: Rng + ?Sized>(&self, rng: &mut G) -> Fx {
        let val: f64 = rng.gen();
        return Fx(I32F32::from_num(val));
    }
}

impl PrimitiveSimdValue for Fx {}

impl SimdValue for Fx {
    type Element = Self;
    type SimdBool = bool;

    #[inline(always)]
    fn lanes() -> usize {
        return 1;
    }

    #[inline(always)]
    fn splat(val: Self::Element) -> Self {
        return val;
    }

    #[inline(always)]
    fn extract(&self, _: usize) -> Self::Element {
        return *self;
    }

    #[inline(always)]
    unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
        return *self;
    }

    #[inline(always)]
    fn replace(&mut self, _: usize, val: Self::Element) {
        *self = val;
    }

    #[inline(always)]
    unsafe fn replace_unchecked(&mut self, _: usize, val: Self::Element) {
        *self = val;
    }

    #[inline(always)]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        return if cond { self } else { other };
    }
}

impl Mul for Fx {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        return Self(self.0.saturating_mul(rhs.0));
    }
}

impl Div for Fx {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        if !rhs.is_zero() {
            return Self(self.0.saturating_div(rhs.0));
        } else {
            if self.0 > 0 {
                return Self::max_value();
            } else if self.0 < 0 {
                return Self::min_value();
            } else {
                return Self::zero();
            }
        }
    }
}

impl Rem for Fx {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        if !rhs.is_zero() {
            return Self(self.0 % rhs.0);
        } else {
            return Self::zero();
        }
    }
}

impl Add for Fx {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        return Self(self.0.saturating_add(rhs.0));
    }
}

impl Sub for Fx {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        return Self(self.0.saturating_sub(rhs.0));
    }
}

impl Neg for Fx {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        return Self(-self.0);
    }
}

impl MulAssign for Fx {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl DivAssign for Fx {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl RemAssign for Fx {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl AddAssign for Fx {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Fx {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Zero for Fx {
    #[inline(always)]
    fn zero() -> Self {
        return Self(I32F32::from_num(0));
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        return self.0 == Self::zero().0;
    }
}

impl One for Fx {
    #[inline(always)]
    fn one() -> Self {
        return Self(I32F32::from_num(1));
    }
}

impl Num for Fx {
    type FromStrRadixErr = ();
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

impl Field for Fx {}

impl SubsetOf<Fx> for f64 {
    #[inline]
    fn to_superset(&self) -> Fx {
        return Fx(I32F32::from_num(*self));
    }

    #[inline]
    fn from_superset(element: &Fx) -> Option<Self> {
        return Some(Self::from_superset_unchecked(element));
    }

    #[inline]
    fn from_superset_unchecked(element: &Fx) -> Self {
        return element.0.to_num::<f64>();
    }

    #[inline]
    fn is_in_subset(_: &Fx) -> bool {
        return true;
    }
}

impl SubsetOf<Fx> for Fx {
    #[inline]
    fn to_superset(&self) -> Fx {
        return *self;
    }

    #[inline]
    fn from_superset(element: &Fx) -> Option<Self> {
        return Some(*element);
    }

    #[inline]
    fn from_superset_unchecked(element: &Fx) -> Self {
        return *element;
    }

    #[inline]
    fn is_in_subset(_: &Fx) -> bool {
        return true;
    }
}

impl AbsDiffEq for Fx {
    type Epsilon = Self;
    fn default_epsilon() -> Self::Epsilon {
        return Self(I32F32::from_bits(0x16));
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        // This is the impl used in the approx crate.
        if self > other {
            return (*self - *other) <= epsilon;
        } else {
            return (*other - *self) <= epsilon;
        }
    }
}

impl RelativeEq for Fx {
    fn default_max_relative() -> Self::Epsilon {
        return Self::default_epsilon();
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        // This is the impl used in the approx crate.
        let abs_diff = (*self - *other).abs();

        if abs_diff <= epsilon {
            return true;
        }

        let abs_self = self.abs();
        let abs_other = other.abs();

        let largest = if abs_other > abs_self {
            abs_other
        } else {
            abs_self
        };

        return abs_diff <= largest * max_relative;
    }
}

impl UlpsEq for Fx {
    fn default_max_ulps() -> u32 {
        return 4;
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        if self.abs_diff_eq(other, epsilon) {
            return true;
        }

        if self.signum() != other.signum() {
            return false;
        }

        let bits1 = self.0.to_bits();
        let bits2 = other.0.to_bits();

        if bits1 > bits2 {
            return (bits1 - bits2) <= max_ulps as i64;
        } else {
            return (bits2 - bits1) <= max_ulps as i64;
        }
    }
}

impl Bounded for Fx {
    #[inline]
    fn min_value() -> Self {
        return Self(I32F32::MIN);
    }

    #[inline]
    fn max_value() -> Self {
        return Self(I32F32::MAX);
    }
}

impl FromPrimitive for Fx {
    fn from_i64(n: i64) -> Option<Self> {
        return Some(convert::fx_i64(n));
    }
    fn from_u64(n: u64) -> Option<Self> {
        return Some(convert::fx_u64(n));
    }
    fn from_isize(n: isize) -> Option<Self> {
        return Some(convert::fx_isize(n));
    }
    fn from_i8(n: i8) -> Option<Self> {
        return Some(convert::fx_i8(n));
    }
    fn from_i16(n: i16) -> Option<Self> {
        return Some(convert::fx_i16(n));
    }
    fn from_i32(n: i32) -> Option<Self> {
        return Some(convert::fx_i32(n));
    }
    fn from_usize(n: usize) -> Option<Self> {
        return Some(convert::fx_usize(n));
    }
    fn from_u8(n: u8) -> Option<Self> {
        return Some(convert::fx_u8(n));
    }
    fn from_u16(n: u16) -> Option<Self> {
        return Some(convert::fx_u16(n));
    }
    fn from_u32(n: u32) -> Option<Self> {
        return Some(convert::fx_u32(n));
    }
    fn from_f32(n: f32) -> Option<Self> {
        return Some(convert::fx_f32(n));
    }
    fn from_f64(n: f64) -> Option<Self> {
        return Some(convert::fx_f64(n));
    }
}

impl Signed for Fx {
    fn abs(&self) -> Self {
        return Self(self.0.abs());
    }

    fn abs_sub(&self, other: &Self) -> Self {
        return self.abs() - *other;
    }

    fn signum(&self) -> Self {
        return Self(self.0.signum());
    }

    fn is_positive(&self) -> bool {
        return self.0 >= Self::zero().0;
    }

    fn is_negative(&self) -> bool {
        return self.0 <= Self::zero().0;
    }
}

impl ComplexField for Fx {
    type RealField = Self;

    #[inline]
    fn from_real(re: Self::RealField) -> Self {
        return re;
    }

    #[inline]
    fn real(self) -> Self::RealField {
        return self;
    }

    #[inline]
    fn imaginary(self) -> Self::RealField {
        return Self::zero();
    }

    #[inline]
    fn norm1(self) -> Self::RealField {
        return self.abs();
    }

    #[inline]
    fn modulus(self) -> Self::RealField {
        return self.abs();
    }

    #[inline]
    fn modulus_squared(self) -> Self::RealField {
        return self * self;
    }

    #[inline]
    fn argument(self) -> Self::RealField {
        if self >= Self::zero() {
            return Self::zero();
        } else {
            return Self::pi();
        }
    }

    #[inline]
    fn to_exp(self) -> (Self, Self) {
        if self >= Self::zero() {
            return (self, Self::one());
        } else {
            return (-self, -Self::one());
        }
    }

    #[inline]
    fn recip(self) -> Self {
        return Self::one() / self;
    }

    #[inline]
    fn conjugate(self) -> Self {
        return self;
    }

    #[inline]
    fn scale(self, factor: Self::RealField) -> Self {
        return self * factor;
    }

    #[inline]
    fn unscale(self, factor: Self::RealField) -> Self {
        return self / factor;
    }

    #[inline]
    fn floor(self) -> Self {
        return Self(self.0.floor());
    }

    #[inline]
    fn ceil(self) -> Self {
        return Self(self.0.ceil());
    }

    #[inline]
    fn round(self) -> Self {
        return Self(self.0.round());
    }

    #[inline]
    fn trunc(self) -> Self {
        return Self(self.0.int());
    }

    #[inline]
    fn fract(self) -> Self {
        return Self(self.0.frac());
    }

    #[inline]
    fn abs(self) -> Self {
        return Self(self.0.abs());
    }

    #[inline]
    fn signum(self) -> Self {
        return Self(self.0.signum());
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        return self * a + b;
    }

    #[inline]
    fn powi(self, _n: i32) -> Self {
        unimplemented!()
    }

    #[inline]
    fn powf(self, _n: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn powc(self, _n: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn sqrt(self) -> Self {
        return Self(cordic::sqrt(self.0));
    }

    #[inline]
    fn try_sqrt(self) -> Option<Self> {
        if self >= Self::zero() {
            return Some(self.sqrt());
        } else {
            return None;
        }
    }

    #[inline]
    fn exp(self) -> Self {
        return Self(cordic::exp(self.0));
    }

    #[inline]
    fn exp2(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn exp_m1(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn ln_1p(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn ln(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn log(self, _base: Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn log2(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn log10(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn cbrt(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn hypot(self, _other: Self) -> Self::RealField {
        unimplemented!()
    }

    #[inline]
    fn sin(self) -> Self {
        Self(cordic::sin(self.0))
    }

    #[inline]
    fn cos(self) -> Self {
        return Self(cordic::cos(self.0));
    }

    #[inline]
    fn tan(self) -> Self {
        return Self(cordic::tan(self.0));
    }

    #[inline]
    fn asin(self) -> Self {
        return Self(cordic::asin(self.0));
    }

    #[inline]
    fn acos(self) -> Self {
        return Self(cordic::acos(self.0));
    }

    #[inline]
    fn atan(self) -> Self {
        return Self(cordic::atan(self.0));
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = cordic::sin_cos(self.0);
        return (Self(sin), Self(cos));
    }

    #[inline]
    fn sinh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn cosh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn tanh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn asinh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn acosh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn atanh(self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn is_finite(&self) -> bool {
        return true;
    }
}

impl RealField for Fx {
    #[inline]
    fn is_sign_positive(self) -> bool {
        return self.0.is_positive();
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        return self.0.is_negative();
    }

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        if sign >= Self::zero() {
            return self.abs();
        } else {
            return -self.abs();
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        if self >= other {
            return self;
        } else {
            return other;
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        if self < other {
            return self;
        } else {
            return other;
        }
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            return min;
        } else if self > max {
            return max;
        } else {
            return self;
        }
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        return Self(cordic::atan2(self.0, other.0));
    }

    /// Archimedes' constant.
    #[inline]
    fn pi() -> Self {
        return Self(I32F32::PI);
    }

    /// 2.0 * pi.
    #[inline]
    fn two_pi() -> Self {
        return Self::pi() + Self::pi();
    }

    /// pi / 2.0.
    #[inline]
    fn frac_pi_2() -> Self {
        return Self(I32F32::FRAC_PI_2);
    }

    /// pi / 3.0.
    #[inline]
    fn frac_pi_3() -> Self {
        return Self(I32F32::FRAC_PI_3);
    }

    /// pi / 4.0.
    #[inline]
    fn frac_pi_4() -> Self {
        return Self(I32F32::FRAC_PI_4);
    }

    /// pi / 6.0.
    #[inline]
    fn frac_pi_6() -> Self {
        return Self(I32F32::FRAC_PI_6);
    }

    /// pi / 8.0.
    #[inline]
    fn frac_pi_8() -> Self {
        return Self(I32F32::FRAC_PI_8);
    }

    /// 1.0 / pi.
    #[inline]
    fn frac_1_pi() -> Self {
        return Self(I32F32::FRAC_1_PI);
    }

    /// 2.0 / pi.
    #[inline]
    fn frac_2_pi() -> Self {
        return Self(I32F32::FRAC_2_PI);
    }

    /// 2.0 / sqrt(pi).
    #[inline]
    fn frac_2_sqrt_pi() -> Self {
        return Self(I32F32::FRAC_2_SQRT_PI);
    }

    /// Euler's number.
    #[inline]
    fn e() -> Self {
        return Self(I32F32::E);
    }

    /// log2(e).
    #[inline]
    fn log2_e() -> Self {
        return Self(I32F32::LOG2_E);
    }

    /// log10(e).
    #[inline]
    fn log10_e() -> Self {
        return Self(I32F32::LOG10_E);
    }

    /// ln(2.0).
    #[inline]
    fn ln_2() -> Self {
        return Self(I32F32::LN_2);
    }

    /// ln(10.0).
    #[inline]
    fn ln_10() -> Self {
        return Self(I32F32::LN_10);
    }
}

//
// critical-point extension
//

impl Fx {
    #[inline]
    pub fn to_i8(&self) -> i8 {
        return self.0.to_num::<i8>();
    }

    #[inline]
    pub fn to_u8(&self) -> i8 {
        return self.0.to_num::<i8>();
    }

    #[inline]
    pub fn to_i16(&self) -> i16 {
        return self.0.to_num::<i16>();
    }

    #[inline]
    pub fn to_u16(&self) -> i16 {
        return self.0.to_num::<i16>();
    }

    #[inline]
    pub fn to_i32(&self) -> i32 {
        return self.0.to_num::<i32>();
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        return self.0.to_num::<u32>();
    }

    #[inline]
    pub fn to_i64(&self) -> i64 {
        return self.0.to_num::<i64>();
    }

    #[inline]
    pub fn to_u64(&self) -> u64 {
        return self.0.to_num::<u64>();
    }

    #[inline]
    pub fn to_isize(&self) -> isize {
        return self.0.to_num::<isize>();
    }

    #[inline]
    pub fn to_usize(&self) -> usize {
        return self.0.to_num::<usize>();
    }

    #[inline]
    pub fn to_f32(&self) -> f32 {
        return self.0.to_num::<f32>();
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        return self.0.to_num::<f64>();
    }
}
