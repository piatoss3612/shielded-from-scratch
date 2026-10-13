//! W1 — 금액.
//!
//! zatoshi는 ZEC의 가장 작은 단위다. Zcash는 금액을 전부 zatoshi 정수로 센다.

// `+`와 `-` 연산자에 붙은 트레이트. 구현해 두면 `a + b`, `a - b`가 그 타입의 `add`, `sub` 함수를 부른다.
use std::ops::{Add, Sub};

/// 1 ZEC가 몇 zatoshi인가.
/// 긴 숫자는 `100_000_000`처럼 밑줄로 끊어 써도 된다. Rust는 숫자 속 밑줄을 무시한다.
pub const COIN: u64 = 100_000_000;

/// Zcash에 존재할 수 있는 최대 금액, 2100만 ZEC를 zatoshi로 적은 값.
pub const MAX_MONEY: u64 = COIN * 21_000_000;

/// 금액을 만들 수 없을 때의 이유. `enum`은 적어 둔 경우 가운데 하나가 되는 타입이다.
///
/// `derive(…)`는 이 타입에 출력(`Debug`, `{:?}`)·비교(`PartialEq`, `==`)·복사(`Clone`, `Copy`)를
/// 자동으로 붙여 달라는 주문이다. `assert_eq!`는 두 값을 `==`로 비교하고 실패하면 `{:?}`로 찍으므로,
/// 테스트에서 비교하는 타입에는 `Debug`와 `PartialEq`가 있어야 한다.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BalanceError {
    /// `MAX_MONEY`를 넘었다.
    Overflow,
    /// 0보다 작다.
    Underflow,
}

/// 본보기(튜터가 써 둠). 0부터 100까지만 담는 백분율. Zcash와는 상관없고, 아래 `Zatoshis`와 모양이 같아서 둔다.
///
/// `struct 이름(타입);`처럼 괄호 안에 타입만 적은 구조체를 튜플 구조체라 하고, 안의 값은 `.0`으로 꺼낸다.
/// 필드 앞에 `pub`이 없으면(private) 이 모듈 밖에서는 `Percent(150)`처럼 직접 만들 수 없고 `from_u8`을 거쳐야 한다.
/// 그래서 100을 넘는 `Percent`는 어디에도 만들어질 수 없다. 오류 이유는 `Zatoshis`와 같은 `BalanceError`를 빌려 쓴다.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percent(u8);

impl Percent {
    /// 100 이하면 `Ok`, 넘으면 `Err(BalanceError::Overflow)`.
    ///
    /// `Result<A, E>`는 성공하면 `Ok(A 값)`, 실패하면 `Err(E 값)`인 타입이다.
    /// `Self`는 지금 `impl`하고 있는 타입(여기서는 `Percent`)을 가리킨다.
    pub fn from_u8(value: u8) -> Result<Self, BalanceError> {
        if value <= 100 {
            Ok(Percent(value))
        } else {
            Err(BalanceError::Overflow)
        }
    }
}

/// 본보기(튜터가 써 둠). `u8`에서 `Percent`로 가는, 실패할 수도 있는 변환.
///
/// 트레이트는 "이 이름의 함수를 이런 모양으로 갖고 있다"는 약속이고, `impl 트레이트 for 타입`으로 그 약속을 지킨다.
/// `TryFrom<u8>`은 표준 라이브러리의 약속이라, 구현해 두면 누구나 `Percent::try_from(50u8)`이라는 같은 이름으로 부른다.
/// `type Error = …`는 실패할 때 돌려줄 오류 타입을 정하고, 아래 `Self::Error`가 그것을 가리킨다.
/// 검사는 새로 쓰지 않고 이미 있는 `from_u8`에 맡긴다(위임). 그리고 `TryFrom`을 구현하면 `50u8.try_into()`도 저절로 생긴다.
impl TryFrom<u8> for Percent {
    type Error = BalanceError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
    }
}

/// 본보기(튜터가 써 둠). 두 `Percent`의 합. 100을 넘으면 `None`.
impl Add for Percent {
    /// `+`의 결과 타입. 합이 상한을 넘을 수 있어서 `Percent`가 아니라 `Option<Percent>`로 둔다.
    /// `Option<T>`는 값이 있으면 `Some(값)`, 없으면 `None`인 타입이다.
    type Output = Option<Percent>;

