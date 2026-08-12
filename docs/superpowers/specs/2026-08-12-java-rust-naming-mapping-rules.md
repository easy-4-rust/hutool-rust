# Java -> Rust 命名映射规则设计规格

日期：2026-08-12

## 1. 背景

hutool-rust 的核心目标是"一比一复刻 Hutool Java"，命名严格对齐 Java 源码。
Java 与 Rust 的命名风格差异（PascalCase vs snake_case、getXxx vs xxx()）需要一套
统一、可验证的映射规则，让开发者和自动化工具都能判断"对齐"还是"偏离"。

本规格从 `IMPLEMENTATION_PLAN.md` 第三节提炼并固化，作为所有迁移工作的命名权威源。

## 2. 目标与非目标

### 2.1 目标

- 提供一套可机器验证的命名映射规则，覆盖包/类/方法/参数/常量/文件各层级。
- 规则必须能被 `scripts/verify-parity.py` 和 CI 静态检查覆盖。
- 保留 Java 业务语义（动词、领域词汇不翻译），仅做 Rust 风格转换。

### 2.2 非目标

- 不重新发明已有的 Rust 社区命名惯例（如 `impl Trait` 命名）。
- 不强制 Java `interface` 映射为特定 Rust trait 命名后缀（由具体设计决定）。
- 不覆盖 `unsafe-to-copy` 文件的内部实现命名（只需文件名对齐）。

## 3. 映射规则

### 3.1 包 -> crate + mod

| Java | Rust | 示例 |
|---|---|---|
| `cn.hutool.core.collection` | `hutool_core::collection` | crate 名用连字符，mod 用下划线 |
| `cn.hutool.core.date.format` | `hutool_core::date::format` | 子包 = 子目录 |
| `cn.hutool.extra.qrcode` | `hutool_extra::qrcode` | 同上 |

### 3.2 类 -> struct / enum / trait

| Java | Rust | 示例 |
|---|---|---|
| `class CollUtil` | `pub struct CollUtil` | PascalCase 保留 |
| `class DateUnit` (enum-like) | `pub enum DateUnit` | 同上 |
| `interface Foo` | `pub trait Foo` | 同上 |
| `abstract class Bar` | `pub trait Bar` | 抽象类 -> trait |
| `record Foo(A, B)` | `pub struct Foo(pub A, pub B)` | Java 14+ record -> tuple struct |

### 3.3 方法 -> fn

| Java | Rust | 规则 |
|---|---|---|
| `isEmpty()` | `is_empty()` | snake_case 转换 |
| `getDate()` | `date()` | getter 去掉 `get` 前缀 |
| `setDate(d)` | `set_date(d)` | setter 保留 `set_` 前缀 |
| `isXxx()` | `is_xxx()` | 保留 `is_` 前缀 |
| `hasXxx()` | `has_xxx()` | 保留 `has_` 前缀 |
| `static method()` | `Struct::method()` | 静态方法 -> 关联函数 |
| `instance.method()` | `self.method()` | 实例方法 -> &self 方法 |

### 3.4 参数与变量

| Java | Rust | 示例 |
|---|---|---|
| `String name` | `name: &str` | 参数倾向 `&str`，返回 `String` |
| `int count` | `count: i32` | 默认 `i32` |
| `long timestamp` | `timestamp: i64` | Java long = Rust i64 |
| `List<String> items` | `items: Vec<String>` | 泛型容器 |
| `Map<K,V> map` | `map: HashMap<K, V>` | 同上 |
| `collUtil` (变量) | `coll_util` | snake_case |

### 3.5 常量

| Java | Rust | 规则 |
|---|---|---|
| `static final int MAX_SIZE = 100` | `pub const MAX_SIZE: i32 = 100` | SCREAMING_SNAKE 不变 |
| `static final String DEFAULT_CHARSET` | `pub const DEFAULT_CHARSET: &str = "UTF-8"` | 同上 |

### 3.6 文件名

| Java | Rust | 规则 |
|---|---|---|
| `CollUtil.java` | `coll_util.rs` | snake_case 文件名 |
| `package-info.java` | `mod.rs` | 目录索引 |
| `BeanUtil.java` | `bean_util.rs` | 同上 |

### 3.7 必须保留的业务动词（禁止翻译）

| Java 业务动词 | Rust 命名 | 出现模块 |
|---|---|---|
| `login` / `logout` | `login` / `logout` | core, extra |
| `kickout` / `replaced` | `kickout` / `replaced` | core |
| `openSafe` / `closeSafe` | `open_safe` / `close_safe` | core |
| `parseObj` / `toBean` | `parse_obj` / `to_bean` | json |
| `toJsonStr` / `fromJsonStr` | `to_json_str` / `from_json_str` | json |
| `send` / `recv` | `send` / `recv` | socket, http |
| `upload` / `download` | `upload` / `download` | ftp, http |
| `digest` / `encrypt` / `decrypt` | `digest` / `encrypt` / `decrypt` | crypto |
| `sign` / `verify` | `sign` / `verify` | crypto, jwt |

## 4. 类型映射表

| Java | Rust | 备注 |
|---|---|---|
| `String` | `String` / `&str` | 参数 `&str`，返回 `String` |
| `int` / `long` | `i32` / `i64` | 默认 i64（Java long） |
| `boolean` | `bool` | — |
| `List<T>` | `Vec<T>` | — |
| `Map<K,V>` | `HashMap<K,V>` | 顺序保留用 `IndexMap` |
| `Set<T>` | `HashSet<T>` | 顺序保留用 `IndexSet` |
| `Collection<T>` | `&[T]` | 只读场景 |
| `Optional<T>` | `Option<T>` | — |
| `Date` / `Calendar` | `chrono::NaiveDateTime` | 类型别名 `DateTime` |
| `InputStream` | `R: Read` | trait 泛型 |
| `OutputStream` | `W: Write` | trait 泛型 |
| `Throwable` | `Result<T, E>` | — |
| `Class<T>` | `TypeId` | 类型标识 |
| `Object` | `serde_json::Value` | 通用对象 |

## 5. 验证方式

```bash
# 1. 命名一致性检查（按模块）
python3 scripts/verify-parity.py --by-module

# 2. 文件名映射检查
# 每个 Java 文件名 -> camelCase_to_snake_case -> 检查对应 .rs 是否存在

# 3. 方法名映射检查
# 每个 Java public 方法名 -> snake_case -> 检查 Rust struct 是否有同名 fn
```

## 6. 参考

- `IMPLEMENTATION_PLAN.md` 第三节"命名映射规则（严格对齐 Java）"
- `PHASE_BASELINE.md` API 覆盖率基线
- `parity/decisions.csv` 每个 API 的命名决策记录
