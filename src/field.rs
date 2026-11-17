//! W2 — 시계 위의 수를 타입으로.
//!
//! `clock.rs`에서는 시계의 칸 수를 함수마다 인자 `p`로 넘겼다. 여기서는 칸 수를 타입에 적어 둔다.
//! `Fp<13>`은 13칸 시계 위의 수이고 `Fp<17>`은 17칸 시계 위의 수다. 둘은 서로 다른 타입이다.

// `+`, 앞에 붙는 `-`, 사이에 오는 `-`, `*`에 딸린 트레이트.
// 구현해 두면 `a + b`는 `add`를, `-a`는 `neg`를, `a - b`는 `sub`를, `a * b`는 `mul`을 부른다.
use std::ops::{Add, Mul, Neg, Sub};

/// P칸 시계 위의 수. 안에 든 값은 언제나 0 이상 P 미만이다.
///
/// `<const P: u64>`는 타입 이름 옆에 타입이 아니라 값을 적는 자리다(const generics).
/// `Fp<13>`이라고 쓰면 이 정의 안의 `P`가 전부 13이 된다.
/// 필드는 `Zatoshis`처럼 private이라 `new`를 거쳐야만 만들 수 있다.
/// P는 2 이상이어야 한다. 0이면 `% P`가 패닉하고, 1이면 `one`이 P 미만이 아니다.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp<const P: u64>(u64);

// `impl<const P: u64> Fp<P>`는 "P가 무엇이든 `Fp<P>`에 아래 함수들을 붙인다"는 뜻이다.
impl<const P: u64> Fp<P> {
    /// 아무 `u64`나 받아 P칸 시계 위의 수로 만든다. P 이상이면 감긴다.
    pub fn new(value: u64) -> Self {
        Self(value % P)
    }

    /// 0. 어느 시계에나 있다.
    pub fn zero() -> Self {
        Self(0)
    }

    /// 1.
    pub fn one() -> Self {
        Self(1)
    }

    /// 안에 든 수를 꺼낸다. 필드가 private이라 모듈 밖에서 값을 읽는 길은 이것뿐이다.
    pub fn into_u64(self) -> u64 {
        self.0
    }
}

/// 시계 위의 덧셈. 결과도 0 이상 P 미만이어야 한다.
impl<const P: u64> Add for Fp<P> {
    /// `+`의 결과 타입. `Zatoshis`와 달리 실패가 없어서 `Option`이 아니라 `Self`다.
    type Output = Self;

    /// 두 값이 모두 P 미만이라 합은 2P 미만이다. 그래서 P를 한 번만 빼면 다시 P 미만이 된다.
    fn add(self, rhs: Self) -> Self {
        let mut sum = self.0 + rhs.0;
        if sum >= P {
            sum -= P;
        }
        Self(sum)
    }
}

/// 부호 바꾸기. `-a`는 `a`에 더하면 0이 되는 수다.
impl<const P: u64> Neg for Fp<P> {
    type Output = Self;

    /// `P - a`는 a가 0일 때만 P가 되어 시계 밖으로 나간다. `% P`가 그 P를 0으로 돌려놓는다.
    fn neg(self) -> Self {
        Self((P - self.0) % P)
    }
}

/// 시계 위의 뺄셈. 결과도 0 이상 P 미만이어야 한다.
impl<const P: u64> Sub for Fp<P> {
    type Output = Self;

    /// `self.0 - rhs.0`이라고 그대로 빼면 0 아래로 내려갈 때 패닉한다.
    /// 그래서 빼는 대신 `-rhs`를 더한다. `-rhs`도 0 이상 P 미만이라 어디에서도 0 아래로 내려가지 않는다.
    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

/// 시계 위의 곱셈. 결과도 0 이상 P 미만이어야 한다.
impl<const P: u64> Mul for Fp<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // L11: 먼저 `Self(self.0 * rhs.0 % P)`라고 쓰고 `cargo test field`에서 어느 테스트가 깨지는지 본다.
        //      그다음 곱하기 전에 두 수를 `u128`로 넓힌다(`self.0 as u128`). P도 같은 타입이어야 나눌 수 있다.
        //      마지막에 결과를 `u64`로 되돌린다(`… as u64`).
        Self((self.0 as u128 * rhs.0 as u128 % P as u128) as u64)
    }
}

