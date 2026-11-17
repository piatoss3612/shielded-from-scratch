//! W2 — 진짜 Zcash 필드.
//!
//! `field.rs`의 `Fp<13>`은 13에서 감긴다. Zcash가 증명에 쓰는 수는 아주 큰 소수 r에서 감긴다.
//! 그 수의 타입이 `Scalar`다. `bls12_381`은 그 타입이 들어 있는 crate의 이름일 뿐이다.

/// Zcash 증명 필드가 감기는 소수 r. Zcash 프로토콜 명세에 적힌 값을 16진수로 옮겼다. 255비트다.
pub const SPEC_R: &str = "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

/// 아직 적지 않은 16진수 예측을 나타내는 값.
pub const UNSET_HEX: &str = "";

// ── 내 예측 (L10) ──────────────────────────────────────────────
// 셋을 다 적으면 실행하기 전에 튜터에게 말한다. 튜터가 커밋한 뒤에 돌린다.

/// r − 1을 16진수로 적는다. `SPEC_R`과 같은 모양이다(`0x`로 시작, 소문자, 그 뒤 64자리).
pub const R_MINUS_1_HEX: &str = "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000";

/// 이 필드에서 2 − 3과 0 − 1은 같은 수인가.
/// `None`을 지우고 같다면 `Some(true)`, 다르다면 `Some(false)`를 적는다.
pub const TWO_MINUS_THREE_IS_ZERO_MINUS_ONE: Option<bool> = Some(true);

/// 이 필드에서 5 · 6은 30과 같은 수인가. mod 13 시계에서는 4였다.
/// `None`을 지우고 같다면 `Some(true)`, 다르다면 `Some(false)`를 적는다.
pub const FIVE_TIMES_SIX_IS_THIRTY: Option<bool> = Some(true);

#[cfg(test)]
mod tests {
    use super::*;
    // Zcash 증명 필드의 수. 이 파일에서는 테스트만 쓴다.
    use bls12_381::Scalar;
    // `Scalar::MODULUS`는 `PrimeField`라는 트레이트에 딸린 상수라서, 그 트레이트를 가져와야 보인다.
    // `as _`는 이름은 쓰지 않고 가져오기만 한다는 뜻이다.
    use ff::PrimeField as _;

    /// L10 게이트(튜터가 써 둠). crate가 쓰는 r이 명세의 r과 같다.
    #[test]
    fn sapling_scalar_field_modulus_matches_spec() {
        assert_eq!(Scalar::MODULUS, SPEC_R);
    }

    /// L10 게이트. 이 필드에서 0 − 1이 어디로 감기는지 본다.
    #[test]
    fn zero_minus_one_wraps() {
        assert!(
            R_MINUS_1_HEX != UNSET_HEX,
            "먼저 `R_MINUS_1_HEX` 예측을 적는다"
        );

        // 배선(튜터가 써 둠). `{:?}`는 `Scalar`를 `0x`로 시작하는 16진수 글자로 찍는다.
        let zero_minus_one = Scalar::zero() - Scalar::one();
        let printed = format!("{zero_minus_one:?}");

        // L10: `printed`가 내가 적은 `R_MINUS_1_HEX`와 같다고 단언하는 한 줄을 쓴다.
        //      다 쓰면 아래 `todo!` 줄을 지운다.
        assert_eq!(printed, R_MINUS_1_HEX, "0 - 1이 어디로 감기는지 본다");
    }

    /// L10 게이트(튜터가 써 둠). 내 예측 셋을 진짜 필드의 답과 견준다.
    ///
    /// 예측을 다 적기 전에는 필드의 답을 찍지 않는다. 다 적은 뒤에는 견준 표를 찍는다.
    /// 테스트가 빨가면 표가 저절로 보이고, 초록일 때는 `cargo test real_field -- --nocapture`로 본다.
    #[test]
    fn my_scalar_predictions() {
        // `.is_some()`은 `Some(…)`이면 true, `None`이면 false다.
        let all_written = R_MINUS_1_HEX != UNSET_HEX
            && TWO_MINUS_THREE_IS_ZERO_MINUS_ONE.is_some()
            && FIVE_TIMES_SIX_IS_THIRTY.is_some();
        assert!(all_written, "아직 적지 않은 예측이 있다. 셋을 다 적는다");

        let zero_minus_one = format!("{:?}", Scalar::zero() - Scalar::one());
        let sub_same = Scalar::from(2) - Scalar::from(3) == Scalar::zero() - Scalar::one();
        let mul_same = Scalar::from(5) * Scalar::from(6) == Scalar::from(30);

        let hex_ok = R_MINUS_1_HEX == zero_minus_one;
        let sub_ok = TWO_MINUS_THREE_IS_ZERO_MINUS_ONE == Some(sub_same);
        let mul_ok = FIVE_TIMES_SIX_IS_THIRTY == Some(mul_same);

        println!();
        println!("  Zcash 증명 필드");
        println!("  r        {SPEC_R}");
        println!("  0 - 1    {zero_minus_one}");
        println!("  내 예측  {R_MINUS_1_HEX}   {}", mark(hex_ok));
        println!();
        println!(
            "  2 - 3 과 0 - 1 이 같은가   내 예측 {:?}   필드 {sub_same}   {}",
            TWO_MINUS_THREE_IS_ZERO_MINUS_ONE,
            mark(sub_ok)
        );
        println!(
            "  5 · 6 과 30 이 같은가      내 예측 {:?}   필드 {mul_same}   {}",
            FIVE_TIMES_SIX_IS_THIRTY,
            mark(mul_ok)
        );
        println!();

        assert!(hex_ok && sub_ok && mul_ok, "표에서 '다름'인 줄을 본다");
    }

    fn mark(ok: bool) -> &'static str {
        if ok { "맞음" } else { "다름" }
    }
}
