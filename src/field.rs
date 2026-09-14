//! U02 — toy 유한체 F_p.
//!
//! 정수를 소수 p로 나눈 나머지 {0, 1, …, p-1} 위에 덧셈·뺄셈·곱셈·나눗셈을 정의한다.
//! p가 소수여야 0이 아닌 모든 원소에 곱셈 역원이 있다(Fermat의 소정리: a^(p-1) ≡ 1).
//! bitcoin-from-scratch chapter01의 Go `FieldElement`와 같은 것을 const generic으로 다시 만든다.
//!
//! U03에서 이 toy를 `bls12_381::Scalar`(p = U00의 r, 255비트)로 교체한다. r은 u64에 안 들어가므로
//! 여기서는 P < 2^63인 소수만 다룬다. 덧셈 중간값은 u64에, 곱셈 중간값은 u128에 들어온다.

use core::ops::{Add, Div, Mul, Neg, Sub};

/// 소수 P를 법으로 하는 유한체의 원소.
/// 내부 값은 항상 `0..P`로 환원돼 있어야 한다. U01의 `Zatoshis`처럼 private 필드가 이 불변식을 지킨다.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp<const P: u64>(u64);

impl<const P: u64> Fp<P> {
    /// 아무 u64나 받아 `mod P`로 환원한다. `new(P) == zero()`, `new(P + 1) == one()`.
    pub fn new(value: u64) -> Self {
        if value >= P {
            Fp(value % P)
        } else {
            Fp(value)
        }
    }

    pub fn zero() -> Self {
        Fp(0)
    }

    pub fn one() -> Self {
        Fp(1)
    }

    pub fn into_u64(self) -> u64 {
        self.0
    }

    /// 거듭제곱. square-and-multiply로 O(log exp). `pow(0) == one()`.
    pub fn pow(self, exp: u64) -> Self {
        let mut result = Fp::one();
        let mut base = self;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base;
            }
            base = base * base;
            exp /= 2;
        }

        result
    }

    /// 곱셈 역원. Fermat의 소정리로 a^(P-2). 0은 역원이 없으므로 `None`.
    pub fn inv(self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(self.pow(P - 2))
        }
    }
}

impl<const P: u64> Add for Fp<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let sum = self.0 + rhs.0;
        if sum >= P {
            Fp(sum - P)
        } else {
            Fp(sum)
        }
    }
}

impl<const P: u64> Sub for Fp<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.0 >= rhs.0 {
            Fp(self.0 - rhs.0)
        } else {
            Fp(self.0 + P - rhs.0)
        }
    }
}

impl<const P: u64> Mul for Fp<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let product = (self.0 as u128) * (rhs.0 as u128);
        Fp((product % (P as u128)) as u64)
    }
}

impl<const P: u64> Neg for Fp<P> {
    type Output = Self;
    fn neg(self) -> Self {
        if self.0 == 0 {
            Fp::zero()
        } else {
            Fp(P - self.0)
        }
    }
}

/// 0으로 나누면 패닉한다(정수 나눗셈과 같은 계약). Option이 필요하면 `inv()`를 직접 쓴다.
impl<const P: u64> Div for Fp<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv().unwrap_or_else(|| panic!("Division by zero"))
    }
}

/// 세션 1 게이트: `cargo test axioms`
#[cfg(test)]
mod axioms {
    use super::*;

    type F13 = Fp<13>;
    /// bitcoin-from-scratch chapter02의 toy 곡선 y² = x³ + 7 이 살던 체.
    type F223 = Fp<223>;
    /// 2^61 - 1 (메르센 소수). 곱셈 중간값이 u64를 넘는다.
    type F61 = Fp<2_305_843_009_213_693_951>;

    fn all13() -> impl Iterator<Item = F13> {
        (0..13).map(F13::new)
    }

    #[test]
    fn new_reduces_mod_p() {
        assert_eq!(F13::new(0), F13::zero());
        assert_eq!(F13::new(13), F13::zero());
        assert_eq!(F13::new(14), F13::one());
        assert_eq!(F13::new(27).into_u64(), 1);
        assert_eq!(F223::new(1000).into_u64(), 1000 % 223);
    }

