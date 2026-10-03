//! W1 — 금액.
//!
//! zatoshi는 ZEC의 가장 작은 단위다. Zcash는 금액을 전부 zatoshi 정수로 센다.

/// 1 ZEC가 몇 zatoshi인가.
/// L01: 0을 맞는 값으로 바꾼다. 긴 숫자는 `100_000`처럼 밑줄로 끊어 써도 된다.
pub const COIN: u64 = 0;

/// Zcash에 존재할 수 있는 최대 금액, 2100만 ZEC를 zatoshi로 적은 값.
/// L01: 0을 맞는 값으로 바꾼다. 위의 `COIN`을 써도 된다.
pub const MAX_MONEY: u64 = 0;

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

    // L01: 위 두 본보기를 보고 테스트를 하나 직접 쓴다.
    //   이름: max_money_is_21_million_zec
    //   내용: 2100만 ZEC를 zatoshi로 바꾼 수를 손으로 곱해 숫자 그대로 적고,
    //         MAX_MONEY가 그 숫자와 같은지 assert_eq! 로 확인한다.
    //         COIN을 쓰지 않는 이유: MAX_MONEY를 COIN으로 정의했다면 같은 식끼리 비교하게 되어
    //         이 테스트는 절대 빨개지지 않는다. 숫자로 적어야 내 계산을 따로 확인할 수 있다.
    //   모양: #[test] 한 줄, fn 이름() { … } 한 덩어리.
}
