//! JWT parity tests —— 对齐 Hutool `hutool-jwt` 测试。
//!
//! 对齐: `cn.hutool.jwt.JWTTest`
//! 对齐: `cn.hutool.jwt.JWTUtilTest`
//! 对齐: `cn.hutool.jwt.JWTSignerTest`
//! 对齐: `cn.hutool.jwt.JWTValidatorTest`
//! 对齐: `cn.hutool.jwt.Issue3205Test`
//! 对齐: `cn.hutool.jwt.Issue3732Test`
//! 对齐: `cn.hutool.jwt.Issue4105Test`
//! 对齐: `cn.hutool.jwt.IssueI5QRUOTest`
//! 对齐: `cn.hutool.jwt.IssueI6IS5BTest`
//!
//! 来源: hutool-jwt/src/test/java/cn/hutool/jwt/*.java

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use hj::{
    AlgorithmUtil, JWT, JWTException, JWTHeader, JWTSigner, JWTSignerUtil, JWTUtil, JWTValidator,
    NoneJWTSigner, RegisteredPayload,
};
use hutool_jwt as hj;
use serde::Deserialize;
use serde_json::{Map, Value};

const RSA_PRIVATE: &[u8] = include_bytes!("fixtures/rsa-private.pem");
const RSA_PUBLIC: &[u8] = include_bytes!("fixtures/rsa-public.pem");
const EC_PRIVATE: &[u8] = include_bytes!("fixtures/ec-private.pem");
const EC_PUBLIC: &[u8] = include_bytes!("fixtures/ec-public.pem");
const EC384_PRIVATE: &[u8] = include_bytes!("fixtures/ec384-private.pem");
const EC384_PUBLIC: &[u8] = include_bytes!("fixtures/ec384-public.pem");

/// Builds an ordered JSON object map (insertion order preserved).
fn obj(pairs: &[(&str, Value)]) -> Map<String, Value> {
    let mut map = Map::new();
    for (key, value) in pairs {
        map.insert((*key).to_owned(), value.clone());
    }
    map
}

/// Current UTC epoch seconds.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Sign-and-verify helper mirroring Hutool `JWTSignerTest.signAndVerify`.
fn sign_and_verify(signer: Arc<dyn JWTSigner>) {
    let mut jwt = JWT::create();
    jwt.set_payload("sub", Value::String("1234567890".into()))
        .set_payload("name", Value::String("looly".into()))
        .set_payload("admin", Value::Bool(true))
        .set_expires_at(now_secs() + 86_400)
        .set_signer(Arc::clone(&signer));
    let token = jwt.sign().expect("sign");
    let parsed = JWT::of(&token).expect("parse");
    assert!(
        parsed.verify_with(signer.as_ref()).expect("verify"),
        "signAndVerify should succeed"
    );
}

/// Asserts a legacy/non-JOSE algorithm is intentionally rejected.
fn assert_legacy_rejected(result: Result<Arc<dyn JWTSigner>, JWTException>, id: &str) {
    let err = match result {
        Ok(_) => panic!("{id} should be rejected"),
        Err(err) => err,
    };
    assert!(
        err.to_string().contains("intentionally unavailable")
            || err.to_string().contains("unsupported"),
        "{id} error should mention unavailability: {err}"
    );
}

// ---------------------------------------------------------------------------
// Existing smoke tests (保留，勿删)
// ---------------------------------------------------------------------------

#[test]
fn hmac_signer_test() {
    let signer = hj::HMacJWTSigner::new(hj::Algorithm::HS256, b"mysecret123");
    assert!(signer.is_ok(), "HMacJWTSigner 创建应成功");
}

