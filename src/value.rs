//! W1 — 금액.
//!
//! zatoshi는 ZEC의 가장 작은 단위다. Zcash는 금액을 전부 zatoshi 정수로 센다.

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

/// 0 이상 `MAX_MONEY` 이하의 zatoshi 금액.
///
/// 필드가 private이라 이 모듈 밖에서는 `from_u64`를 거쳐야만 만들 수 있다.
/// 그래서 `MAX_MONEY`를 넘는 `Zatoshis`는 프로그램 어디에도 존재할 수 없다.
// L02: 아래 derive 줄 앞의 `// `를 지우고, 그 바로 밑에 struct 한 줄을 쓴다.
//      모양은 위의 `Percent`와 같고, 안에 담는 타입만 `u64`다.
// #[derive(Debug, PartialEq, Clone, Copy)]

impl Zatoshis {
    /// 0 zatoshi. `impl` 안에 적은 `const`는 타입에 붙은 상수라 `Zatoshis::ZERO`로 쓴다.
    pub const ZERO: Zatoshis = Zatoshis(0);

    /// `0..=MAX_MONEY` 안이면 `Ok`, 넘으면 `Err(BalanceError::Overflow)`.
    pub fn from_u64(amount: u64) -> Result<Self, BalanceError> {
        // L02: 위 `Percent::from_u8`과 같은 모양으로 본문을 쓴다.
        todo!("L02 (amount = {amount})")
    }

    /// 안에 든 zatoshi 수를 꺼낸다. 필드가 private이라 모듈 밖에서 값을 읽는 길은 이것뿐이다.
    pub fn into_u64(self) -> u64 {
        self.0
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
        assert_eq!(Zatoshis::from_u64(MAX_MONEY).map(Zatoshis::into_u64), Ok(MAX_MONEY));
    }

    /// L02 게이트(튜터가 써 둠). 상한 바깥쪽(`MAX_MONEY + 1`과 `u64::MAX`)은 거절한다.
    #[test]
    fn from_u64_rejects_above_cap() {
        assert_eq!(Zatoshis::from_u64(MAX_MONEY + 1), Err(BalanceError::Overflow));
        assert_eq!(Zatoshis::from_u64(u64::MAX), Err(BalanceError::Overflow));
    }
}
