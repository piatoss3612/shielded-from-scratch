# Shielded from Scratch — Progress

클리어 월드 0 · 클리어 레벨 1 · 지금 W1 금액 1/5 · 이 월드의 보스 L05 정답지와 대조

## 지금 레벨
L02 상한이 있는 금액 타입 — 고치기, 35분
새로 배우는 것: private 필드를 가진 newtype과 Result를 돌려주는 검사 생성자
본보기: src/value.rs의 튜터 형제 코드 `Percent(u8)::from_u8`(100을 넘으면 Err)
먼저 할 것: `cargo test value`를 돌려, `Zatoshis` 정의가 없어서 `ZERO`가 컴파일되지 않는 첫 red 보기
채울 곳: `pub struct Zatoshis(u64);` 한 줄, `from_u64` 본문(상한 검사 후 Ok/Err)
통과 기준: `cargo test value`에서 `from_u64_accepts_zero_and_cap`, `from_u64_rejects_above_cap`이 초록(L01의 3개도 그대로 초록)
막힌 지점: —

## W1 금액 (1/5)
- [x] L01 첫 green: 1 ZEC와 2100만 ZEC
- [ ] L02 상한이 있는 금액 타입
- [ ] L03 음수 거르기와 TryFrom
- [ ] L04 금액 덧셈·뺄셈
- [ ] L05 정답지와 대조 (보스)

## 세션 로그
형식: 날짜 (레벨): 배운 것 — 학습자가 말한 '이 깃발이 증명한 것' 한 문장. 막힌 것 — 설명하지 못한 줄 포함. 되돌려 말하기 — 바로 앞 레벨 질문에 정답/미응답. 다음 — 다음 레벨 ID.
- 2026-10-11 (L01): 배운 것 — "이제 ZEC을 base unit으로 환산한 값과, Zcash 블록체인에서의 전체 발행량이 2100만 ZEC이다 라는 것을 증명했지." 막힌 것 — 테스트에서 `COIN` 대신 숫자를 쓴 이유를 "딱히 의미는 없다"고 설명했다. `COIN`으로 비교하면 같은 식끼리 비교라 절대 빨개지지 않는다는 점은 튜터가 알려 줬다. L02를 열 때 다시 본다. 되돌려 말하기 — 건너뜀(첫 레벨). 다음 — L02.
- 2026-10-03 (새 출발, 레벨 아님): 처음부터 다시 시작했다. 브랜치 `w01-amount`에서 L01부터 연다. v2(2026-09-10~15)의 코드와 세션 로그는 태그 `v2-archive`에 그대로 있다(`git show v2-archive:PROGRESS.md`). 다음 — L01.

## 복습
레벨을 깨면 다시 볼 것을 여기에 적는다.
- L01: `assert_eq!(a, b)`의 left와 right는 자리로 정해진다(첫째가 left, 둘째가 right). 실패하면 두 값을 다 보여 준다.
- L01: 테스트는 빨개질 수 있어야 한다. `MAX_MONEY`를 `COIN`으로 정의했으면 테스트에서는 `COIN`을 쓰지 않는다.
- L01: 이 깃발이 확인한 것은 체인이 아니라 "내 상수가 진짜 Zcash 코드(zcash_protocol)의 상수와 같다"는 것이다.

## 나중에
- LICENSE 파일 정하기 (레포는 이미 공개)