#[test]
fn claims_parse_test() {
    let claims = hj::Claims::parse(r#"{"sub":"alice","iss":"hutool"}"#).unwrap();
    assert_eq!(
        claims
            .get_claim("sub")
            .map(|v| v.as_str().unwrap_or_default()),
        Some("alice")
    );
}

#[test]
fn claims_set_test() {
    let mut claims = hj::Claims::parse("{}").unwrap();
    claims.set_claim("sub", serde_json::json!("bob"));
    assert_eq!(
        claims
            .get_claim("sub")
            .map(|v| v.as_str().unwrap_or_default()),
        Some("bob")
    );
}

// ---------------------------------------------------------------------------
// JWTTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `JWTTest.createHs256Test()`
#[test]
fn create_hs256_test() {
    let key = b"1234567890";
    let mut jwt = JWT::create();
    jwt.set_payload("sub", Value::String("1234567890".into()))
        .set_payload("name", Value::String("looly".into()))
        .set_payload("admin", Value::Bool(true))
        .set_expires_at(1_640_966_400);
    jwt.set_key(key).expect("set_key");
    let right_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
        eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6Imxvb2x5IiwiYWRtaW4iOnRydWUsImV4cCI6MTY0MDk2NjQwMH0.\
        8siIwEMHf-DRyUjVElS_yipb6Mo3c1z0wFiheGXWGQw";
    let token = jwt.sign().expect("sign");
    assert_eq!(token, right_token);
    let mut verified = JWT::of(right_token).expect("of");
    verified.set_key(key).expect("set_key");
    assert!(verified.verify().expect("verify"));
}

/// 对齐 Java: `JWTTest.parseTest()`
#[test]
fn jwt_parse_test() {
    let right_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.\
        eyJzdWIiOiIxMjM0NTY3ODkwIiwiYWRtaW4iOnRydWUsIm5hbWUiOiJsb29seSJ9.\
        U2aQkC2THYV9L0fTN-yBBI7gmo5xhmvMhATtu8v0zEA";
    let mut jwt = JWT::of(right_token).expect("of");
    jwt.set_key(b"1234567890").expect("set_key");
    assert!(jwt.verify().expect("verify"));
    assert_eq!(
        jwt.header().claims().get_claim(JWTHeader::TYPE),
        Some(&Value::String("JWT".into()))
    );
    assert_eq!(
        jwt.header().claims().get_claim(JWTHeader::ALGORITHM),
        Some(&Value::String("HS256".into()))
    );
    assert!(
        jwt.header()
            .claims()
            .get_claim(JWTHeader::CONTENT_TYPE)
            .is_none()
    );
    assert_eq!(
        jwt.payload().claims().get_claim("sub"),
        Some(&Value::String("1234567890".into()))
    );
    assert_eq!(
        jwt.payload().claims().get_claim("name"),
        Some(&Value::String("looly".into()))
    );
    assert_eq!(
        jwt.payload().claims().get_claim("admin"),
        Some(&Value::Bool(true))
    );
}

/// 对齐 Java: `JWTTest.createNoneTest()`
#[test]
fn create_none_test() {
    let mut jwt = JWT::create();
    jwt.set_payload("sub", Value::String("1234567890".into()))
        .set_payload("name", Value::String("looly".into()))
        .set_payload("admin", Value::Bool(true))
        .set_signer(Arc::new(JWTSignerUtil::none()));
    let right_token = "eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.\
        eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6Imxvb2x5IiwiYWRtaW4iOnRydWV9.";
    let token = jwt.sign().expect("sign");
    assert_eq!(token, right_token);
    let parsed = JWT::of(right_token).expect("of");
    assert!(
        parsed
            .verify_with(&JWTSignerUtil::none())
            .expect("verify none")
    );
}

/// 对齐 Java: `JWTTest.needSignerTest()`
#[test]
fn need_signer_test() {
    let mut jwt = JWT::create();
    jwt.set_payload("sub", Value::String("1234567890".into()))
        .set_payload("name", Value::String("looly".into()))
        .set_payload("admin", Value::Bool(true));
    let err = jwt.sign().expect_err("sign without signer must fail");
    assert!(
        err.to_string().contains("no signer"),
        "expected missing signer error, got {err}"
    );
}

/// 对齐 Java: `JWTTest.verifyTest()`
#[test]
fn jwt_verify_test() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
        eyJ1c2VyX25hbWUiOiJhZG1pbiIsInNjb3BlIjpbImFsbCJdLCJleHAiOjE2MjQwMDQ4MjIsInVzZXJJZCI6MSwiYXV0aG9yaXRpZXMiOlsiUk9MRV_op5LoibLkuozlj7ciLCJzeXNfbWVudV8xIiwiUk9MRV_op5LoibLkuIDlj7ciLCJzeXNfbWVudV8yIl0sImp0aSI6ImQ0YzVlYjgwLTA5ZTctNGU0ZC1hZTg3LTVkNGI5M2FhNmFiNiIsImNsaWVudF9pZCI6ImhhbmR5LXNob3AifQ.\
        aixF1eKlAKS_k3ynFnStE7-IRGiD5YaqznvK2xEjBew";
    let mut jwt = JWT::of(token).expect("of");
    jwt.set_key(b"123456").expect("set_key");
    assert!(jwt.verify().expect("verify"));
}