    #[test]
    fn add_mul_commutative_associative() {
        for a in all13() {
            for b in all13() {
                assert_eq!(a + b, b + a);
                assert_eq!(a * b, b * a);
                for c in all13() {
                    assert_eq!((a + b) + c, a + (b + c));
                    assert_eq!((a * b) * c, a * (b * c));
                }
            }
        }
    }

    #[test]
    fn distributive() {
        for a in all13() {
            for b in all13() {
                for c in all13() {
                    assert_eq!(a * (b + c), a * b + a * c);
                }
            }
        }
    }

    #[test]
    fn identities() {
        for a in all13() {
            assert_eq!(a + F13::zero(), a);
            assert_eq!(a * F13::one(), a);
            assert_eq!(a * F13::zero(), F13::zero());
        }
    }

    #[test]
    fn additive_inverse_and_wraparound() {
        for a in all13() {
            assert_eq!(a + (-a), F13::zero());
            assert_eq!(a - a, F13::zero());
            for b in all13() {
                assert_eq!(a - b, a + (-b));
            }
        }
        assert_eq!(F13::zero() - F13::one(), F13::new(12));
        assert_eq!(-F13::zero(), F13::zero());
    }

    /// Programming Bitcoin 1장의 예제 값.
    #[test]
    fn programming_bitcoin_chapter1_examples() {
        assert_eq!(Fp::<57>::new(44) + Fp::<57>::new(33), Fp::<57>::new(20));
        assert_eq!(Fp::<57>::new(9) - Fp::<57>::new(29), Fp::<57>::new(37));
        assert_eq!(
            Fp::<97>::new(95) * Fp::<97>::new(45) * Fp::<97>::new(31),
            Fp::<97>::new(23)
        );
    }

    /// 곱셈 중간값이 u64를 넘는 경우. 2^40 · 2^40 = 2^80 ≡ 2^19 (mod 2^61 - 1).
    #[test]
    fn mul_does_not_overflow_u64() {
        let p_minus_1 = F61::new(2_305_843_009_213_693_950);
        assert_eq!(p_minus_1 * p_minus_1, F61::one());
        assert_eq!(F61::new(1 << 40) * F61::new(1 << 40), F61::new(1 << 19));
    }
}

/// 세션 2 게이트: `cargo test` 전체
#[cfg(test)]
mod fermat {
    use super::*;

    type F13 = Fp<13>;
    type F223 = Fp<223>;

    #[test]
    fn pow_basics() {
        for v in 0..13 {
            let a = F13::new(v);
            assert_eq!(a.pow(0), F13::one());
            assert_eq!(a.pow(1), a);
            assert_eq!(a.pow(2), a * a);
            assert_eq!(a.pow(3), a * a * a);
        }
        assert_eq!(F13::new(3).pow(3), F13::one()); // 27 = 2·13 + 1
    }

    #[test]
    fn fermat_little_theorem() {
        for v in 1..13 {
            assert_eq!(F13::new(v).pow(12), F13::one());
        }
        for v in [1u64, 2, 3, 100, 222] {
            assert_eq!(F223::new(v).pow(222), F223::one());
        }
        assert_eq!(F13::zero().pow(12), F13::zero());
    }

    #[test]
    fn inverse() {
        assert_eq!(F13::zero().inv(), None);
        for v in 1..13 {
            let a = F13::new(v);
            assert_eq!(a * a.inv().unwrap(), F13::one());
        }
        assert_eq!(F13::new(2).inv(), Some(F13::new(7))); // 2·7 = 14 ≡ 1
        assert_eq!(Fp::<97>::new(3).inv(), Some(Fp::<97>::new(65))); // 3·65 = 195 = 2·97 + 1
    }

    #[test]
    fn division() {
        for a in (0..13).map(F13::new) {
            for b in (1..13).map(F13::new) {
                assert_eq!(a / b, a * b.inv().unwrap());
                assert_eq!((a / b) * b, a);
            }
        }
        assert_eq!(Fp::<31>::new(3) / Fp::<31>::new(24), Fp::<31>::new(4)); // Programming Bitcoin 1장
    }

    #[test]
    #[should_panic]
    fn division_by_zero_panics() {
        let _ = F13::one() / F13::zero();
    }
}
