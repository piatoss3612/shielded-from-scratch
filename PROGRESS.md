# Shielded from Scratch — Progress

카운터: 유닛 2/31 · 마일스톤 M0/11 · 세션 2회 · 마지막 세션 2026-09-14

## 현재 유닛
U02 toy 유한체 `Fp<const P>` (세션 0/2) — 다음 세션에 시작
게이트: 필드 공리 테스트(덧셈·곱셈 결합/교환/분배, 항등원, 역원), Fermat 역원, `pow`
막힌 지점: —
워밍업: 세션 시작 15분은 2023년 bitcoin-from-scratch chapter01의 Go `FieldElement`를 다시 읽는다 (https://github.com/piatoss3612/bitcoin-from-scratch/tree/chapter01, 블로그 /15)

## 세션 로그
- 2026-09-14 (U01, 1/1): 배운 것 — newtype의 private 필드가 불변식을 가둔다. u64 오버플로 검사와 MAX_MONEY 검사는 별개이고 이 타입에선 후자가 진짜 게이트다. `as u64`는 `< 0` 검사 뒤에서만 안전하다. 막힌 것 — 보고 없음. 되돌려 말하기(U00 `use … as _` 이유) — 미응답, 마감에서 답 제공. 다음 — U02: 워밍업(2023 Go FieldElement 읽기) → `src/field.rs`에 `Fp<const P: u64>`.
- 2026-09-10 (U00, 1/1): 배운 것 — 트레이트의 associated const는 트레이트가 스코프에 있어야 보인다. `use ff::PrimeField as _;`로 이름 없이 들여올 수 있다. 막힌 것 — 보고 없음. MODULUS 문자열 형식이 불확실해 양쪽 `0x`를 벗기고 비교했다. 다음 — U01: `Cargo.toml`에서 `zcash_protocol` 주석을 풀고 `Zatoshis` newtype부터.

## 나중에
- LICENSE 파일 (공개 레포 전에 정하기)
- U01 `Add`: 범위 규칙을 `from_u64` 한 곳에만 두는 리팩터(`Self::from_u64(sum).ok()`). 선택.