/// 对齐 Java: `JWTTest.getLongTest()`
#[test]
fn get_long_test() {
    let right_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9\
        .eyJsb2dpblR5cGUiOiJsb2dpbiIsImxvZ2luSWQiOiJhZG1pbiIsImRldmljZSI6ImRlZmF1bHQtZGV2aWNlIiwiZWZmIjoxNjc4Mjg1NzEzOTM1LCJyblN0ciI6IkVuMTczWFhvWUNaaVZUWFNGOTNsN1pabGtOalNTd0pmIn0\
        .wRe2soTaWYPhwcjxdzesDi1BgEm9D61K-mMT3fPc4YM";
    let jwt = JWTUtil::parse_token(right_token).expect("parse");
    let payloads = jwt.payload().claims();
    assert_eq!(
        payloads.to_string(),
        "{\"loginType\":\"login\",\"loginId\":\"admin\",\"device\":\"default-device\",\
         \"eff\":1678285713935,\"rnStr\":\"En173XXoYCZiVTXSF93l7ZZlkNjSSwJf\"}"
    );
    assert_eq!(
        payloads.get_claim("eff").and_then(Value::as_u64),
        Some(1_678_285_713_935)
    );
}

// ---------------------------------------------------------------------------
// JWTUtilTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `JWTUtilTest.createTest()`
#[test]
fn jwt_util_create_test() {
    let key = b"1234";
    let payload = obj(&[
        ("uid", Value::from(123)),
        (
            "expire_time",
            Value::from(now_secs() * 1000 + 1_000 * 60 * 60 * 24 * 15),
        ),
    ]);
    let token = JWTUtil::create_token(payload, key).expect("create_token");
    assert_eq!(token.matches('.').count(), 2);
    assert!(JWTUtil::verify(&token, key).expect("verify"));
}

/// 对齐 Java: `JWTUtilTest.parseTest()`
#[test]
fn jwt_util_parse_test() {
    let right_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.\
        eyJzdWIiOiIxMjM0NTY3ODkwIiwiYWRtaW4iOnRydWUsIm5hbWUiOiJsb29seSJ9.\
        U2aQkC2THYV9L0fTN-yBBI7gmo5xhmvMhATtu8v0zEA";
    let mut jwt = JWTUtil::parse_token(right_token).expect("parse");
    jwt.set_key(b"1234567890").expect("set_key");
    assert!(jwt.verify().expect("verify"));
    assert_eq!(
        jwt.header().claims().get_claim(JWTHeader::TYPE),
        Some(&Value::String("JWT".into()))
    );
    assert_eq!(
        jwt.header().claims().get_claim(JWTHeader::ALGORITHM),
        Some(&Value::String("HS256".into()))
    );
    assert!(
        jwt.header()
            .claims()
            .get_claim(JWTHeader::CONTENT_TYPE)
            .is_none()
    );
    assert_eq!(
        jwt.payload().claims().get_claim("sub"),
        Some(&Value::String("1234567890".into()))
    );
    assert_eq!(
        jwt.payload().claims().get_claim("name"),
        Some(&Value::String("looly".into()))
    );
    assert_eq!(
        jwt.payload().claims().get_claim("admin"),
        Some(&Value::Bool(true))
    );
}

