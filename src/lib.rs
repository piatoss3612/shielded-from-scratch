//! sfs — Shielded from Scratch
//!
//! Sapling 차폐 트랜잭션을 밑바닥부터 만들며 배우는 학습용 crate.
//! 레벨을 하나 깰 때마다 코드가 조금씩 늘어난다.

pub mod clock;
// L07: 슬롯을 열 때 튜터가 아래 줄의 `// `를 지운다. 그 전에는 `field.rs`가 빌드에 들어가지 않는다.
// pub mod field;
pub mod value;
