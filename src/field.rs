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

// ───────────────────────── U03 세션 1: toy 제곱근 ─────────────────────────

impl<const P: u64> Fp<P> {
    /// Euler 판정. a^((P-1)/2) 는 a = 0 이면 0, a 가 제곱수면 1, 아니면 P-1 (즉 -1).
    /// 0 은 제곱수로 친다(0 = 0²).
    pub fn is_square(self) -> bool {
        // 0 은 제곱수이므로 true
        if self.0 == 0 {
            return true;
        }

        // Euler 판정: 0이 아닌 a에 대해 a^((P-1)/2)는 1 아니면 -1이고, 1일 때만 제곱수다.
        //
        // (→) 제곱수이면 1이다.
        //     a = s² 이면 a^((P-1)/2) = (s²)^((P-1)/2) = s^(P-1) ≡ 1   (Fermat의 소정리)
        //
        // (←) 1이면 제곱수다. 세 사실을 합친다.
        //     1. 차수 n인 방정식은 체에서 해가 많아야 n개다. (x-1)(x+1) = 0 이면 둘 중 하나가 0이어야 하듯이.
        //        그래서 x^((P-1)/2) = 1 의 해는 많아야 (P-1)/2개.
        //     2. 0이 아닌 제곱수는 정확히 (P-1)/2개다. s와 -s가 같은 제곱수를 내므로 P-1개 원소가
        //        {s, -s} 쌍 (P-1)/2개로 묶이고, 다른 쌍은 다른 제곱수를 낸다 (s² = t² 이면 t = ±s).
        //     3. 제곱수는 전부 그 방정식의 해다 ((→)에서 보임).
        //     의자가 최대 (P-1)/2개인데 제곱수 (P-1)/2개가 전부 앉아 있으니 비제곱수가 앉을 자리는 없다.
        //     비제곱수의 값은 제곱하면 1이 되는 나머지 하나, 즉 -1 (= P-1).
        //
        // 예: F13. 제곱수 {1,4,9,3,12,10} 여섯 개가 x⁶ = 1의 해 여섯 자리를 다 채운다. 비제곱수 2는 2⁶ = 64 ≡ 12 = -1.

        let exp = (P - 1) / 2;
        let res = self.pow(exp);
        res.0 == 1
    }
}

/// n = 2^s · t (t 홀수) 로 쪼갠다. 반환은 (s, t). `split_two_adic(12) == (2, 3)`.
/// 2로 나누어떨어지는 동안 나누며 s를 센다. n = 1 이면 (0, 1).
pub fn split_two_adic(n: u64) -> (u32, u64) {
    let mut s = 0;
    let mut t = n;
    
    while t % 2 == 0 {
        s += 1;
        t /= 2;
    }

    (s, t)
}

impl<const P: u64> Fp<P> {
    /// 비제곱수 하나. 2, 3, 4, … 순서로 `is_square` 가 false 인 첫 원소.
    /// 0이 아닌 원소의 절반이 비제곱수라 몇 걸음 안에 나온다. F13 에서는 2.
    pub fn non_residue() -> Self {
        // 0과 1은 제곱수이므로 2부터 시작한다.
        let mut z = Fp::new(2);
        while z.is_square() {
            z = z + Fp::one();
        }
        z
    }

    /// self 를 몇 번 제곱해야 1이 되는지. 그 최소 횟수 i 를 돌려준다 (self^(2^i) == 1).
    /// self == 1 이면 0. 전제: self 는 유한 번 제곱하면 1이 되는 원소다 (Tonelli-Shanks 루프 안의 u 가 그렇다).
    /// 주의: 제곱해 가는 임시 변수를 따로 두어 self 자체는 바꾸지 않는다.
    pub fn order_exponent(self) -> u32 {
        if self == Fp::one() {
            return 0;
        }

        let mut i = 1;
        let mut power = self * self; // self^(2^1)
        while power != Fp::one() {
            i += 1;
            power = power * power; // self^(2^(i+1)) = (self^(2^i))^2
        }

        i
    }