    /// `checked_add`는 정수끼리 더하다 그 타입의 끝을 넘으면 `None`, 아니면 `Some(합)`을 준다.
    /// 정수 타입에는 `checked_sub`(0 아래로 내려가면 `None`)도 같은 모양으로 있다.
    /// `?`는 `None`이면 그 자리에서 함수 전체가 `None`을 돌려주고 끝나고, `Some`이면 안의 값을 꺼낸다.
    /// `.ok()`는 `Result`를 `Option`으로 바꾼다. `Ok(v)`는 `Some(v)`, `Err`는 이유를 버리고 `None`이 된다.
    /// 상한 검사는 여기서 새로 쓰지 않고 `from_u8`에 맡긴다.
    fn add(self, rhs: Percent) -> Option<Percent> {
        Self::from_u8(self.0.checked_add(rhs.0)?).ok()
    }
}

/// 0 이상 `MAX_MONEY` 이하의 zatoshi 금액.
///
/// 필드가 private이라 이 모듈 밖에서는 `from_u64`를 거쳐야만 만들 수 있다.
/// 그래서 `MAX_MONEY`를 넘는 `Zatoshis`는 프로그램 어디에도 존재할 수 없다.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Zatoshis(u64);

impl Zatoshis {
    /// 0 zatoshi. `impl` 안에 적은 `const`는 타입에 붙은 상수라 `Zatoshis::ZERO`로 쓴다.
    pub const ZERO: Zatoshis = Zatoshis(0);

    /// `0..=MAX_MONEY` 안이면 `Ok`, 넘으면 `Err(BalanceError::Overflow)`.
    pub fn from_u64(amount: u64) -> Result<Self, BalanceError> {
        if amount <= MAX_MONEY {
            Ok(Zatoshis(amount))
        } else {
            Err(BalanceError::Overflow)
        }
    }

    /// 음수면 `Err(BalanceError::Underflow)`, 그 밖에는 `from_u64`와 같다.
    ///
    /// `amount as u64`는 `i64` 값을 `u64`로 옮긴다. 음수가 아님을 확인한 뒤라면 값이 그대로 옮겨진다.
    pub fn from_nonnegative_i64(amount: i64) -> Result<Self, BalanceError> {
        // L03: 음수를 먼저 거르고, 나머지 검사는 `from_u64`에 맡긴다.
        if amount < 0 {
            Err(BalanceError::Underflow)
        } else {
            Self::from_u64(amount as u64)
        }
    }

    /// 안에 든 zatoshi 수를 꺼낸다. 필드가 private이라 모듈 밖에서 값을 읽는 길은 이것뿐이다.
    pub fn into_u64(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for Zatoshis {
    type Error = BalanceError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::from_u64(value)
    }
}

/// 합이 `MAX_MONEY`를 넘으면 `None`.
impl Add for Zatoshis {
    type Output = Option<Zatoshis>;

    fn add(self, rhs: Zatoshis) -> Option<Zatoshis> {
        Self::try_from(self.0.checked_add(rhs.into_u64())?).ok()
    }
}

/// 결과가 0보다 작아지면 `None`.
impl Sub for Zatoshis {
    type Output = Option<Zatoshis>;