#[cfg(test)]
mod axioms {
    use super::*;

    // `type`은 타입에 짧은 별명을 붙인다. `F13::new(14)`는 `Fp::<13>::new(14)`와 같다.
    type F13 = Fp<13>;
    type F223 = Fp<223>;
    // 2^61 − 1. 소수이고, 여기서는 곱이 `u64`의 끝을 넘는다.
    type F61 = Fp<2_305_843_009_213_693_951>;

    /// L07 게이트(튜터가 써 둠). `new`는 P 이상인 수를 감아서 넣는다.
    #[test]
    fn new_reduces_mod_p() {
        assert_eq!(F13::new(0), F13::zero());
        assert_eq!(F13::new(13), F13::zero());
        assert_eq!(F13::new(14), F13::one());
        assert_eq!(F13::new(27).into_u64(), 1);
        assert_eq!(F223::new(1000).into_u64(), 1000 % 223);
    }

    /// L07 게이트(튜터가 써 둠). `zero`와 `one`은 0과 1이고, `new`로 만든 것과 같다.
    #[test]
    fn zero_one_are_reduced() {
        assert_eq!(F13::zero().into_u64(), 0);
        assert_eq!(F13::one().into_u64(), 1);
        assert_eq!(F13::new(13), F13::zero());
        assert_eq!(F13::new(14), F13::one());
    }

    /// L08 게이트(튜터가 써 둠). 합이 13 이상이면 감긴다.
    #[test]
    fn add_wraps() {
        assert_eq!(F13::new(9) + F13::new(7), F13::new(3));
        assert_eq!(F13::new(12) + F13::new(1), F13::zero());
        assert_eq!(F13::zero() + F13::zero(), F13::zero());
    }

    /// L08 게이트(튜터가 써 둠). 13칸 시계의 모든 a에서 `-a`를 본다.
    ///
    /// 줄마다 a와 `-a`를 찍고 나서 검사한다. 빨가면 어디까지 찍혔는지가 틀린 자리다.
    /// 초록일 때는 `cargo test neg_adds_to_zero -- --nocapture`로 본다.
    #[test]
    fn neg_adds_to_zero() {
        println!();
        println!("  mod 13");
        for value in 0..13 {
            let a = F13::new(value);
            let neg_a = (-a).into_u64();
            println!("  a = {value:>2}   -a = {neg_a:>2}");
            // `-a`도 시계 위의 수라서 13 미만이어야 한다.
            assert!(neg_a < 13, "-{value}가 13 미만이 아니다");
            // a에 더하면 0이 된다.
            assert_eq!(a + (-a), F13::zero(), "{value} + (-{value})");
        }
    }

    /// 깨 볼 대상(튜터가 써 둠). `u64` 두 개를 아무 검사 없이 그대로 뺀다.
    fn naive_u64_sub(a: u64, b: u64) -> u64 {
        a - b
    }

    /// L09에서 학습자가 쓴 테스트. `#[should_panic]`이 붙은 테스트는 패닉해야 초록이다.
    #[test]
    #[should_panic]
    fn naive_u64_sub_panics() {
        // 3 - 8은 0 아래로 내려가므로 패닉한다.
        naive_u64_sub(3, 8);
    }

    /// L09 게이트(튜터가 써 둠). 0 아래로 내려가는 뺄셈도 시계 위의 답을 낸다.
    #[test]
    fn additive_inverse_and_wraparound() {
        assert_eq!(F13::new(3) - F13::new(8), F13::new(8));
        assert_eq!(F13::zero() - F13::one(), F13::new(12));
        for x in 0..13 {
            let a = F13::new(x);
            assert_eq!(a - a, F13::zero(), "{x} - {x}");
            for y in 0..13 {
                let b = F13::new(y);
                // 빼고 나서 같은 수를 다시 더하면 처음 수로 돌아온다.
                assert_eq!((a - b) + b, a, "({x} - {y}) + {y}");
            }
        }
    }

