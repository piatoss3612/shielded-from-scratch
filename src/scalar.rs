//! U03 — 진짜 필드로: `ff::PrimeField`를 통해 `bls12_381::Scalar`를 다룬다.
//!
//! p = U00에서 못 박은 r (255비트). u64에 안 들어가므로 toy `Fp`를 버리고 crate의 타입을 쓴다.
//! 대신 crate가 노출하는 인터페이스(`ff::PrimeField` 트레이트)를 읽고, 그 트레이트만 아는
//! 제네릭 함수를 쓴다. U02에서 `Add`를 "구현"했다면 여기서는 남이 정의한 트레이트를 "요구"한다.
//!
//! `jubjub::Fq`는 `bls12_381::Scalar`와 같은 타입이다. Jubjub 곡선의 좌표가 이 필드의 원소이고,
//! 그래서 U04의 곡선 산술이 전부 이 필드 위에서 일어난다.

use ff::PrimeField;

// ───────────────────────── 세션 1: Repr ─────────────────────────

/// 필드 원소를 리틀엔디안 바이트로. `F::Repr`은 `AsRef<[u8]>`이라 `.as_ref()`로 슬라이스를 얻는다.
pub fn to_le_bytes<F: PrimeField>(x: &F) -> Vec<u8> {
    x.to_repr().as_ref().to_vec()
}

/// 리틀엔디안 바이트에서 필드 원소로. 길이가 다르거나 값이 p 이상(non-canonical)이면 `None`.
/// `F::Repr: Default + AsMut<[u8]>`이니 기본값을 만들고 그 안에 복사해 넣은 뒤 `from_repr`.
/// `from_repr`은 `CtOption`을 돌려준다. `Option::from(ct)` 또는 `ct.into()`로 바꿀 수 있다.
pub fn from_le_bytes<F: PrimeField>(bytes: &[u8]) -> Option<F> {
    let mut repr = F::Repr::default();
    if bytes.len() != repr.as_ref().len() {
        return None;
    }
    repr.as_mut().copy_from_slice(bytes);
    F::from_repr(repr).into()
}

// ───────────────────────── 세션 2: Tonelli-Shanks, 제네릭 ─────────────────────────

/// `bls12_381::Scalar`의 (t - 1) / 2. r - 1 = 2^32 · t. u64 limb 리틀엔디안.
/// Python으로 계산해 crate 소스(`bls12_381/src/scalar.rs` `fn sqrt`)의 상수와 대조했다.
pub const BLS12_381_SCALAR_T_MINUS1_OVER2: [u64; 4] = [
    0x7fff_2dff_7fff_ffff,
    0x04d0_ec02_a9de_d201,
    0x94ce_bea4_199c_ec04,
    0x0000_0000_39f6_d3a9,
];

/// U03 세션 1의 toy `sqrt`를 `ff::PrimeField` 위에서 제네릭으로 다시 쓴다.
/// 시그니처는 `ff::helpers::sqrt_tonelli_shanks`와 같은 모양이다.
///
/// 재료: `F::S`(s), `F::ROOT_OF_UNITY`(= 생성원^t, toy의 c = z^t 에 해당), `x.pow_vartime(&[u64])`,
/// `x.square()`, `F::ONE`, `x.is_zero_vartime()`. t 자체는 없고 (t-1)/2 만 인자로 받는다.
/// 함정(U00과 같은 것): `square`·`pow_vartime`·`ONE`·`is_zero_vartime`은 `PrimeField`가 아니라 그 상위 트레이트
/// `ff::Field`의 항목이다. 상위 트레이트의 항목은 자동으로 스코프에 들어오지 않는다.
/// 참고: 구체 타입 `bls12_381::Scalar`에는 고유 메서드 `pow_vartime(&self, &[u64; 4])`가 따로 있어 트레이트 메서드를
/// 가리지만, 이 함수처럼 `F: PrimeField` 제네릭 안에서는 트레이트의 `pow_vartime<S: AsRef<[u64]>>`가 쓰인다.
/// a^((t+1)/2) = a^((t-1)/2) · a, a^t = a^((t-1)/2)² · a 로 만들 수 있다.
pub fn sqrt_tonelli_shanks<F: PrimeField>(a: F, t_minus_1_over_2: &[u64]) -> Option<F> {
    // 1. a가 0이면 0을 돌려준다.
    if a.is_zero_vartime() {
        return Some(F::ZERO);
    }

    let w = a.pow_vartime(t_minus_1_over_2); // w = a^((t-1)/2)

    // 2. r, u, c, m을 초기화한다.
    let mut r = w * a; // r = a^((t-1)/2) · a = a^((t+1)/2)
    let mut u = r * w; // u = a^((t-1)/2)² · a = a^t
    let mut c = F::ROOT_OF_UNITY; // c = z^t
    let mut m = F::S; // m = s

    // 2. u가 1이 될 때까지 반복
    while u != F::ONE {
        // 2.1. u^(2^i) = 1이 되는 최소 i를 찾는다.
        let mut i = 1;
        let mut u2i = u.square();
        while u2i != F::ONE {
            u2i = u2i.square();
            i += 1;
        }

        if i >= m {
            // u^(2^m) = 1이므로 u는 제곱근이 아니다.
            return None;
        }

        // 2.2. b = c^(2^(m-i-1))를 계산한다.
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = b.square();
        }

        // 2.3. r, u, c, m을 갱신한다.
        r *= b; // r = r * b
        u *= b * b; // u = u * b²
        c = b.square(); // c = b²
        m = i; // m = i
    }

    // 3. r이 제곱근이다.
    Some(r)
}

