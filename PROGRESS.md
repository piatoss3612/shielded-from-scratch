# Shielded from Scratch — Progress

클리어 월드 0 · 클리어 레벨 0 · 지금 W1 금액 0/5 · 이 월드의 보스 L05 정답지와 대조

## 지금 레벨
L01 첫 green: 1 ZEC와 2100만 ZEC — 쓰기, 30분
새로 배우는 것: 테스트 하나가 빨강에서 초록으로 바뀌는 흐름(`#[test]`, `assert_eq!`, `cargo test` 출력 읽기)
본보기: src/value.rs의 튜터 테스트 `coin_matches_zcash_protocol`
먼저 할 것: `cargo test value`를 돌려 빨간 테스트 2개를 보고, 실패 메시지의 `left`와 `right`가 각각 무엇인지 읽기
채울 곳: `COIN`, `MAX_MONEY`, 새 테스트 `max_money_is_21_million_zec`(COIN 대신 손으로 곱한 숫자와 비교)
통과 기준: `cargo test value`에서 3 passed
막힌 지점: —

## W1 금액 (0/5)
- [ ] L01 첫 green: 1 ZEC와 2100만 ZEC
- [ ] L02 상한이 있는 금액 타입
- [ ] L03 음수 거르기와 TryFrom
- [ ] L04 금액 덧셈·뺄셈
- [ ] L05 정답지와 대조 (보스)

## 세션 로그
형식: 날짜 (레벨): 배운 것 — 학습자가 말한 '이 깃발이 증명한 것' 한 문장. 막힌 것 — 설명하지 못한 줄 포함. 다음 — 다음 레벨 ID.
- 2026-10-03 (새 출발, 레벨 아님): 처음부터 다시 시작했다. 브랜치 `w01-amount`에서 L01부터 연다. v2(2026-09-10~15)의 코드와 세션 로그는 태그 `v2-archive`에 그대로 있다(`git show v2-archive:PROGRESS.md`). 다음 — L01.

## 복습
레벨을 깨면 다시 볼 것을 여기에 적는다.

## 나중에
- LICENSE 파일 정하기 (레포는 이미 공개)
