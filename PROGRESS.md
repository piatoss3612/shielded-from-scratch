# Shielded from Scratch — Progress

카운터: 유닛 3/31 · 마일스톤 M0/11 · 세션 3회 · 마지막 세션 2026-09-14

## 현재 유닛
U03 진짜 필드로 교체 (세션 0/2) — 다음 세션에 시작
게이트: 제네릭 Tonelli-Shanks `sqrt`가 `bls12_381::Scalar::sqrt()`와 일치, `to_repr`/`from_repr` 리틀엔디안 왕복
막힌 지점: —
준비물: U02의 `Fp<P>`에 `ff::PrimeField`가 요구하는 모양을 맞춰 보는 것부터. crate 지도 §2 `ff` 행.

## 세션 로그
- 2026-09-14 (U02, 1/1, 두 게이트 한 세션에): 배운 것 — u64 뺄셈은 감기기 전에 비교해야 한다(`a >= b`면 빼고, 아니면 `a + P - b`). 곱셈 중간값은 u128로. 0⁰ = 1은 빈 곱의 관례이고 `pow`에 특수 케이스가 없는 편이 안전하다. Fermat 축약 `exp % (P-1)`은 base 0에서 틀리므로 뺐다. 막힌 것 — Sub의 overflow 2회(힌트 2단까지), pow(0,0)=0 1회. 되돌려 말하기(U01 private 필드) — 정답. 다음 — U03: `ff::PrimeField` 트레이트가 요구하는 항목을 crate 문서에서 읽고, toy `Fp`와 `bls12_381::Scalar`의 대응표부터.
- 2026-09-14 (U01, 1/1): 배운 것 — newtype의 private 필드가 불변식을 가둔다. u64 오버플로 검사와 MAX_MONEY 검사는 별개이고 이 타입에선 후자가 진짜 게이트다. `as u64`는 `< 0` 검사 뒤에서만 안전하다. 막힌 것 — 보고 없음. 되돌려 말하기(U00 `use … as _` 이유) — 미응답, 마감에서 답 제공. 다음 — U02: 워밍업(2023 Go FieldElement 읽기) → `src/field.rs`에 `Fp<const P: u64>`.
- 2026-09-10 (U00, 1/1): 배운 것 — 트레이트의 associated const는 트레이트가 스코프에 있어야 보인다. `use ff::PrimeField as _;`로 이름 없이 들여올 수 있다. 막힌 것 — 보고 없음. MODULUS 문자열 형식이 불확실해 양쪽 `0x`를 벗기고 비교했다. 다음 — U01: `Cargo.toml`에서 `zcash_protocol` 주석을 풀고 `Zatoshis` newtype부터.

## 나중에
- U02 `Div`의 clippy `suspicious_arithmetic_impl` 오탐: `#[allow]` + 사유 주석. 선택.
- LICENSE 파일 (공개 레포 전에 정하기)
- U01 `Add`: 범위 규칙을 `from_u64` 한 곳에만 두는 리팩터(`Self::from_u64(sum).ok()`). 선택.