/// 对齐 Java: `JWTUtilTest.parseNullTest()`
#[test]
fn parse_null_test() {
    // Java passes null; Rust has no null `&str` — empty / invalid input is the boundary.
    assert!(JWTUtil::parse_token("").is_err());
}

/// 对齐 Java: `JWTUtilTest.verifyTest()`
#[test]
fn jwt_util_verify_test() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
        eyJ1c2VyX25hbWUiOiJhZG1pbiIsInNjb3BlIjpbImFsbCJdLCJleHAiOjE2MjQwMDQ4MjIsInVzZXJJZCI6MSwiYXV0aG9yaXRpZXMiOlsiUk9MRV_op5LoibLkuozlj7ciLCJzeXNfbWVudV8xIiwiUk9MRV_op5LoibLkuIDlj7ciLCJzeXNfbWVudV8yIl0sImp0aSI6ImQ0YzVlYjgwLTA5ZTctNGU0ZC1hZTg3LTVkNGI5M2FhNmFiNiIsImNsaWVudF9pZCI6ImhhbmR5LXNob3AifQ.\
        aixF1eKlAKS_k3ynFnStE7-IRGiD5YaqznvK2xEjBew";
    assert!(JWTUtil::verify(token, b"123456").expect("verify"));
}

// ---------------------------------------------------------------------------
// JWTSignerTest — JOSE algorithms
// ---------------------------------------------------------------------------

/// 对齐 Java: `JWTSignerTest.hs256Test()`
#[test]
fn hs256_test() {
    let id = "hs256";
    let key = b"hutool-hs256-test-key-32bytes!!";
    let signer = JWTSignerUtil::create_signer(id, key).expect("create_signer");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(Arc::new(signer));
}

/// 对齐 Java: `JWTSignerTest.hs256Test2()`
#[test]
fn hs256_test2() {
    let signer = JWTSignerUtil::hs256(b"123456").expect("hs256");
    sign_and_verify(Arc::new(signer));
}

/// 对齐 Java: `JWTSignerTest.hs384Test()`
#[test]
fn hs384_test() {
    let id = "hs384";
    let key = b"hutool-hs384-test-key-material!!";
    let signer = JWTSignerUtil::create_signer(id, key).expect("create_signer");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(Arc::new(signer));
}

/// 对齐 Java: `JWTSignerTest.hs512Test()`
#[test]
fn hs512_test() {
    let id = "hs512";
    let key = b"hutool-hs512-test-key-material-long-enough!!";
    let signer = JWTSignerUtil::create_signer(id, key).expect("create_signer");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(Arc::new(signer));
}

/// 对齐 Java: `JWTSignerTest.rs256Test()`
#[test]
fn rs256_test() {
    let id = "rs256";
    let signer = JWTSignerUtil::create_signer_from_pem(id, RSA_PRIVATE, RSA_PUBLIC).expect("rs256");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(signer);
}

/// 对齐 Java: `JWTSignerTest.rs384Test()`
#[test]
fn rs384_test() {
    let id = "rs384";
    let signer = JWTSignerUtil::create_signer_from_pem(id, RSA_PRIVATE, RSA_PUBLIC).expect("rs384");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(signer);
}

/// 对齐 Java: `JWTSignerTest.rs512Test()`
#[test]
fn rs512_test() {
    let id = "rs512";
    let signer = JWTSignerUtil::create_signer_from_pem(id, RSA_PRIVATE, RSA_PUBLIC).expect("rs512");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(signer);
}

/// 对齐 Java: `JWTSignerTest.es256Test()`
#[test]
fn jwt_signer_es256_test() {
    let id = "es256";
    let signer = JWTSignerUtil::create_signer_from_pem(id, EC_PRIVATE, EC_PUBLIC).expect("es256");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(signer);
}

/// 对齐 Java: `JWTSignerTest.es384Test()`
#[test]
fn es384_test() {
    let id = "es384";
    let signer =
        JWTSignerUtil::create_signer_from_pem(id, EC384_PRIVATE, EC384_PUBLIC).expect("es384");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(signer);
}

