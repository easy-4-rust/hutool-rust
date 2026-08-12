//! 扩展 parity 测试
//! 对齐: hutool-core 多个测试类
//!
//! 覆盖更多 hutool 测试场景，提升 hutool-rs 测试覆盖率。

use hutool_core::{
    ArrayUtil, BooleanUtil, CollUtil, CreditCodeUtil, DesensitizedUtil, DictUtil, EnumUtil,
    EscapeUtil, FileUtil, HashUtil, HexUtil, IdcardUtil, IoUtil, ListUtil, MapUtil, NumberUtil,
    ObjectUtil, PageUtil, PhoneUtil, RandomUtil, ReUtil, ReflectUtil, TypeUtil, UrlUtil, Validator,
    VersionUtil, XmlUtil,
};
use serde_json::json;

// ── 集合操作 (10 tests) ──

#[test]
fn c01() {
    assert!(CollUtil::is_empty(None::<&[i32]>));
}
#[test]
fn c02() {
    assert!(CollUtil::is_empty(Some(&[] as &[i32])));
}
#[test]
fn c03() {
    assert!(!CollUtil::is_empty(Some(&[1] as &[i32])));
}
#[test]
fn c04() {
    assert!(!CollUtil::is_not_empty(None::<&[i32]>));
}
#[test]
fn c05() {
    assert!(!CollUtil::is_not_empty(Some(&[] as &[i32])));
}
#[test]
fn c06() {
    assert!(CollUtil::is_not_empty(Some(&[1] as &[i32])));
}
#[test]
fn c07() {
    assert!(CollUtil::is_not_empty(Some(&[1, 2, 3] as &[i32])));
}
#[test]
fn c08() {
    assert!(CollUtil::is_empty(Some(&[] as &[String])));
}
#[test]
fn c09() {
    assert!(!CollUtil::is_empty(Some(&["a".to_string()] as &[String])));
}
#[test]
fn c10() {
    assert!(CollUtil::is_not_empty(Some(&[true, false] as &[bool])));
}

// ── 列表操作 (10 tests) ──

#[test]
fn l01() {
    let i: Vec<i32> = (1..=10).collect();
    assert_eq!(ListUtil::page(&i, 0, 3).unwrap(), &[1, 2, 3]);
}
#[test]
fn l02() {
    let i: Vec<i32> = (1..=10).collect();
    assert_eq!(ListUtil::page(&i, 1, 3).unwrap(), &[4, 5, 6]);
}
#[test]
fn l03() {
    let i: Vec<i32> = (1..=10).collect();
    assert_eq!(ListUtil::page(&i, 3, 3).unwrap(), &[10]);
}
#[test]
fn l04() {
    let i = vec![1, 2, 3, 4, 5];
    assert_eq!(ListUtil::partition(&i, 2).unwrap().len(), 3);
}
#[test]
fn l05() {
    let i = vec![1, 2, 3, 4, 5];
    assert_eq!(ListUtil::partition(&i, 5).unwrap().len(), 1);
}
#[test]
fn l06() {
    let mut i = vec![3, 1, 2];
    ListUtil::sort_by(&mut i, |a, b| a.cmp(b));
    assert_eq!(i, vec![1, 2, 3]);
}
#[test]
fn l07() {
    let i = vec![1, 2, 3, 4, 5];
    assert_eq!(ListUtil::sub(&i, 1, 3, 1).unwrap(), vec![2, 3]);
}
#[test]
fn l08() {
    let i = vec![1, 2, 3, 4, 5];
    assert_eq!(ListUtil::sub(&i, 0, 5, 2).unwrap(), vec![1, 3, 5]);
}
#[test]
fn l09() {
    let i = vec![1, 2, 3, 2, 1];
    assert_eq!(ListUtil::last_index_of(&i, |x| *x == 2), Some(3));
}
#[test]
fn l10() {
    let i = vec![1, 2, 3, 2, 1];
    assert_eq!(ListUtil::index_of_all(&i, |x| *x == 2), vec![1, 3]);
}

// ── 数组操作 (15 tests) ──