    /// L11 게이트(튜터가 써 둠). 곱이 `u64`의 끝(약 1.8 · 10^19)을 넘는 경우에도 맞는 답을 낸다.
    #[test]
    fn mul_does_not_overflow_u64() {
        // `1 << 40`은 2를 40번 곱한 수다. 2^40 · 2^40 = 2^80이고, 2^61 − 1로 나눈 나머지는 2^19다.
        assert_eq!(F61::new(1 << 40) * F61::new(1 << 40), F61::new(1 << 19));
        // P − 1은 −1과 같은 수라서 제곱하면 1이다.
        let p_minus_1 = F61::new(2_305_843_009_213_693_950);
        assert_eq!(p_minus_1 * p_minus_1, F61::one());
    }

    /// L11 게이트(튜터가 써 둠). 0과 1이 하는 일.
    #[test]
    fn identities() {
        for x in 0..13 {
            let a = F13::new(x);
            assert_eq!(a + F13::zero(), a, "{x} + 0");
            assert_eq!(a * F13::one(), a, "{x} · 1");
            assert_eq!(a * F13::zero(), F13::zero(), "{x} · 0");
        }
    }

    /// L11 게이트(튜터가 써 둠). 순서를 바꾸거나 묶는 자리를 바꿔도 답이 같다.
    #[test]
    fn add_mul_commutative_associative() {
        for x in 0..13 {
            for y in 0..13 {
                let (a, b) = (F13::new(x), F13::new(y));
                assert_eq!(a + b, b + a, "{x} + {y}");
                assert_eq!(a * b, b * a, "{x} · {y}");
                for z in 0..13 {
                    let c = F13::new(z);
                    assert_eq!((a + b) + c, a + (b + c), "{x} + {y} + {z}");
                    assert_eq!((a * b) * c, a * (b * c), "{x} · {y} · {z}");
                }
            }
        }
    }

    /// L11 게이트(튜터가 써 둠). 곱셈을 덧셈 위로 펼칠 수 있다.
    #[test]
    fn distributive() {
        for x in 0..13 {
            for y in 0..13 {
                for z in 0..13 {
                    let (a, b, c) = (F13::new(x), F13::new(y), F13::new(z));
                    assert_eq!(a * (b + c), a * b + a * c, "{x} · ({y} + {z})");
                }
            }
        }
    }

    /// L11 게이트(튜터가 써 둠). 손으로 검산할 수 있는 값들.
    #[test]
    fn hand_checked_examples() {
        // mod 59: 44 + 33 = 77 = 59 + 18, 9 − 29 = −20 = −59 + 39
        assert_eq!(Fp::<59>::new(44) + Fp::<59>::new(33), Fp::<59>::new(18));
        assert_eq!(Fp::<59>::new(9) - Fp::<59>::new(29), Fp::<59>::new(39));
        // mod 97: 50 · 60 = 3000 = 30 · 97 + 90, 90 · 70 = 6300 = 64 · 97 + 92
        assert_eq!(
            Fp::<97>::new(50) * Fp::<97>::new(60) * Fp::<97>::new(70),
            Fp::<97>::new(92)
        );
    }

    /// 돌려 볼 것(튜터가 써 둠). 13칸 시계의 곱셈표를 찍는다.
    /// `cargo test multiplication_table -- --nocapture`로 본다.
    #[test]
    fn multiplication_table() {
        println!();
        println!("  mod 13 곱셈표 (가로 b, 세로 a, 칸은 a · b)");
        print!("      ");
        for y in 0..13 {
            print!("{y:>3}");
        }
        println!();
        for x in 0..13 {
            print!("  {x:>2} |");
            for y in 0..13 {
                let product = F13::new(x) * F13::new(y);
                print!("{:>3}", product.into_u64());
            }
            println!();
        }
        println!();
    }

    /// L07에서 돌려 본 것(튜터가 써 둠). 주석을 풀면 타입이 달라 컴파일되지 않는다.
    #[test]
    fn different_p_is_a_different_type() {
        let a = Fp::<13>::new(1);
        let b = Fp::<17>::new(1);
        // 안에 든 수는 둘 다 1이다.
        assert_eq!(a.into_u64(), b.into_u64());
        // 그런데 타입이 다르다. 아래 줄의 `// `를 지우면 E0308 expected `13`, found `17`이 난다.
        // assert_eq!(a, b);
    }
}