/// 对齐 Java: `JWTSignerTest.es512Test()`
#[test]
fn es512_test() {
    // API decision: ES512 intentionally unavailable (RustCrypto JOSE engine gap).
    assert_legacy_rejected(JWTSignerUtil::es512(EC_PRIVATE, EC_PUBLIC), "ES512");
}

/// 对齐 Java: `JWTSignerTest.ps256Test()`
#[test]
fn ps256_test() {
    let id = "ps256";
    let signer = JWTSignerUtil::create_signer_from_pem(id, RSA_PRIVATE, RSA_PUBLIC).expect("ps256");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(signer);
}

/// 对齐 Java: `JWTSignerTest.ps384Test()`
#[test]
fn ps384_test() {
    let id = "ps384";
    let signer = JWTSignerUtil::create_signer_from_pem(id, RSA_PRIVATE, RSA_PUBLIC).expect("ps384");
    assert_eq!(
        AlgorithmUtil::get_id(AlgorithmUtil::get_algorithm(id).unwrap()),
        signer.algorithm_id()
    );
    sign_and_verify(signer);
}

/// 对齐 Java: `JWTSignerTest.hmd5Test()`
#[test]
fn hmd5_test() {
    assert_legacy_rejected(JWTSignerUtil::hmd5(b"key"), "HMD5");
}

/// 对齐 Java: `JWTSignerTest.hsha1Test()`
#[test]
fn hsha1_test() {
    assert_legacy_rejected(JWTSignerUtil::hsha1(b"key"), "HSHA1");
}

/// 对齐 Java: `JWTSignerTest.sm4cmacTest()`
#[test]
fn sm4cmac_test() {
    assert_legacy_rejected(JWTSignerUtil::sm4cmac(b"0123456789abcdef"), "SM4CMAC");
}

/// 对齐 Java: `JWTSignerTest.rmd2Test()`
#[test]
fn rmd2_test() {
    assert_legacy_rejected(JWTSignerUtil::rmd2(RSA_PRIVATE), "RMD2");
}

/// 对齐 Java: `JWTSignerTest.rmd5Test()`
#[test]
fn rmd5_test() {
    assert_legacy_rejected(JWTSignerUtil::rmd5(RSA_PRIVATE), "RMD5");
}

/// 对齐 Java: `JWTSignerTest.rsha1Test()`
#[test]
fn rsha1_test() {
    assert_legacy_rejected(JWTSignerUtil::rsha1(RSA_PRIVATE), "RSHA1");
}

/// 对齐 Java: `JWTSignerTest.dnoneTest()`
#[test]
fn dnone_test() {
    assert_legacy_rejected(JWTSignerUtil::dnone(RSA_PRIVATE), "DNONE");
}

/// 对齐 Java: `JWTSignerTest.dsha1Test()`
#[test]
fn dsha1_test() {
    assert_legacy_rejected(JWTSignerUtil::dsha1(RSA_PRIVATE), "DSHA1");
}

/// 对齐 Java: `JWTSignerTest.enoneTest()`
#[test]
fn enone_test() {
    assert_legacy_rejected(JWTSignerUtil::enone(EC_PRIVATE), "ENONE");
}

/// 对齐 Java: `JWTSignerTest.esha1Test()`
#[test]
fn esha1_test() {
    assert_legacy_rejected(JWTSignerUtil::esha1(EC_PRIVATE), "ESHA1");
}

// ---------------------------------------------------------------------------
// JWTValidatorTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `JWTValidatorTest.expiredAtTest()`
#[test]
fn expired_at_test() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE0Nzc1OTJ9.\
        isvT0Pqx0yjnZk53mUFSeYFJLDs-Ls9IsNAm86gIdZo";
    let validator = JWTValidator::of_token(token).expect("of_token");
    assert!(validator.validate_date().is_err());
}

/// 对齐 Java: `JWTValidatorTest.issueAtTest()`
#[test]
fn issue_at_test() {
    let mut jwt = JWT::create();
    let issued = now_secs();
    jwt.set_issued_at(issued);
    jwt.set_key(b"123456").expect("set_key");
    let token = jwt.sign().expect("sign");
    // 签发时间晚于被检查的时间（昨天）→ 失败
    let yesterday = issued.saturating_sub(86_400);
    assert!(
        JWTValidator::of_token(&token)
            .expect("of")
            .validate_date_at(yesterday, 0)
            .is_err()
    );
}

