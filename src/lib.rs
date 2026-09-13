//! sfs — Shielded from Scratch
//!
//! Sapling 차폐 트랜잭션을 밑바닥부터 만들며 배우는 학습용 crate.
//! 유닛이 끝날 때마다 `pub mod`가 하나씩 늘어난다.

pub mod value;

#[cfg(test)]
mod u00_first_green {
    use ff::PrimeField as _;

    /// BLS12-381 scalar field의 modulus r. Zcash protocol spec의 BLS12-381 절에서 가져왔다.
    /// Sapling의 모든 산술이 이 소수 하나 위에서 일어난다. 의미는 U03에서 돌아와 본다.
    const SPEC_R: &str = "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

    #[test]
    fn sapling_scalar_field_modulus_matches_spec() {
        assert_eq!(
            bls12_381::Scalar::MODULUS.trim_start_matches("0x"),
            SPEC_R.trim_start_matches("0x")
        );

        // todo!("U00: bls12_381::Scalar 의 MODULUS 가 SPEC_R 과 같은지 assert_eq! 로 확인")
    }
}
