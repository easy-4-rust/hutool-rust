# CodeGraph 方法级对比: StrUtil (Java vs Rust)

> 使用 codegraph 工具对 hutool-core StrUtil 进行方法级对比
> 对比时间: 2026-07-25

## 总览

| 指标 | Java | Rust | 覆盖率 |
|------|------|------|--------|
| CharSequenceUtil 方法 | ~268 | — | — |
| StrUtil 特有方法 | ~29 | — | — |
| Rust str_util.rs 方法 | — | 33 | — |
| Rust string.rs 函数 | — | 42 | — |
| **Rust 总方法** | — | **75** | **~28%** |
| 已实现 (非桩) | — | 75 | **100%** |
| PendingEngine 桩 | — | 0 | **0%** |

## 方法映射表

### str_util.rs (33 方法, 全部已实现)

| Java 方法 | Rust 方法 | 参数匹配 | 返回匹配 | 实现 |
|-----------|----------|---------|---------|------|
| `isBlankIfStr(Object)` | `is_blank_if_str(Option<&str>)` | ✅ | ✅ | ✅ |
| `isEmptyIfStr(Object)` | `is_empty_if_str(Option<&str>)` | ✅ | ✅ | ✅ |
| `trim(String[])` | `trim(&mut [String])` | ✅ | ✅ | ✅ |
| `utf8Str(Object)` | `utf8_str(Option<&dyn Display>)` | ✅ | ✅ | ✅ |
| `str(Object, String)` | `str(&dyn Display, &str)` | ✅ | ✅ | ✅ |
| `str(Object, Charset)` | `str_2(&dyn Display, &str)` | ✅ | ✅ | ✅ |
| `str(byte[], String)` | `str_3(&[u8], &str)` | ✅ | ✅ | ✅ |
| `str(byte[], Charset)` | `str_4(&[u8], &str)` | ✅ | ✅ | ✅ |
| `str(Byte[], String)` | `str_5(&[Option<u8>], &str)` | ✅ | ✅ | ✅ |
| `str(Byte[], Charset)` | `str_6(&[Option<u8>], &str)` | ✅ | ✅ | ✅ |
| `str(ByteBuffer, String)` | `str_7(&[u8], &str)` | ✅ | ✅ | ✅ |
| `str(ByteBuffer, Charset)` | `str_8(&[u8], &str)` | ✅ | ✅ | ✅ |
| `toString(Object)` | `to_string(Option<&dyn Display>)` | ✅ | ✅ | ✅ |
| `toStringOrNull(Object)` | `to_string_or_null(Option<&dyn Display>)` | ✅ | ✅ | ✅ |
| `toStringOrEmpty(Object)` | `to_string_or_empty(Option<&dyn Display>)` | ✅ | ✅ | ✅ |
| `builder()` | `builder()` | ✅ | ✅ | ✅ |
| `strBuilder()` | `str_builder()` | ✅ | ✅ | ✅ |
| `builder(int)` | `builder_with_capacity(usize)` | ✅ | ✅ | ✅ |
| `strBuilder(int)` | `str_builder_with_capacity(usize)` | ✅ | ✅ | ✅ |
| `getReader(CharSequence)` | `get_reader(&str)` | ✅ | ✅ | ✅ |
| `getWriter()` | `get_writer()` | ✅ | ✅ | ✅ |
| `reverse(String)` | `reverse(&str)` | ✅ | ✅ | ✅ |
| `reverseByCodePoint(String)` | `reverse_by_code_point(&str)` | ✅ | ✅ | ✅ |
| `fillBefore(String, char, int)` | `fill_before(&str, char, usize)` | ✅ | ✅ | ✅ |
| `fillAfter(String, char, int)` | `fill_after(&str, char, usize)` | ✅ | ✅ | ✅ |
| `fill(String, char, int, boolean)` | `fill(&str, char, usize, bool)` | ✅ | ✅ | ✅ |
| `similar(String, String)` | `similar(&str, &str)` | ✅ | ✅ | ✅ |
| `similar(String, String, int)` | `similar_str(&str, &str, usize)` | ✅ | ✅ | ✅ |
| `uuid()` | `uuid()` | ✅ | ✅ | ✅ |
| `format(CharSequence, Map)` | `format(&str, &HashMap<&str, &str>)` | ✅ | ✅ | ✅ |
| `format(CharSequence, Map, boolean)` | `format_ignore_null(...)` | ✅ | ✅ | ✅ |
| `truncateUtf8(String, int)` | `truncate_utf8(&str, usize)` | ✅ | ✅ | ✅ |
| `truncateByByteLength(...)` | `truncate_by_byte_length(...)` | ✅ | ✅ | ✅ |

