//! Bloom filter parity tests —— 对齐 Hutool `hutool-bloomFilter` 测试。
//!
//! 对齐: `cn.hutool.bloomfilter.BitMapBloomFilterTest`
//! 对齐: `cn.hutool.bloomfilter.BitSetBloomFilterTest`
//! 来源:
//! - hutool-bloomFilter/src/test/java/cn/hutool/bloomfilter/BitMapBloomFilterTest.java
//! - hutool-bloomFilter/src/test/java/cn/hutool/bloomfilter/BitSetBloomFilterTest.java

use hutool_bloom_filter::{
    BitMap, BitMapBloomFilter, BitSetBloomFilter, DefaultFilter, IntMap, LongMap, MachineWord,
    StringBloomFilter,
};

/// 对齐 Java: `BitMapBloomFilterTest.filterTest()`
///
/// Java 逻辑：`new BitMapBloomFilter(10)`，add `"123"/"abc"/"ddd"`，assert contains 三者均为 true。
#[test]
fn filter_test() {
    let mut filter = BitMapBloomFilter::new(10).expect("BitMapBloomFilter::new(10)");
    filter.add("123");
    filter.add("abc");
    filter.add("ddd");

    assert!(
        filter.contains("abc"),
        "对齐 Java BitMapBloomFilterTest.filterTest assertTrue(contains(\"abc\"))"
    );
    assert!(
        filter.contains("ddd"),
        "对齐 Java BitMapBloomFilterTest.filterTest assertTrue(contains(\"ddd\"))"
    );
    assert!(
        filter.contains("123"),
        "对齐 Java BitMapBloomFilterTest.filterTest assertTrue(contains(\"123\"))"
    );
}

/// 对齐 Java: `BitMapBloomFilterTest.testIntMap()`（Java 侧为 `@Disabled` 打印型用例）
///
/// Java 逻辑：IntMap add 0..31，remove(30)，再打印 contains；此处改为可断言的对等验证。
#[test]
fn test_int_map() {
    let mut int_map = IntMap::default();
    for i in 0..32u64 {
        int_map.add(i).expect("IntMap::add");
    }
    int_map.remove(30).expect("IntMap::remove(30)");

    for i in 0..32u64 {
        let present = int_map.contains(i).expect("IntMap::contains");
        if i == 30 {
            assert!(
                !present,
                "对齐 Java testIntMap: remove(30) 后 contains(30) 应为 false"
            );
        } else {
            assert!(present, "对齐 Java testIntMap: contains({i}) 应为 true");
        }
    }
}

/// 对齐 Java: `BitMapBloomFilterTest.testLongMap()`（Java 侧为 `@Disabled` 打印型用例）
///
/// Java 逻辑：LongMap add 0..63，remove(30)，再打印 contains；此处改为可断言的对等验证。
#[test]
fn test_long_map() {
    let mut long_map = LongMap::default();
    for i in 0..64u64 {
        long_map.add(i).expect("LongMap::add");
    }
    long_map.remove(30).expect("LongMap::remove(30)");

    for i in 0..64u64 {
        let present = long_map.contains(i).expect("LongMap::contains");
        if i == 30 {
            assert!(
                !present,
                "对齐 Java testLongMap: remove(30) 后 contains(30) 应为 false"
            );
        } else {
            assert!(present, "对齐 Java testLongMap: contains({i}) 应为 true");
        }
    }
}

/// 对齐 Java: `BitSetBloomFilterTest.testConstructorWithInvalidParameters()`
///
/// Java 逻辑：对 c<=0、n<=0、k∉[1,8] 的构造参数断言抛出 IllegalArgumentException。
#[test]
fn test_constructor_with_invalid_parameters() {
    assert!(
        BitSetBloomFilter::new(0, 100, 3).is_err(),
        "对齐 Java: c=0 应失败"
    );
    assert!(
        BitSetBloomFilter::new(200, 0, 3).is_err(),
        "对齐 Java: n=0 应失败"
    );
    assert!(
        BitSetBloomFilter::new(200, 100, 0).is_err(),
        "对齐 Java: k=0 应失败"
    );
    assert!(
        BitSetBloomFilter::new(200, 100, 9).is_err(),
        "对齐 Java: k=9 应失败"
    );
}

/// 对齐 Java: `AbstractFilterTest.testInitWhenMaxValueLessThanMachineNum()`
///
/// Java 逻辑：DefaultFilter(maxValue, MACHINE32/64) 在 maxValue < machineNum 时 add 不抛异常。
#[test]
fn test_init_when_max_value_less_than_machine_num() {
    let cases = [
        (1u64, MachineWord::Bits32),
        (31, MachineWord::Bits32),
        (1, MachineWord::Bits64),
        (63, MachineWord::Bits64),
    ];
    for (max_value, machine) in cases {
        let mut filter = DefaultFilter::with_machine(max_value, machine).unwrap_or_else(|err| {
            panic!("对齐 Java AbstractFilterTest: DefaultFilter({max_value}, {machine:?}) 构造失败: {err}")
        });
        filter.add("init");
    }
}
