//! hutool-core Issue 回归测试
//! 对齐: hutool-core Issue*Test 系列

use hutool_core::{
    CollUtil, ListUtil, BooleanUtil, HexUtil, HashUtil, IdcardUtil, PhoneUtil,
};

// ── Issue: 字符串处理 ──

#[test]
fn issue_str_trim() {
    let trimmed = "  hello  ";
    assert_eq!(trimmed.trim(), "hello");
}

#[test]
fn issue_str_is_blank() {
    assert!("".is_empty());
    assert!("   ".trim().is_empty());
    assert!(!"hello".is_empty());
}

// ── Issue: 集合操作 ──

#[test]
fn issue_coll_is_empty() {
    assert!(CollUtil::is_empty(Some(&[] as &[i32])));
    assert!(!CollUtil::is_empty(Some(&[1, 2] as &[i32])));
}

#[test]
fn issue_coll_is_not_empty() {
    assert!(!CollUtil::is_not_empty(Some(&[] as &[i32])));
    assert!(CollUtil::is_not_empty(Some(&[1, 2] as &[i32])));
}

#[test]
fn issue_list_page() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let page1 = ListUtil::page(&items, 0, 3).unwrap();
    assert_eq!(page1, &[1, 2, 3]);
    let page2 = ListUtil::page(&items, 1, 3).unwrap();
    assert_eq!(page2, &[4, 5, 6]);
}

#[test]
fn issue_list_partition() {
    let items = vec![1, 2, 3, 4, 5];
    let p = ListUtil::partition(&items, 2).unwrap();
    assert_eq!(p.get(0), Some(&[1, 2][..]));
    assert_eq!(p.get(1), Some(&[3, 4][..]));
    assert_eq!(p.get(2), Some(&[5][..]));
}

// ── Issue: 布尔判断 ──

#[test]
fn issue_boolean_is_true() {
    assert!(BooleanUtil::is_true(Some(true)));
    assert!(!BooleanUtil::is_true(Some(false)));
}

// ── Issue: 字节操作 ──

#[test]
fn issue_byte_hex() {
    let bytes = vec![0xCA, 0xFE, 0xBA, 0xBE];
    let hex = HexUtil::encode_hex(&bytes);
    assert_eq!(hex.to_lowercase(), "cafebabe");
}

// ── Issue: 哈希操作 ──

#[test]
fn issue_hash_fnv() {
    let h1 = HashUtil::fnv_hash("hello");
    let h2 = HashUtil::fnv_hash("hello");
    assert_eq!(h1, h2);
    let h3 = HashUtil::fnv_hash("world");
    assert_ne!(h1, h3);
}

// ── Issue: 身份证验证 ──

#[test]
fn issue_idcard_valid() {
    assert!(IdcardUtil::is_valid_card_18("11010519491231002X"));
    assert!(!IdcardUtil::is_valid_card_18("invalid"));
}

#[test]
fn issue_idcard_gender() {
    let gender = IdcardUtil::get_gender_by_id_card("11010519491231002X").unwrap();
    assert_eq!(gender, 0);
}

// ── Issue: 手机号验证 ──

#[test]
fn issue_phone_mobile() {
    assert!(PhoneUtil::is_mobile("13800138000"));
    assert!(!PhoneUtil::is_mobile("12345678901"));
}

#[test]
fn issue_phone_hide() {
    let hidden = PhoneUtil::hide_before("13800138000");
    assert!(hidden.starts_with("***"));
}

// ── Issue: 坐标转换 ──
