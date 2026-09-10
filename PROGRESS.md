# Shielded from Scratch — Progress

카운터: 유닛 1/31 · 마일스톤 M0/11 · 세션 1회 · 마지막 세션 2026-09-10

## 현재 유닛
U01 Zatoshi 금액 타입 (세션 0/1) — 다음 세션에 시작
게이트: `Zatoshis` newtype이 MAX_MONEY 초과·음수·오버플로를 거부하고 `zcash_protocol::value::Zatoshis`와 같은 값을 낸다
막힌 지점: —

## 세션 로그
- 2026-09-10 (U00, 1/1): 배운 것 — 트레이트의 associated const는 트레이트가 스코프에 있어야 보인다. `use ff::PrimeField as _;`로 이름 없이 들여올 수 있다. 막힌 것 — 보고 없음. MODULUS 문자열 형식이 불확실해 양쪽 `0x`를 벗기고 비교했다. 다음 — U01: `Cargo.toml`에서 `zcash_protocol` 주석을 풀고 `Zatoshis` newtype부터.

## 나중에
- LICENSE 파일 (공개 레포 전에 정하기)