### string.rs (42 函数, 对应 CharSequenceUtil 核心方法)

| Java CharSequenceUtil 方法 | Rust 函数 | 实现 |
|---------------------------|----------|------|
| `isBlank(CharSequence)` | `is_blank(&str)` | ✅ |
| `trim(CharSequence)` | `trim(&str)` | ✅ |
| `cleanBlank(CharSequence)` | `clean_blank(&str)` | ✅ |
| `contains(CharSequence, CharSequence)` | `contains(&str, &str)` | ✅ |
| `containsIgnoreCase(...)` | `contains_ignore_case(&str, &str)` | ✅ |
| `startWith(...)` | `start_with(&str, &str)` | ✅ |
| `endWith(...)` | `end_with(&str, &str)` | ✅ |
| `replace(...)` | `replace(&str, &str, &str)` | ✅ |
| `reverse(String)` | `reverse(&str)` | ✅ |
| `repeat(CharSequence, int)` | `repeat(&str, usize)` | ✅ |
| `equals(...)` | `equals(&str, &str)` | ✅ |
| `equalsIgnoreCase(...)` | `equals_ignore_case(&str, &str)` | ✅ |
| `upperFirst(...)` | `upper_first(&str)` | ✅ |
| `lowerFirst(...)` | `lower_first(&str)` | ✅ |
| `indexOfIgnoreCase(...)` | `index_of_ignore_case(&str, &str)` | ✅ |
| `lastIndexOf(...)` | `last_index_of(&str, &str)` | ✅ |
| `lastIndexOfIgnoreCase(...)` | `last_index_of_ignore_case(&str, &str)` | ✅ |
| `strip(...)` | `strip(&str, &str)` | ✅ |
| `stripIgnoreCase(...)` | `strip_ignore_case(&str, &str)` | ✅ |
| `cut(...)` | `cut(&str, usize)` | ✅ |
| `split(...)` | `split(&str, char, bool, bool)` | ✅ |
| `length(CharSequence)` | `length(Option<&str>)` | ✅ |
| `subByCodePoint(...)` | `sub_by_code_point(&str, i32, i32)` | ✅ |
| `replaceByCodePoint(...)` | `replace_by_code_point(...)` | ✅ |
| `indexedFormat(...)` | `indexed_format(&str, &[&dyn Display])` | ✅ |
| `splitToArray(...)` | `split_to_array(Option<&str>, char)` | ✅ |
| `removeAll(...)` | `remove_all(&str, &str)` | ✅ |
| `format(CharSequence, Object...)` | `format_template(&str, &[&dyn Display])` | ✅ |

## CodeGraph 调用关系

```
str_util.rs ──calls──→ string.rs
  ├── reverse() ──→ string::reverse()
  ├── fill_before() ──→ string::fill_before()
  ├── fill_after() ──→ string::fill_after()
  ├── fill() ──→ string::fill()
  ├── similar() ──→ string::similarity()
  ├── format() ──→ string::format_map()
  ├── truncate_utf8() ──→ string::truncate_utf8()
  └── truncate_by_byte_length() ──→ string::truncate_by_byte_length()
```

## 结论

- **Rust 总方法**: 75 (str_util 33 + string 42)
- **Java 总方法**: ~268 (CharSequenceUtil) + ~29 (StrUtil 特有) = ~297
- **覆盖率**: ~28% (方法数), 但核心功能全部覆盖
- **桩函数**: 0 (全部已实现)
- **PendingEngine**: 0 (全部已移除)
