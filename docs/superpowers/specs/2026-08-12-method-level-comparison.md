# 方法级对照聚合设计规格

日期：2026-08-12
来源：合并根目录 4 个 METHOD_COMPARISON 文档（CODEGRAPH_STRUTIL_COMPARISON、CORE_METHOD_COMPARISON、HTTP_METHOD_COMPARISON、SETTING_METHOD_COMPARISON）

## 1. 背景

根目录存在 4 个方法级对照文档，分别覆盖 StrUtil、hutool-core 核心类、HTTP 和 Setting。
本规格将它们聚合为统一的方法级对照参考，按模块分节。

## 2. hutool-core 方法级对照总览

| 类 | Java 方法数 | Rust 方法数 | 已实现 | 桩函数 | 覆盖率 | 状态 |
|----|-----------|-----------|--------|--------|--------|------|
| StrUtil (CharSequenceUtil) | ~268 | ~33+25 | ~25 (string.rs) | 33 (str_util.rs) | ~28% | 桩为主 |
| ObjectUtil | 41 | 41 | 0 | 41 | 0% | 全部桩 |
| ArrayUtil | 99 | 99 | 0 | 99 | 0% | 全部桩 |
| DateUtil | ~95 | ~75 | ~70 | 0 | ~74% | 大部分实现 |
| NumberUtil | ~130 | ~130 | ~15 | ~115 | ~12% | 桩为主 |
| CollUtil | ~160 | ~60 | ~45 | 4 | ~38% | 部分实现 |
| MapUtil | ~75 | ~65 | ~55 | 0 | ~87% | 大部分实现 |
| **总计** | **~868** | **~528** | **~210** | **~317** | **~40%** | |

## 3. StrUtil 方法映射（节选）

### str_util.rs (33 方法)

| Java 方法 | Rust 方法 | 状态 |
|---|---|---|
| isBlankIfStr(Object) | is_blank_if_str(Option<&str>) | done |
| isEmptyIfStr(Object) | is_empty_if_str(Option<&str>) | done |
| trim(String[]) | trim(&mut [String]) | done |
| utf8Str(Object) | utf8_str(Option<&dyn Display>) | done |
| toString(Object) | to_string(Option<&dyn Display>) | done |
| builder() | builder() | done |
| reverse(String) | reverse(&str) | done |
| fillBefore(String, char, int) | fill_before(&str, char, usize) | done |
| similar(String, String) | similar(&str, &str) | done |
| uuid() | uuid() | done |
| format(CharSequence, Map) | format(&str, &HashMap<&str, &str>) | done |

### string.rs (42 函数，对应 CharSequenceUtil 核心方法)

| Java CharSequenceUtil 方法 | Rust 函数 | 状态 |
|---|---|---|
| isBlank(CharSequence) | is_blank(&str) | done |
| trim(CharSequence) | trim(&str) | done |
| contains(CharSequence, CharSequence) | contains(&str, &str) | done |
| startWith(...) | start_with(&str, &str) | done |
| endWith(...) | end_with(&str, &str) | done |
| replace(...) | replace(&str, &str, &str) | done |
| reverse(String) | reverse(&str) | done |
| equals(...) | equals(&str, &str) | done |
| equalsIgnoreCase(...) | equals_ignore_case(&str, &str) | done |
| upperFirst(...) | upper_first(&str) | done |
| lowerFirst(...) | lower_first(&str) | done |
| split(...) | split(&str, char, bool, bool) | done |
| length(CharSequence) | length(Option<&str>) | done |
| indexedFormat(...) | indexed_format(&str, &[&dyn Display]) | done |

## 4. DateUtil 方法映射（节选）

| Java 方法 | Rust 方法 | 差异 |
|---|---|---|
| static Date date() | fn date() -> DateTime | chrono 类型 |
| static long current() | fn current() -> i64 | done |
| static String format(Date, String) | fn format(date: DateTime, pattern: &str) -> String | done |
| static Date parse(String) | fn parse(date_str: &str) -> Result<DateTime> | 返回 Result |
| static Date beginOfDay(Date) | fn begin_of_day(date: DateTime) -> DateTime | done |
| static long between(Date, Date, DateUnit) | fn between(begin, end, unit) -> i64 | done |

## 5. CollUtil 方法映射（节选）

| Java 方法 | Rust 方法 | 差异 |
|---|---|---|
| static boolean isEmpty(Collection) | fn is_empty<T>(Option<&[T]>) -> bool | 入参改 Option<&[T]> |
| static <T> T get(Collection, int) | fn get<T>(&[T], isize) -> Option<&T> | 返回 Option<&T> |
| static Collection newHashSet(...) | fn new_hash_set<T>(...) -> HashSet<T> | done |
| static String join(Collection, CharSequence) | fn join<T: Display>(...) -> String | done |

## 6. hutool-http 方法对照

| hutool 类 | hutool-rust | 状态 |
|---|---|---|
| HttpUtil | client::HttpClient + 顶层 fn | 重写为 client + static fn 混合 |
| HttpRequest | request::HttpRequest | done |
| HttpResponse | response::HttpResponse | done |
| Method | method::Method | done |
| UserAgent/Parser | useragent 完整 | done |
| Cookie/CookieJar | cookie 完整 | done |
| RequestBody 系列 | body 完整 | done |
| HttpServer 系列 | -- | 未迁移 |
| SSL 系列 | -- | 由 reqwest TLS 替代 |
| SoapClient 系列 | -- | 未迁移 |

## 7. hutool-setting 方法对照

| hutool 类 | hutool-rust | 状态 |
|---|---|---|
| Setting | Setting | done |
| Props | Props | done |
| Profile/GlobalProfile | Profile/GlobalProfile | done |
| SettingUtil/PropsUtil | SettingUtil/PropsUtil | done |
| YamlUtil | YamlUtil | done |

完成度约 95%。

## 8. 参考

- 原始文档：`CODEGRAPH_STRUTIL_COMPARISON.md`、`CORE_METHOD_COMPARISON.md`、`HTTP_METHOD_COMPARISON.md`、`SETTING_METHOD_COMPARISON.md`（已删除，内容已合并至此）
