//! U01 — Zatoshi 금액 타입.
//!
//! zatoshi는 ZEC의 최소 단위다(1 ZEC = 10^8 zatoshi). 합의 규칙은 총량 21,000,000 ZEC를
//! 넘는 금액을 허용하지 않으므로, 그 상한을 넘는 값은 이 타입으로 아예 만들 수 없게 한다.
//! 대조 기준: `zcash_protocol::value::Zatoshis` (dev-dependency, 테스트에서만 쓴다).

use core::ops::{Add, Sub};

/// 1 ZEC = 100,000,000 zatoshi.
pub const COIN: u64 = 100_000_000;
/// 합의 규칙상 존재할 수 있는 최대 zatoshi 수 (21,000,000 ZEC).
pub const MAX_MONEY: u64 = 21_000_000 * COIN;

/// 금액이 허용 범위를 벗어났을 때의 이유.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BalanceError {
    /// `MAX_MONEY`를 넘었다.
    Overflow,
    /// 0보다 작다.
    Underflow,
}

/// 0 이상 `MAX_MONEY` 이하의 zatoshi 금액.
///
/// 필드가 private이라 이 모듈 밖에서는 `from_u64` 같은 검사 생성자를 통해서만 만들 수 있다.
/// 그래서 "MAX_MONEY를 넘는 Zatoshis"는 프로그램 어디에도 존재할 수 없다. 이것이 불변식이다.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Zatoshis(u64);

impl Zatoshis {
    pub const ZERO: Zatoshis = Zatoshis(0);

    /// `0..=MAX_MONEY` 안이면 `Ok`, 넘으면 `Err(BalanceError::Overflow)`.
    pub fn from_u64(amount: u64) -> Result<Self, BalanceError> {
        if amount <= MAX_MONEY {
            Ok(Zatoshis(amount))
        } else {
            Err(BalanceError::Overflow)
        }
    }

    /// 음수면 `Err(BalanceError::Underflow)`, 그 외에는 `from_u64`와 같다.
    pub fn from_nonnegative_i64(amount: i64) -> Result<Self, BalanceError> {
        if amount < 0 {
            Err(BalanceError::Underflow)
        } else {
            Self::from_u64(amount as u64)
        }
    }

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
        let sum = self.0.checked_add(rhs.0)?;
        // if sum <= MAX_MONEY {
        //     Some(Zatoshis(sum))
        // } else {
        //     None
        // }
        // 중복된 검사 대신 from_u64를 사용하여 범위 검사를 위임
        Zatoshis::from_u64(sum).ok()
    }
}

/// 결과가 음수가 되면 `None`. (`checked_sub`를 쓰면 오버플로우가 발생하지 않고 `None`이 된다.)
impl Sub for Zatoshis {
    type Output = Option<Zatoshis>;

    fn sub(self, rhs: Zatoshis) -> Option<Zatoshis> {
        let diff = self.0.checked_sub(rhs.0)?;
        Some(Zatoshis(diff))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_money_is_21_million_zec() {
        assert_eq!(MAX_MONEY, 2_100_000_000_000_000);
        assert_eq!(MAX_MONEY, zcash_protocol::value::MAX_MONEY);
    }

    #[test]
    fn from_u64_accepts_zero_and_cap() {
        assert_eq!(Zatoshis::from_u64(0), Ok(Zatoshis::ZERO));
        assert_eq!(Zatoshis::from_u64(MAX_MONEY).map(Zatoshis::into_u64), Ok(MAX_MONEY));
    }

    #[test]
    fn from_u64_rejects_above_cap() {
        assert_eq!(Zatoshis::from_u64(MAX_MONEY + 1), Err(BalanceError::Overflow));
        assert_eq!(Zatoshis::from_u64(u64::MAX), Err(BalanceError::Overflow));
    }

    #[test]
    fn negative_is_underflow() {
        assert_eq!(Zatoshis::from_nonnegative_i64(-1), Err(BalanceError::Underflow));
        assert_eq!(Zatoshis::from_nonnegative_i64(i64::MIN), Err(BalanceError::Underflow));
        assert_eq!(Zatoshis::from_nonnegative_i64(7).map(Zatoshis::into_u64), Ok(7));
        assert_eq!(Zatoshis::from_nonnegative_i64(i64::MAX), Err(BalanceError::Overflow));
    }

    #[test]
    fn try_from_u64_delegates() {
        assert_eq!(Zatoshis::try_from(5u64), Zatoshis::from_u64(5));
        let z: Result<Zatoshis, BalanceError> = (MAX_MONEY + 1).try_into();
        assert_eq!(z, Err(BalanceError::Overflow));
    }

    #[test]
    fn add_stops_at_cap() {
        let one = Zatoshis::from_u64(1).unwrap();
        let two = Zatoshis::from_u64(2).unwrap();
        let cap = Zatoshis::from_u64(MAX_MONEY).unwrap();
        assert_eq!(one + two, Some(Zatoshis::from_u64(3).unwrap()));
        assert_eq!(cap + Zatoshis::ZERO, Some(cap));
        assert_eq!(cap + one, None);
    }

    #[test]
    fn sub_never_goes_negative() {
        let one = Zatoshis::from_u64(1).unwrap();
        let two = Zatoshis::from_u64(2).unwrap();
        assert_eq!(two - one, Some(one));
        assert_eq!(one - one, Some(Zatoshis::ZERO));
        assert_eq!(one - two, None);
    }

    /// 대조: 같은 입력에 대해 zcash_protocol과 같은 답을 내야 한다.
    #[test]
    fn agrees_with_zcash_protocol() {
        use zcash_protocol::value::Zatoshis as Ref;
        let samples = [0u64, 1, COIN, MAX_MONEY - 1, MAX_MONEY, MAX_MONEY + 1, u64::MAX];
        for &v in &samples {
            let ours = Zatoshis::from_u64(v).map(Zatoshis::into_u64).ok();
            let theirs = Ref::from_u64(v).map(Ref::into_u64).ok();
            assert_eq!(ours, theirs, "from_u64({v})");
        }
        for &a in &samples {
            for &b in &samples {
                let (Ok(oa), Ok(ob)) = (Zatoshis::from_u64(a), Zatoshis::from_u64(b)) else {
                    continue;
                };
                let (ra, rb) = (Ref::from_u64(a).unwrap(), Ref::from_u64(b).unwrap());
                assert_eq!((oa + ob).map(Zatoshis::into_u64), (ra + rb).map(Ref::into_u64), "{a} + {b}");
                assert_eq!((oa - ob).map(Zatoshis::into_u64), (ra - rb).map(Ref::into_u64), "{a} - {b}");
            }
        }
    }
}