    /// 제곱근 (Tonelli-Shanks). 부품 1·2·3 을 조립한다.
    ///
    /// 원리: 일단 추측하고, 오차를 2진수 자릿수 단위로 지워 나간다.
    /// 루프가 들고 다니는 상태는 넷이다.
    ///
    ///   r  추측.   답이 되고 싶은 값. 시작값 a^((t+1)/2) 를 제곱하면 a · a^t 라 오차 a^t 가 붙어 있다.
    ///   u  오차.   언제나  r² == a · u  가 성립하도록 유지한다. u 가 1 이 되는 순간 r 이 답이다.
    ///              a 가 제곱수면 u^(2^(s-1)) = a^((P-1)/2) = 1 이라, u 는 "2^s 번 안에 1 이 되는 원소"들의
    ///              작은 순환군(크기 2^s) 안에 갇혀 있다. 루프는 그 군 안에서만 u 를 움직인다.
    ///   c  도구.   그 순환군의 생성원. 비제곱수 z 에 대해 c = z^t 는 차수가 정확히 2^s 라서
    ///              군의 모든 원소가 c 의 거듭제곱이다. 매 바퀴 c 의 거듭제곱 b 를 골라 r 에 곱한다.
    ///              r 에 b 를 곱하면 r² == a·u 를 지키기 위해 u 에는 b² 를 곱해야 한다.
    ///   m  방 크기. u 가 사는 군의 크기를 지수로 적은 것. 시작은 s. 매 바퀴 줄어들어 루프는 s 번 안에 끝난다.
    ///
    /// 한 바퀴: u 를 i 번 제곱하면 1 이 된다고 하자 (u 의 차수 2^i, i < m).
    ///   b = c^(2^(m-i-1)) 는 차수가 2^(i+1), b² 는 2^i 라서 u·b² 의 차수는 2^i 미만으로 떨어진다.
    ///   그래서 m = i 로 줄이고, 다음 바퀴의 도구도 c = b² (차수 2^i) 로 줄여 새 m 에 맞춘다.
    ///
    ///   0.  a = 0 이면 Some(0). 제곱수가 아니면 None.
    ///   1.  (s, t) = split_two_adic(P - 1)
    ///   2.  z = non_residue()
    ///   3.  c = z^t,  r = a^((t+1)/2),  u = a^t,  m = s        (불변식: r² == a · u)
    ///   4.  u != 1 인 동안:
    ///         i = u.order_exponent()
    ///         b = c 를 (m - i - 1) 번 제곱한 것                 (= c^(2^(m-i-1)))
    ///         r = r·b,  c = b²,  u = u·c,  m = i                (c 를 먼저 b² 로 바꾸면 u·c 가 곧 u·b²)
    ///   5.  Some(r)
    ///
    /// 예 (F13, a = 10): s = 2, t = 3, z = 2, c = 8.  시작 r = 9, u = 12, m = 2.
    ///   i = 1 (12² = 1), b = c^(2^0) = 8.  r = 9·8 = 7, c = 12, u = 12·12 = 1, m = 1.  → Some(7), 7² = 49 ≡ 10 ✓
    pub fn sqrt(self) -> Option<Self> {
        if self.0 == 0 {
            return Some(Fp::zero());
        }

        if !self.is_square() {
            return None;
        }

        let (s, t) = split_two_adic(P - 1);
        let z = Fp::non_residue();
        let mut r = self.pow((t + 1) / 2); // 제곱근 추측
        let mut u = self.pow(t); // 추측에 대한 오차
        let mut c = z.pow(t); // 오차를 줄이는 계수
        let mut m = s; // 오차를 줄이는 계수의 차수

        while u != Fp::one() {
            let i = u.order_exponent();
            let exp = 1 << (m - i - 1);
            let b = c.pow(exp);
            r = r * b;
            c = b * b;
            u = u * c;
            m = i;
        }

        Some(r)
    }
}