/// 对齐 Java: `JWTValidatorTest.issueAtPassTest()`
#[test]
fn issue_at_pass_test() {
    let mut jwt = JWT::create();
    let issued = now_secs();
    jwt.set_issued_at(issued);
    jwt.set_key(b"123456").expect("set_key");
    let token = jwt.sign().expect("sign");
    JWTValidator::of_token(&token)
        .expect("of")
        .validate_date_at(issued, 0)
        .expect("iat should pass at now");
}

/// 对齐 Java: `JWTValidatorTest.notBeforeTest()`
#[test]
fn not_before_test() {
    let mut jwt = JWT::create();
    let nbf = now_secs();
    jwt.set_not_before(nbf);
    let yesterday = nbf.saturating_sub(86_400);
    assert!(
        JWTValidator::of_jwt(jwt)
            .validate_date_at(yesterday, 0)
            .is_err()
    );
}

/// 对齐 Java: `JWTValidatorTest.notBeforePassTest()`
#[test]
fn not_before_pass_test() {
    let mut jwt = JWT::create();
    let nbf = now_secs();
    jwt.set_not_before(nbf);
    JWTValidator::of_jwt(jwt)
        .validate_date_at(nbf, 0)
        .expect("nbf should pass at now");
}

/// 对齐 Java: `JWTValidatorTest.validateAlgorithmTest()`
#[test]
fn validate_algorithm_test() {
    let mut jwt = JWT::create();
    jwt.set_not_before(now_secs());
    jwt.set_key(b"123456").expect("set_key");
    let token = jwt.sign().expect("sign");
    let signer = JWTSignerUtil::hs256(b"123456").expect("hs256");
    JWTValidator::of_token(&token)
        .expect("of")
        .validate_algorithm_with(&signer)
        .expect("validateAlgorithm");
}

/// 对齐 Java: `JWTValidatorTest.validateTest()`
#[test]
fn validate_test() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.\
        eyJpc3MiOiJNb0xpIiwiZXhwIjoxNjI0OTU4MDk0NTI4LCJpYXQiOjE2MjQ5NTgwMzQ1MjAsInVzZXIiOiJ1c2VyIn0.\
        L0uB38p9sZrivbmP0VlDe--j_11YUXTu3TfHhfQhRKc";
    let key = b"1234567890";
    let mut jwt = JWT::of(token).expect("of");
    jwt.set_key(key).expect("set_key");
    // Hutool validate(0) → signature OK but dates expired → false
    assert!(!jwt.is_valid_at(now_secs(), 0).expect("is_valid_at"));
}

/// 对齐 Java: `JWTValidatorTest.validateDateTest()`
#[test]
fn validate_date_test() {
    let mut jwt = JWT::create();
    jwt.set_payload("id", Value::from(123))
        .set_payload("username", Value::String("hutool".into()))
        // 2021-10-13 09:59:00 UTC ≈ 1634119140
        .set_expires_at(1_634_119_140);
    assert!(
        JWTValidator::of_jwt(jwt).validate_date().is_err(),
        "expired token must fail date validation"
    );
}

/// 对齐 Java: `JWTValidatorTest.issue2329Test()`
#[test]
fn issue2329_test() {
    let now = now_secs();
    let expired = 3_u64;
    let mut builder = JWT::create();
    builder
        .set_payload("sub", Value::String("blue-light".into()))
        .set_issued_at(now)
        .set_not_before(now + expired)
        .set_expires_at(now + expired);
    builder.set_key(b"123456").expect("set_key");
    let token = builder.sign().expect("sign");
    // leeway=10 covers ±4s skew around nbf/exp/iat windows
    JWTValidator::of_token(&token)
        .expect("of")
        .validate_date_at(now.saturating_sub(4), 10)
        .expect("validate early with leeway");
    JWTValidator::of_token(&token)
        .expect("of")
        .validate_date_at(now + 4, 10)
        .expect("validate late with leeway");
}