    fn sub(self, rhs: Zatoshis) -> Option<Zatoshis> {
        // L04: `add`와 같은 모양으로 한 줄을 쓴다. 뺄셈이 0 아래로 내려가는 경우를 막는 것이 핵심이다.
        Self::try_from(self.0.checked_sub(rhs.into_u64())?).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 본보기(튜터가 써 둠). 진짜 Zcash crate가 쓰는 상수와 내 상수를 비교한다.
    #[test]
    fn coin_matches_zcash_protocol() {
        assert_eq!(COIN, zcash_protocol::value::COIN);
    }

    #[test]
    fn max_money_matches_zcash_protocol() {
        assert_eq!(MAX_MONEY, zcash_protocol::value::MAX_MONEY);
    }

    /// L01에서 학습자가 쓴 테스트. `COIN` 없이 따로 곱한 값과 비교한다.
    #[test]
    fn max_money_is_21_million_zec() {
        assert_eq!(MAX_MONEY, 21_000_000 * 100_000_000);
    }

    /// 본보기(튜터가 써 둠). 경계값 0, 100, 101에서 `Percent::from_u8`이 내는 답.
    #[test]
    fn percent_from_u8_checks_cap() {
        assert_eq!(Percent::from_u8(0), Ok(Percent(0)));
        assert_eq!(Percent::from_u8(100), Ok(Percent(100)));
        assert_eq!(Percent::from_u8(101), Err(BalanceError::Overflow));
    }

    /// L02 게이트(튜터가 써 둠). 상한 안쪽(0과 `MAX_MONEY`)은 받는다.
    #[test]
    fn from_u64_accepts_zero_and_cap() {
        assert_eq!(Zatoshis::from_u64(0), Ok(Zatoshis::ZERO));
        // `.map(Zatoshis::into_u64)`: `Ok` 안의 값에만 `into_u64`를 적용하고, `Err`는 그대로 둔다.
        assert_eq!(
            Zatoshis::from_u64(MAX_MONEY).map(Zatoshis::into_u64),
            Ok(MAX_MONEY)
        );
    }

    /// L02 게이트(튜터가 써 둠). 상한 바깥쪽(`MAX_MONEY + 1`과 `u64::MAX`)은 거절한다.
    #[test]
    fn from_u64_rejects_above_cap() {
        assert_eq!(
            Zatoshis::from_u64(MAX_MONEY + 1),
            Err(BalanceError::Overflow)
        );
        assert_eq!(Zatoshis::from_u64(u64::MAX), Err(BalanceError::Overflow));
    }

    /// 본보기(튜터가 써 둠). `Percent::try_from`은 `from_u8`과 같은 답을 내고, `try_into()`로도 부를 수 있다.
    #[test]
    fn percent_try_from_delegates() {
        // 숫자 뒤의 `u8`은 그 숫자의 타입을 정해 준다.
        assert_eq!(Percent::try_from(100u8), Percent::from_u8(100));
        // `try_into()`는 무엇으로 바꿀지를 왼쪽에 적은 타입을 보고 정한다.
        let p: Result<Percent, BalanceError> = 101u8.try_into();
        assert_eq!(p, Err(BalanceError::Overflow));
    }

    /// 본보기(튜터가 써 둠). `Percent`끼리의 `+`는 100까지는 `Some`, 넘으면 `None`.
    #[test]
    fn percent_add_stops_at_cap() {
        // `.unwrap()`은 `Ok(v)`면 `v`를 꺼내고 `Err`면 패닉한다. 실패하지 않을 것을 아는 테스트에서만 쓴다.
        let thirty = Percent::from_u8(30).unwrap();
        let seventy = Percent::from_u8(70).unwrap();
        assert_eq!(thirty + seventy, Some(Percent(100)));
        assert_eq!(seventy + seventy, None);
    }

    /// L03 게이트(튜터가 써 둠). 음수는 `Underflow`, 상한을 넘으면 그대로 `Overflow`.
    #[test]
    fn negative_is_underflow() {
        assert_eq!(
            Zatoshis::from_nonnegative_i64(-1),
            Err(BalanceError::Underflow)
        );
        assert_eq!(
            Zatoshis::from_nonnegative_i64(i64::MIN),
            Err(BalanceError::Underflow)
        );
        assert_eq!(
            Zatoshis::from_nonnegative_i64(7).map(Zatoshis::into_u64),
            Ok(7)
        );
        assert_eq!(
            Zatoshis::from_nonnegative_i64(i64::MAX),
            Err(BalanceError::Overflow)
        );
    }

    /// L03 게이트(튜터가 써 둠). `Zatoshis::try_from`은 `from_u64`와 같은 답을 낸다.
    #[test]
    fn try_from_u64_delegates() {
        assert_eq!(Zatoshis::try_from(5u64), Zatoshis::from_u64(5));
        let z: Result<Zatoshis, BalanceError> = (MAX_MONEY + 1).try_into();
        assert_eq!(z, Err(BalanceError::Overflow));
    }

    /// L04 게이트(튜터가 써 둠). 덧셈은 `MAX_MONEY`에서 멈춘다.
    #[test]
    fn add_stops_at_cap() {
        let one = Zatoshis::from_u64(1).unwrap();
        let two = Zatoshis::from_u64(2).unwrap();
        let cap = Zatoshis::from_u64(MAX_MONEY).unwrap();
        assert_eq!(one + two, Some(Zatoshis::from_u64(3).unwrap()));
        assert_eq!(cap + Zatoshis::ZERO, Some(cap));
        assert_eq!(cap + one, None);
    }

    /// L04 게이트(튜터가 써 둠). 뺄셈은 0 아래로 내려가지 않는다.
    #[test]
    fn sub_never_goes_negative() {
        let one = Zatoshis::from_u64(1).unwrap();
        let two = Zatoshis::from_u64(2).unwrap();
        assert_eq!(two - one, Some(one));
        assert_eq!(one - one, Some(Zatoshis::ZERO));
        assert_eq!(one - two, None);
    }
}
