//! W2 — 시계 위의 수를 타입으로.
//!
//! `clock.rs`에서는 시계의 칸 수를 함수마다 인자 `p`로 넘겼다. 여기서는 칸 수를 타입에 적어 둔다.
//! `Fp<13>`은 13칸 시계 위의 수이고 `Fp<17>`은 17칸 시계 위의 수다. 둘은 서로 다른 타입이다.

// `+`, 앞에 붙는 `-`, 사이에 오는 `-`에 딸린 트레이트.
// 구현해 두면 `a + b`는 `add`를, `-a`는 `neg`를, `a - b`는 `sub`를 부른다.
use std::ops::{Add, Neg, Sub};

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

    fn sub(self, rhs: Self) -> Self {
        // L09: 먼저 `Self(self.0 - rhs.0)`이라고 그대로 쓰고 `cargo test field`가 어떻게 깨지는지 본다.
        //      그다음 이미 만든 `+`와 `-a`만 써서 고친다.
        todo!("L09 ({} - {}, P = {P})", self.0, rhs.0)
    }
}

#[cfg(test)]
mod axioms {
    use super::*;

    // `type`은 타입에 짧은 별명을 붙인다. `F13::new(14)`는 `Fp::<13>::new(14)`와 같다.
    type F13 = Fp<13>;
    type F223 = Fp<223>;

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

    // L09: 여기에 테스트 `naive_u64_sub_panics`를 쓴다.
    //      `#[test]` 밑에 `#[should_panic]`을 한 줄 더 붙이면 "패닉해야 초록"인 테스트가 된다.
    //      본문에서는 `naive_u64_sub`에 3과 8을 넣어 부른다.

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