// ---------------------------------------------------------------------------
// Issue3205Test
// ---------------------------------------------------------------------------

/// 对齐 Java: `Issue3205Test.es256Test()`
#[test]
fn issue3205_es256_test() {
    let id = "es256";
    let signer = JWTSignerUtil::create_signer_from_pem(id, EC_PRIVATE, EC_PUBLIC).expect("es256");
    let mut jwt = JWT::create();
    jwt.set_payload("sub", Value::String("1234567890".into()))
        .set_payload("name", Value::String("looly".into()))
        .set_payload("admin", Value::Bool(true))
        .set_expires_at(now_secs() + 86_400)
        .set_signer(Arc::clone(&signer));
    let token = jwt.sign().expect("sign");
    // Java uses jjwt verifyWith(public); we verify with the same ES256 signer.
    assert!(
        JWT::of(&token)
            .expect("of")
            .verify_with(signer.as_ref())
            .expect("verify")
    );
}

// ---------------------------------------------------------------------------
// Issue3732Test
// ---------------------------------------------------------------------------

/// 对齐 Java: `Issue3732Test.hmacTest()`
#[test]
fn hmac_test() {
    let signer: Arc<dyn JWTSigner> =
        Arc::new(JWTSignerUtil::hs256(b"6sf2f5j2a62a3s8f9032hsf").expect("hs256"));
    // HashMap iteration order in Java produced role-then-name for this fixture.
    let payload = obj(&[
        ("role", Value::String("admin".into())),
        ("name", Value::String("test".into())),
    ]);
    let token = JWTUtil::create_token_with_signer(payload, signer).expect("create");
    assert_eq!(
        token,
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYWRtaW4iLCJuYW1lIjoidGVzdCJ9.\
         eS1hjkb2ympf7Gtnh_Xmzmb29bXt3J-1SyNTLMBipbY"
    );
}

// ---------------------------------------------------------------------------
// Issue4105Test
// ---------------------------------------------------------------------------

/// Builds a compact JWT from Hutool-style standard Base64 header/payload (Issue4105).
fn issue4105_token(header_json: &str, payload_json: &str) -> String {
    // Hutool `Base64.encode` uses Java standard encoder (may include padding).
    let head = STANDARD.encode(header_json.as_bytes());
    let payload = STANDARD.encode(payload_json.as_bytes());
    // Prefer URL-safe no-pad for our parser if standard padded form fails.
    let token = format!("{head}.{payload}.");
    if JWTUtil::parse_token(&token).is_ok() {
        return token;
    }
    let head = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
    let payload = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());
    format!("{head}.{payload}.")
}