/// 세션 1 게이트의 나머지 절반: `cargo test repr`
#[cfg(test)]
mod repr {
    use super::*;
    use bls12_381::Scalar;
    use ff::Field; // ZERO·ONE·square·pow_vartime 은 상위 트레이트 Field 의 것

    #[test]
    fn repr_is_little_endian_32_bytes() {
        let one = to_le_bytes(&Scalar::from(1u64));
        assert_eq!(one.len(), 32);
        assert_eq!(one[0], 1);
        assert!(one[1..].iter().all(|&b| b == 0));

        let x = to_le_bytes(&Scalar::from(0x0102u64));
        assert_eq!((x[0], x[1]), (2, 1));
    }

    #[test]
    fn roundtrip() {
        let samples = [
            Scalar::ZERO,
            Scalar::ONE,
            Scalar::from(u64::MAX),
            -Scalar::ONE, // r - 1, 가장 큰 canonical 값
            Scalar::from(7u64).pow_vartime(&[100u64, 0, 0, 0]),
        ];
        for x in samples {
            let bytes = to_le_bytes(&x);
            assert_eq!(from_le_bytes::<Scalar>(&bytes), Some(x));
        }
    }

    #[test]
    fn non_canonical_and_wrong_length_rejected() {
        // r - 1 의 바이트 첫 칸에 1 을 더하면 r 그 자체. r 은 필드 원소가 아니다.
        let mut r = to_le_bytes(&(-Scalar::ONE));
        assert_eq!(r[0], 0);
        r[0] = 1;
        assert_eq!(from_le_bytes::<Scalar>(&r), None);

        assert_eq!(from_le_bytes::<Scalar>(&[1u8; 31]), None);
        assert_eq!(from_le_bytes::<Scalar>(&[0u8; 33]), None);
    }
}

/// 세션 2 게이트: `cargo test` 전체
#[cfg(test)]
mod tonelli {
    use super::*;
    use bls12_381::Scalar;
    use ff::Field;

    fn samples() -> Vec<Scalar> {
        let mut v = vec![
            Scalar::ZERO,
            Scalar::ONE,
            Scalar::from(2u64),
            Scalar::from(3u64),
            Scalar::from(4u64),
            Scalar::from(1u64 << 40),
            -Scalar::ONE,
            Scalar::MULTIPLICATIVE_GENERATOR,
        ];
        let mut x = Scalar::from(123_456_789u64);
        for _ in 0..8 {
            x = x.square() + Scalar::from(11u64);
            v.push(x);
        }
        v
    }

    #[test]
    fn constant_is_consistent_with_crate() {
        // g^((t-1)/2)² · g = g^t = ROOT_OF_UNITY
        let g = Scalar::MULTIPLICATIVE_GENERATOR;
        let half = g.pow_vartime(&BLS12_381_SCALAR_T_MINUS1_OVER2);
        assert_eq!(half.square() * g, Scalar::ROOT_OF_UNITY);
        assert_eq!(Scalar::S, 32);
    }

    #[test]
    fn matches_crate_sqrt_up_to_sign() {
        for a in samples() {
            let ours = sqrt_tonelli_shanks(a, &BLS12_381_SCALAR_T_MINUS1_OVER2);
            let theirs: Option<Scalar> = a.sqrt().into();
            assert_eq!(ours.is_some(), theirs.is_some(), "{a:?}");
            if let (Some(s), Some(t)) = (ours, theirs) {
                assert!(s == t || s == -t, "{a:?}");
                assert_eq!(s.square(), a);
            }
        }
    }

    #[test]
    fn squares_have_roots_and_generator_does_not() {
        for a in samples() {
            let sq = a.square();
            let s = sqrt_tonelli_shanks(sq, &BLS12_381_SCALAR_T_MINUS1_OVER2).expect("a² 는 제곱수");
            assert_eq!(s.square(), sq);
        }
        // 곱셈군의 생성원은 홀수 소수 체에서 절대 제곱수가 아니다.
        assert_eq!(
            sqrt_tonelli_shanks(Scalar::MULTIPLICATIVE_GENERATOR, &BLS12_381_SCALAR_T_MINUS1_OVER2),
            None
        );
    }

    #[test]
    fn jubjub_base_field_is_bls12_381_scalar_field() {
        // 타입이 같다: 한쪽 값을 다른 쪽 타입의 변수에 그대로 넣을 수 있다.
        let x: jubjub::Fq = Scalar::ONE;
        assert_eq!(x, Scalar::ONE);
        assert_eq!(jubjub::Fq::MODULUS, Scalar::MODULUS);
    }
}
