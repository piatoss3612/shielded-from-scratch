//! W2 — 시계처럼 도는 수.
//!
//! 시계는 12를 지나면 다시 1부터 센다. 이 파일의 시계는 13칸이고 0부터 12까지 있다.
//! 12 다음은 0이다.

/// 시계의 칸 수.
pub const P: i64 = 13;

/// 아직 적지 않은 예측을 나타내는 값. 시계에는 없는 수라서 테스트가 빨갛게 남는다.
pub const UNSET: i64 = -1;

// ── 내 예측 (L06) ──────────────────────────────────────────────
// 0부터 12 사이의 수. 실행하기 전에 적고 커밋했다(`wip: L06 clock predictions`).

/// 9 + 7
pub const NINE_PLUS_SEVEN: i64 = 3;

/// 3 − 8
pub const THREE_MINUS_EIGHT: i64 = 8;

/// 5 · 6
pub const FIVE_TIMES_SIX: i64 = 4;

/// −4 (4에 더하면 0이 되는 수)
pub const NEG_FOUR: i64 = 9;

/// 5 · b가 1이 되는 b
pub const B_WHERE_FIVE_B_IS_ONE: i64 = 8;

// ── 시계 (튜터가 써 둠) ─────────────────────────────────────────
// `x.rem_euclid(p)`는 x를 p로 나눈 나머지를 0 이상 p 미만으로 돌려준다.

/// 시계 위의 덧셈.
pub fn add_mod(a: i64, b: i64, p: i64) -> i64 {
    (a + b).rem_euclid(p)
}

/// 시계 위의 뺄셈.
pub fn sub_mod(a: i64, b: i64, p: i64) -> i64 {
    (a - b).rem_euclid(p)
}

/// 시계 위의 곱셈.
pub fn mul_mod(a: i64, b: i64, p: i64) -> i64 {
    (a * b).rem_euclid(p)
}

/// 시계 위의 부호 바꾸기.
pub fn neg_mod(a: i64, p: i64) -> i64 {
    (-a).rem_euclid(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// L06 게이트(튜터가 써 둠). 내 예측 다섯 개를 시계의 답과 견준다.
    ///
    /// 예측을 다 적기 전에는 시계의 답을 찍지 않는다. 다 적은 뒤에는 견준 표를 찍는다.
    /// 테스트가 빨가면 표가 저절로 보이고, 초록일 때는 `cargo test clock -- --nocapture`로 본다.
    #[test]
    fn my_clock_predictions() {
        // (식, 내 예측, 시계의 답)
        let rows = [
            ("9 + 7", NINE_PLUS_SEVEN, add_mod(9, 7, P)),
            ("3 − 8", THREE_MINUS_EIGHT, sub_mod(3, 8, P)),
            ("5 · 6", FIVE_TIMES_SIX, mul_mod(5, 6, P)),
            ("−4", NEG_FOUR, neg_mod(4, P)),
        ];
        let b = B_WHERE_FIVE_B_IS_ONE;

        // 하나라도 `UNSET`이면 시계의 답을 찍지 않고 여기서 멈춘다.
        let mut all_written = b != UNSET;
        for (_, mine, _) in rows {
            if mine == UNSET {
                all_written = false;
            }
        }
        assert!(
            all_written,
            "`UNSET`으로 남은 예측이 있다. 다섯 개를 다 적는다"
        );

        println!();
        println!("  mod {P} 시계");
        println!("  식          내 예측   시계");

        let mut all_match = true;
        for (expr, mine, clock) in rows {
            let mark = if mine == clock { "맞음" } else { "다름" };
            all_match = all_match && mine == clock;
            println!("  {expr:<10}{mine:>9}{clock:>7}   {mark}");
        }

        // 마지막 예측은 답을 견주지 않고, 5에 곱해서 정말 1이 되는지 본다.
        let product = mul_mod(5, b, P);
        let b_ok = (0..P).contains(&b) && product == 1;
        all_match = all_match && b_ok;
        let mark = if b_ok { "맞음" } else { "다름" };
        println!("  5 · {b} = {product}  (1이어야 한다)   {mark}");
        println!();

        assert!(all_match, "표에서 '다름'인 줄을 본다");
    }
}