#[test]
fn a01() {
    assert!(ArrayUtil::is_empty(&[] as &[i32]));
}
#[test]
fn a02() {
    assert!(!ArrayUtil::is_empty(&[1]));
}
#[test]
fn a03() {
    assert!(ArrayUtil::contains(&[1, 2, 3], &2));
}
#[test]
fn a04() {
    assert!(!ArrayUtil::contains(&[1, 2, 3], &5));
}
#[test]
fn a05() {
    assert_eq!(ArrayUtil::index_of(&[1, 2, 3, 2], &2), Some(1));
}
#[test]
fn a06() {
    assert_eq!(ArrayUtil::last_index_of(&[1, 2, 3, 2], &2), Some(3));
}
#[test]
fn a07() {
    let mut a = vec![3, 1, 2];
    ArrayUtil::sort(&mut a);
    assert_eq!(a, vec![1, 2, 3]);
}
#[test]
fn a08() {
    let mut a = vec![1, 2, 3];
    ArrayUtil::reverse(&mut a);
    assert_eq!(a, vec![3, 2, 1]);
}
#[test]
fn a09() {
    assert_eq!(ArrayUtil::join(&[1, 2, 3], ", "), "1, 2, 3");
}
#[test]
fn a10() {
    assert_eq!(ArrayUtil::to_string(&[1, 2, 3]), "[1, 2, 3]");
}
#[test]
fn a11() {
    assert_eq!(ArrayUtil::append(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
}
#[test]
fn a12() {
    assert_eq!(ArrayUtil::insert(&[1, 2, 3], 1, &[10]), vec![1, 10, 2, 3]);
}
#[test]
fn a13() {
    assert_eq!(ArrayUtil::remove(&[1, 2, 3], 1), vec![1, 3]);
}
#[test]
fn a14() {
    assert_eq!(ArrayUtil::sub(&[1, 2, 3, 4, 5], 1, 3), vec![2, 3]);
}
#[test]
fn a15() {
    assert_eq!(
        ArrayUtil::split(&[1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
}

// ── 字典操作 (8 tests) ──

#[test]
fn d01() {
    let d = DictUtil::create();
    assert!(d.is_empty());
}
#[test]
fn d02() {
    let d = DictUtil::of(&[("a", json!(1))]);
    assert_eq!(d.len(), 1);
}
#[test]
fn d03() {
    let mut d = DictUtil::create();
    DictUtil::set(&mut d, "k", json!("v"));
    assert_eq!(DictUtil::get_str(&d, "k"), Some("v".to_string()));
}
#[test]
fn d04() {
    let d = DictUtil::of(&[("k", json!(42))]);
    assert_eq!(DictUtil::get_int(&d, "k"), Some(42));
}
#[test]
fn d05() {
    let d = DictUtil::of(&[("k", json!(true))]);
    assert_eq!(DictUtil::get_bool(&d, "k"), Some(true));
}
#[test]
fn d06() {
    let d = DictUtil::of(&[("k", json!(3.14))]);
    assert_eq!(DictUtil::get_float(&d, "k"), Some(3.14));
}
#[test]
fn d07() {
    let d = DictUtil::of(&[("k", json!("v"))]);
    assert!(DictUtil::contains_key(&d, "k"));
}
#[test]
fn d08() {
    let mut d = DictUtil::of(&[("k", json!("v"))]);
    DictUtil::remove(&mut d, "k");
    assert!(d.is_empty());
}

// ── Map 操作 (5 tests) ──

#[test]
fn m01() {
    let m = MapUtil::of(&[("a", 1)]);
    assert_eq!(m.len(), 1);
}
#[test]
fn m02() {
    let m = MapUtil::of(&[("a", 1), ("b", 2)]);
    let f = MapUtil::filter(&m, |_, v| *v > 1);
    assert_eq!(f.len(), 1);
}
#[test]
fn m03() {
    let m1 = MapUtil::of(&[("a", 1)]);
    let m2 = MapUtil::of(&[("b", 2)]);
    let m = MapUtil::merge(m1, m2);
    assert_eq!(m.len(), 2);
}
#[test]
fn m04() {
    let m = MapUtil::of(&[("a", 1)]);
    let inv = MapUtil::inverse(&m);
    assert_eq!(inv.get(&1), Some(&"a"));
}
#[test]
fn m05() {
    let m = MapUtil::of(&[("a", 1), ("b", 2)]);
    let mut k = MapUtil::keys(&m);
    k.sort();
    assert_eq!(k, vec!["a", "b"]);
}

// ── 转义操作 (6 tests) ──

#[test]
fn e01() {
    assert_eq!(EscapeUtil::escape_html("<div>"), "&lt;div&gt;");
}
#[test]
fn e02() {
    assert_eq!(EscapeUtil::unescape_html("&lt;div&gt;"), "<div>");
}
#[test]
fn e03() {
    let i = "<div>test</div>";
    assert_eq!(EscapeUtil::unescape_html(&EscapeUtil::escape_html(i)), i);
}
#[test]
fn e04() {
    assert_eq!(EscapeUtil::escape_java("line\n"), "line\\n");
}
#[test]
fn e05() {
    assert_eq!(EscapeUtil::unescape_java("line\\n"), "line\n");
}
#[test]
fn e06() {
    assert_eq!(EscapeUtil::escape_sql("it's"), "it''s");
}

// ── 正则操作 (10 tests) ──

#[test]
fn r01() {
    assert!(ReUtil::is_match(r"^\d+$", "123"));
}
#[test]
fn r02() {
    assert!(!ReUtil::is_match(r"^\d+$", "abc"));
}
#[test]
fn r03() {
    assert_eq!(ReUtil::find(r"\d+", "abc123"), Some("123".to_string()));
}
#[test]
fn r04() {
    assert_eq!(ReUtil::find(r"\d+", "abc"), None);
}
#[test]
fn r05() {
    assert_eq!(ReUtil::replace_all(r"\d+", "a1b2", "X"), "aXbX");
}
#[test]
fn r06() {
    assert_eq!(ReUtil::replace_first(r"\d+", "a1b2", "X"), "aXb2");
}
#[test]
fn r07() {
    assert!(ReUtil::is_email("test@example.com"));
}
#[test]
fn r08() {
    assert!(!ReUtil::is_email("bad"));
}
#[test]
fn r09() {
    assert!(ReUtil::is_mobile("13800138000"));
}
#[test]
fn r10() {
    assert!(!ReUtil::is_mobile("12345678901"));
}

// ── 数字操作 (11 tests) ──

#[test]
fn n01() {
    assert_eq!(NumberUtil::compare_i32(1, 2), -1);
}
#[test]
fn n02() {
    assert_eq!(NumberUtil::compare_i32(2, 2), 0);
}
#[test]
fn n03() {
    assert_eq!(NumberUtil::compare_i32(3, 2), 1);
}
#[test]
fn n04() {
    assert_eq!(NumberUtil::min_i32(&[3, 1, 4]).unwrap(), 1);
}
#[test]
fn n05() {
    assert_eq!(NumberUtil::max_i32(&[3, 1, 4]).unwrap(), 4);
}
#[test]
fn n06() {
    assert_eq!(NumberUtil::div(10.0, 2.0).unwrap(), 5.0);
}
#[test]
fn n07() {
    assert!(NumberUtil::div(1.0, 0.0).is_err());
}
#[test]
fn n08() {
    assert!(NumberUtil::is_number("123"));
}
#[test]
fn n09() {
    assert!(!NumberUtil::is_number("abc"));
}
#[test]
fn n10() {
    assert!(NumberUtil::is_integer("123"));
}
#[test]
fn n11() {
    assert!(!NumberUtil::is_integer("3.14"));
}

// ── 随机操作 (5 tests) ──

#[test]
fn rd01() {
    for _ in 0..100 {
        let v = RandomUtil::random_int_range(0, 10);
        assert!(v >= 0 && v < 10);
    }
}
#[test]
fn rd02() {
    assert_eq!(RandomUtil::random_string(10).len(), 10);
}
#[test]
fn rd03() {
    assert_eq!(RandomUtil::random_string(0), "");
}
#[test]
fn rd04() {
    assert!(RandomUtil::random_element(&[1, 2, 3]).is_some());
}
#[test]
fn rd05() {
    assert!(RandomUtil::random_element(&[] as &[i32]).is_none());
}

// ── 反射操作 (5 tests) ──

#[test]
fn rf01() {
    assert!(ReflectUtil::is_basic_type::<i32>());
}
#[test]
fn rf02() {
    assert!(!ReflectUtil::is_basic_type::<String>());
}
#[test]
fn rf03() {
    assert_eq!(ReflectUtil::type_name::<i32>(), "i32");
}
#[test]
fn rf04() {
    assert!(ReflectUtil::type_eq::<i32, i32>());
}
#[test]
fn rf05() {
    assert!(!ReflectUtil::type_eq::<i32, f64>());
}

// ── 身份证操作 (3 tests) ──

#[test]
fn id01() {
    assert!(IdcardUtil::is_valid_card_18("11010519491231002X"));
}
#[test]
fn id02() {
    assert!(!IdcardUtil::is_valid_card_18("invalid"));
}
#[test]
fn id03() {
    let g = IdcardUtil::get_gender_by_id_card("11010519491231002X").unwrap();
    assert_eq!(g, 0);
}

// ── 手机号操作 (5 tests) ──

#[test]
fn p01() {
    assert!(PhoneUtil::is_mobile("13800138000"));
}
#[test]
fn p02() {
    assert!(!PhoneUtil::is_mobile("12345678901"));
}
#[test]
fn p03() {
    let h = PhoneUtil::hide_before("13800138000");
    assert!(h.starts_with("***"));
}
#[test]
fn p04() {
    let h = PhoneUtil::hide_between("13800138000");
    assert!(h.starts_with("138"));
}
#[test]
fn p05() {
    let h = PhoneUtil::hide_after("13800138000");
    assert!(h.ends_with("****"));
}

// ── 脱敏操作 (1 test) ──

#[test]
fn ds01() {
    let r = DesensitizedUtil::mobile_phone(Some("13800138000"));
    assert!(r.contains("***"));
}

// ── 版本操作 (3 tests) ──

#[test]
fn v01() {
    assert!(VersionUtil::is_less_than_str("1.0.0", "2.0.0"));
}
#[test]
fn v02() {
    assert!(VersionUtil::is_greater_than_str("2.0.0", "1.0.0"));
}
#[test]
fn v03() {
    assert!(VersionUtil::is_greater_than_or_equal_str("1.0.0", "1.0.0"));
}

// ── 分页操作 (3 tests) ──

#[test]
fn pg01() {
    assert_eq!(PageUtil::total_page_i32(100, 10).unwrap(), 10);
}
#[test]
fn pg02() {
    assert_eq!(PageUtil::total_page_i32(101, 10).unwrap(), 11);
}
#[test]
fn pg03() {
    assert_eq!(PageUtil::total_page_i32(0, 10).unwrap(), 0);
}

// ── 信用代码 (2 tests) ──

#[test]
fn cc01() {
    assert!(CreditCodeUtil::is_credit_code_simple("91350100M000100Y43"));
}
#[test]
fn cc02() {
    assert!(!CreditCodeUtil::is_credit_code_simple("invalid"));
}

// ── 布尔操作 (3 tests) ──

#[test]
fn b01() {
    assert!(BooleanUtil::is_true(Some(true)));
}
#[test]
fn b02() {
    assert!(!BooleanUtil::is_true(Some(false)));
}
#[test]
fn b03() {
    assert!(!BooleanUtil::is_true(None));
}

// ── 十六进制操作 (2 tests) ──

#[test]
fn h01() {
    let e = HexUtil::encode_hex(b"Hello");
    let d = HexUtil::decode_hex(&e).unwrap();
    assert_eq!(d, b"Hello");
}
#[test]
fn h02() {
    assert_eq!(HexUtil::encode_hex(b""), "");
}

// ── 哈希操作 (2 tests) ──

#[test]
fn hs01() {
    assert_eq!(HashUtil::fnv_hash("hello"), HashUtil::fnv_hash("hello"));
}
#[test]
fn hs02() {
    assert_ne!(HashUtil::fnv_hash("hello"), HashUtil::fnv_hash("world"));
}

// ── 文件操作 (3 tests) ──

#[test]
fn f01() {
    assert!(FileUtil::exists("/tmp"));
}
#[test]
fn f02() {
    assert!(!FileUtil::exists("/nonexistent"));
}
#[test]
fn f03() {
    let p = "/tmp/test_expansion.txt";
    FileUtil::write_utf8_string(p, "test").unwrap();
    assert_eq!(FileUtil::read_utf8_string(p).unwrap(), "test");
    FileUtil::delete(p).unwrap();
}

// ── IO 操作 (3 tests) ──

#[test]
fn i01() {
    use std::io::Cursor;
    let mut r = Cursor::new(b"hi");
    let mut w = Cursor::new(Vec::new());
    IoUtil::copy(&mut r, &mut w).unwrap();
    assert_eq!(w.into_inner(), b"hi");
}
#[test]
fn i02() {
    assert_eq!(IoUtil::bytes_to_hex(&[0xCA, 0xFE]), "cafe");
}
#[test]
fn i03() {
    assert_eq!(IoUtil::hex_to_bytes("cafe").unwrap(), vec![0xCA, 0xFE]);
}

// ── URL 操作 (5 tests) ──

#[test]
fn u01() {
    assert!(UrlUtil::is_url("http://example.com"));
}
#[test]
fn u02() {
    assert!(!UrlUtil::is_url("not-url"));
}
#[test]
fn u03() {
    assert_eq!(
        UrlUtil::get_host("http://example.com/path"),
        Some("example.com")
    );
}
#[test]
fn u04() {
    assert_eq!(UrlUtil::get_protocol("https://x.com"), Some("https"));
}
#[test]
fn u05() {
    assert_eq!(UrlUtil::encode("a b"), "a+b");
}

// ── XML 操作 (5 tests) ──

#[test]
fn x01() {
    assert!(XmlUtil::is_xml("<root/>"));
}
#[test]
fn x02() {
    assert!(!XmlUtil::is_xml("not xml"));
}
#[test]
fn x03() {
    assert_eq!(XmlUtil::escape("<tag>"), "&lt;tag&gt;");
}
#[test]
fn x04() {
    assert_eq!(XmlUtil::unescape("&lt;tag&gt;"), "<tag>");
}
#[test]
fn x05() {
    assert_eq!(XmlUtil::element("name", "val"), "<name>val</name>");
}

// ── 类型操作 (4 tests) ──

#[test]
fn t01() {
    assert!(TypeUtil::is_basic_type::<i32>());
}
#[test]
fn t02() {
    assert!(!TypeUtil::is_basic_type::<String>());
}
#[test]
fn t03() {
    assert!(TypeUtil::is_number::<f64>());
}
#[test]
fn t04() {
    assert!(!TypeUtil::is_number::<bool>());
}

// ── 验证操作 (5 tests) ──

#[test]
fn vl01() {
    assert!(Validator::is_email("test@example.com"));
}
#[test]
fn vl02() {
    assert!(!Validator::is_email("bad"));
}
#[test]
fn vl03() {
    assert!(Validator::is_mobile("13800138000"));
}
#[test]
fn vl04() {
    assert!(Validator::is_between("abc", 1, 5));
}
#[test]
fn vl05() {
    assert!(!Validator::is_between("abcdef", 1, 5));
}

// ── 对象操作 (4 tests) ──

#[test]
fn o01() {
    assert_eq!(ObjectUtil::default_if_null(Some(&42), &0), 42);
}
#[test]
fn o02() {
    assert_eq!(ObjectUtil::default_if_null(None::<&i32>, &0), 0);
}
#[test]
fn o03() {
    assert!(ObjectUtil::equal(Some(&42), Some(&42)));
}
#[test]
fn o04() {
    assert!(!ObjectUtil::equal(Some(&42), Some(&43)));
}

// ── 枚举操作 (3 tests) ──

#[test]
fn en01() {
    #[derive(Debug, Clone)]
    enum E {
        A,
        B,
    }
    let v = vec![E::A, E::B];
    assert_eq!(EnumUtil::names(&v), vec!["A", "B"]);
}
#[test]
fn en02() {
    #[derive(Debug, Clone)]
    enum E {
        A,
        B,
    }
    let v = vec![E::A, E::B];
    assert!(EnumUtil::contains_name(&v, "A"));
}
#[test]
fn en03() {
    #[derive(Debug, Clone)]
    enum E {
        A,
        B,
    }
    let v = vec![E::A, E::B];
    assert!(!EnumUtil::contains_name(&v, "C"));
}