/// 부품 테스트: `cargo test sqrt_parts`
#[cfg(test)]
mod sqrt_parts {
    use super::*;

    #[test]
    fn split_two_adic_examples() {
        assert_eq!(split_two_adic(12), (2, 3)); // 13 - 1
        assert_eq!(split_two_adic(16), (4, 1)); // 17 - 1
        assert_eq!(split_two_adic(222), (1, 111)); // 223 - 1
        assert_eq!(split_two_adic(1), (0, 1)); // 2 - 1
        assert_eq!(split_two_adic(40), (3, 5));
    }

    #[test]
    fn non_residue_is_not_a_square() {
        assert_eq!(Fp::<13>::non_residue(), Fp::<13>::new(2));
        assert_eq!(Fp::<17>::non_residue(), Fp::<17>::new(3)); // 2 = 6² 는 제곱수라 건너뛴다
        let z = Fp::<223>::non_residue();
        assert!(z != Fp::<223>::zero() && !z.is_square());
    }

    #[test]
    fn order_exponent_examples() {
        type F13 = Fp<13>;
        assert_eq!(F13::one().order_exponent(), 0);
        assert_eq!(F13::new(12).order_exponent(), 1); // (-1)² = 1
        assert_eq!(F13::new(5).order_exponent(), 2); // 5² = 12, 5⁴ = 1
        assert_eq!(F13::new(8).order_exponent(), 2); // 8² = 12, 8⁴ = 1
        let u = F13::new(5);
        assert_eq!(u.order_exponent(), 2);
        assert_eq!(u, F13::new(5)); // self 는 바뀌지 않는다
    }
}

/// 세션 1 게이트의 절반: `cargo test sqrt_toy`
#[cfg(test)]
mod sqrt_toy {
    use super::*;

    type F13 = Fp<13>; // 13 ≡ 1 (mod 4), s = 2 — 진짜 루프가 돈다
    type F17 = Fp<17>; // 17 - 1 = 2^4 · 1, s = 4
    type F223 = Fp<223>; // 223 ≡ 3 (mod 4), s = 1 — 루프 없는 경우

    #[test]
    fn euler_criterion_counts_squares() {
        // 0 을 빼면 제곱수와 비제곱수가 정확히 반반이다.
        let squares = (1..13).filter(|&v| F13::new(v).is_square()).count();
        assert_eq!(squares, 6);
        assert!(F13::zero().is_square());
        assert!(F13::new(4).is_square()); // 2² = 4
        assert!(!F13::new(2).is_square()); // {1,4,9,3,12,10} 에 2 는 없다
        assert!(F17::new(2).is_square()); // 6² = 36 = 2·17 + 2
    }

    #[test]
    fn sqrt_agrees_with_is_square() {
        fn check<const P: u64>() {
            for v in 0..P {
                let a = Fp::<P>::new(v);
                match a.sqrt() {
                    Some(s) => {
                        assert!(a.is_square(), "P={P} v={v}");
                        assert_eq!(s * s, a, "P={P} v={v}");
                    }
                    None => assert!(!a.is_square(), "P={P} v={v}"),
                }
            }
        }
        check::<13>();
        check::<17>();
        check::<223>();
    }

    #[test]
    fn every_square_has_a_root() {
        for v in 0..223 {
            let a = F223::new(v);
            let s = (a * a).sqrt().expect("a² 는 항상 제곱수");
            assert_eq!(s * s, a * a);
        }
    }

    #[test]
    fn known_roots() {
        let s = F13::new(4).sqrt().unwrap();
        assert!(s == F13::new(2) || s == F13::new(11));
        assert_eq!(F13::new(2).sqrt(), None);
        let s = F17::new(2).sqrt().unwrap();
        assert!(s == F17::new(6) || s == F17::new(11));
        assert_eq!(F13::zero().sqrt(), Some(F13::zero()));
    }
}