/// 对齐 Java: `Issue4105Test.verifyNoneTest()`
#[test]
fn verify_none_test() {
    let token = issue4105_token(r#"{"alg": "none"}"#, r#"{"exp": 1642196407}"#);
    let jwt = JWTUtil::parse_token(&token).expect("parse");
    assert!(jwt.signer().is_none());
    assert!(NoneJWTSigner::is_none(jwt.algorithm()));
    // Hutool: none alg → verify() true without signer; Rust uses explicit none signer.
    assert!(
        jwt.verify_with(&JWTSignerUtil::none())
            .expect("verify none")
    );
    let mut jwt2 = JWTUtil::parse_token(&token).expect("parse");
    assert!(
        jwt2.set_key(b"123").and_then(|jwt| jwt.verify()).is_err(),
        "key with alg=none must error"
    );
}

/// 对齐 Java: `Issue4105Test.verifyEmptyTest()`
#[test]
fn verify_empty_test() {
    let token = issue4105_token(r#"{"alg": ""}"#, r#"{"exp": 1642196407}"#);
    let jwt = JWTUtil::parse_token(&token).expect("parse");
    assert!(jwt.signer().is_none());
    assert!(NoneJWTSigner::is_none(jwt.algorithm()));
    // Empty alg is treated as none; verify empty signature directly (header alg "" ≠ "none").
    let parts: Vec<_> = token.split('.').collect();
    assert!(
        JWTSignerUtil::none()
            .verify(parts[0], parts[1], parts[2])
            .expect("empty sig")
    );
    let mut jwt2 = JWTUtil::parse_token(&token).expect("parse");
    assert!(
        jwt2.set_key(b"123").and_then(|jwt| jwt.verify()).is_err(),
        "key with empty alg must error"
    );
}

/// 对齐 Java: `Issue4105Test.verifyHs256Test()`
#[test]
fn verify_hs256_test() {
    let token = issue4105_token(r#"{"alg": "HS256"}"#, r#"{"exp": 1642196407}"#);
    let jwt = JWTUtil::parse_token(&token).expect("parse");
    assert!(jwt.signer().is_none());
    assert!(jwt.verify().is_err(), "HS256 without signer/key must error");
    let mut jwt2 = JWTUtil::parse_token(&token).expect("parse");
    assert!(
        !jwt2
            .set_key(b"123")
            .and_then(|jwt| jwt.verify())
            .expect("verify with key"),
        "empty signature must fail HS256 verify"
    );
}

// ---------------------------------------------------------------------------
// IssueI5QRUOTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `IssueI5QRUOTest.createTokenTest()`
#[test]
fn create_token_test() {
    let headers = obj(&[
        (JWTHeader::ALGORITHM, Value::String("HS384".into())),
        (JWTHeader::TYPE, Value::String("JWT".into())),
    ]);
    let payload = obj(&[
        ("sub", Value::String("1234567890".into())),
        ("name", Value::String("John Doe".into())),
        ("iat", Value::from(1_516_239_022_u64)),
    ]);
    let token = JWTUtil::create_token_with_headers(headers, payload, b"123456").expect("create");
    assert_eq!(
        token,
        "eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.\
         eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.\
         3Ywq9NlR3cBST4nfcdbR-fcZ8374RHzU50X6flKvG-tnWFMalMaHRm3cMpXs1NrZ"
    );
    let mut jwt = JWT::of(&token).expect("of");
    jwt.set_key(b"123456").expect("set_key");
    assert!(jwt.verify().expect("verify"));
}

// ---------------------------------------------------------------------------
// IssueI6IS5BTest
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
struct JwtTokenIat {
    iat: i64,
}

/// 对齐 Java: `IssueI6IS5BTest.payloadToBeanTest()`
#[test]
fn payload_to_bean_test() {
    // 2023-03-03 00:00:00 UTC
    let iat = 1_677_772_800_i64;
    let payload = obj(&[("iat", Value::from(iat as u64))]);
    let token = JWTUtil::create_token(payload, b"123").expect("create");
    assert_eq!(
        token,
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE2Nzc3NzI4MDB9.\
         W88PB2ovAqCXV4QdbeKbdFW-P057xOTXEosD8hbOa9U"
    );
    let payloads = JWTUtil::parse_token(&token)
        .expect("parse")
        .payload()
        .claims()
        .clone();
    assert_eq!(payloads.to_string(), "{\"iat\":1677772800}");
    let bean: JwtTokenIat =
        serde_json::from_value(Value::Object(payloads.claims_json().clone())).expect("toBean");
    assert_eq!(bean.iat, iat);
}

/// 对齐 Java: `IssueI6IS5BTest.payloadToBeanTest2()`
#[test]
fn payload_to_bean_test2() {
    let iat = 1_677_772_800_i64;
    let payload = obj(&[("iat", Value::from(iat as u64))]);
    let token = JWTUtil::create_token(payload, b"123").expect("create");
    assert_eq!(
        token,
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE2Nzc3NzI4MDB9.\
         W88PB2ovAqCXV4QdbeKbdFW-P057xOTXEosD8hbOa9U"
    );
    let payloads = JWTUtil::parse_token(&token)
        .expect("parse")
        .payload()
        .claims()
        .clone();
    assert_eq!(payloads.to_string(), "{\"iat\":1677772800}");
    let bean: JwtTokenIat =
        serde_json::from_value(Value::Object(payloads.claims_json().clone())).expect("toBean");
    assert_eq!(bean.iat, iat);
}
