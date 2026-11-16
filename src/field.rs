//! W2 — 시계 위의 수를 타입으로.
//!
//! `clock.rs`에서는 시계의 칸 수를 함수마다 인자 `p`로 넘겼다. 여기서는 칸 수를 타입에 적어 둔다.
//! `Fp<13>`은 13칸 시계 위의 수이고 `Fp<17>`은 17칸 시계 위의 수다. 둘은 서로 다른 타입이다.

/// P칸 시계 위의 수. 안에 든 값은 언제나 0 이상 P 미만이다.
///
/// `<const P: u64>`는 타입 이름 옆에 타입이 아니라 값을 적는 자리다(const generics).
/// `Fp<13>`이라고 쓰면 이 정의 안의 `P`가 전부 13이 된다.
/// 필드는 `Zatoshis`처럼 private이라 `new`를 거쳐야만 만들 수 있다.
// L07: 아래 derive 줄 앞의 `// `를 지우고, 그 바로 밑에 struct 한 줄을 쓴다.
//      모양은 `value.rs`의 `pub struct Zatoshis(u64);`와 같다.
//      이름 `Fp` 바로 뒤에 `<const P: u64>`가 붙는 것만 다르다.
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]

// `impl<const P: u64> Fp<P>`는 "P가 무엇이든 `Fp<P>`에 아래 함수들을 붙인다"는 뜻이다.
impl<const P: u64> Fp<P> {
    /// 아무 `u64`나 받아 P칸 시계 위의 수로 만든다. P 이상이면 감긴다.
    pub fn new(value: u64) -> Self {
        // L07: 한 줄을 쓴다. `%`는 나머지 연산자다(17 % 12는 5).
        //      이 안에서 `P`는 보통 상수처럼 쓸 수 있다.
        todo!("L07 (value = {value}, P = {P})")
    }

    /// 0. 어느 시계에나 있다.
    pub fn zero() -> Self {
        // L07: 한 줄을 쓴다.
        todo!("L07 (zero)")
    }

    /// 1.
    pub fn one() -> Self {
        // L07: 한 줄을 쓴다.
        todo!("L07 (one)")
    }

    /// 안에 든 수를 꺼낸다. 필드가 private이라 모듈 밖에서 값을 읽는 길은 이것뿐이다.
    pub fn into_u64(self) -> u64 {
        self.0
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

    /// 돌려 볼 것(튜터가 써 둠). 두 게이트가 초록이 된 뒤에 본다.
    #[test]
    fn different_p_is_a_different_type() {
        let a = Fp::<13>::new(1);
        let b = Fp::<17>::new(1);
        // 안에 든 수는 둘 다 1이다.
        assert_eq!(a.into_u64(), b.into_u64());
        // 그런데 타입이 다르다. 아래 줄의 `// `를 지우고 `cargo test field`를 돌려 에러를 읽은 뒤 되돌린다.
        // assert_eq!(a, b);
    }
}
