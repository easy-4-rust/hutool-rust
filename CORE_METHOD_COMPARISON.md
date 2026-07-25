# hutool-core 方法级对比报告

> 对比时间：2026-07-25
> Java 源：`/Users/wandl/workspaces/workspace-github/hutool/hutool-core`
> Rust 源：`/Users/wandl/workspaces/workspace-github/hutool-rust/crates/hutool-core`

---

## 总览

| 类 | Java 方法数 | Rust 方法数 | 已实现 | 桩函数 | 覆盖率 | 状态 |
|----|-----------|-----------|--------|--------|--------|------|
| StrUtil (CharSequenceUtil) | ~268 | ~33+25 | ~25 (string.rs) | 33 (str_util.rs) | ~9% 实现 | ⚠️ 桩为主 |
| ObjectUtil | 41 | 41 | 0 | 41 | 0% | ❌ 全部桩 |
| ArrayUtil | 99 | 99 | 0 | 99 | 0% | ❌ 全部桩 |
| DateUtil | ~95 | ~75 | ~70 | 0 | ~74% | ✅ 大部分实现 |
| NumberUtil | ~130 | ~130 | ~15 | ~115 | ~12% | ⚠️ 桩为主 |
| CollUtil | ~160 | ~60 | ~45 | 4 | ~38% 方法数 | ⚠️ 部分实现 |
| MapUtil | ~75 | ~65 | ~55 | 0 | ~87% | ✅ 大部分实现 |
| **总计** | **~868** | **~528** | **~210** | **~317** | **~40%** | |

---

## 1. StrUtil / CharSequenceUtil

**Java**: `cn.hutool.core.util.StrUtil` (extends CharSequenceUtil, ~268 方法)
**Rust**: `str_util.rs` (33 桩) + `string.rs` (~25 实现)

### 架构说明

- `str_util.rs` 是**纯桩文件**，所有方法返回 `Err(CoreError::PendingEngine(...))`
- `string.rs` 包含**真实实现**，对应 CharSequenceUtil 的核心方法
- Java 的 `Object` 参数在 Rust 中用 `*const ()` (原始指针) 替代

### string.rs 已实现方法

| Java 方法 | Rust 方法 | 参数匹配 | 返回匹配 | 实现 |
|-----------|----------|---------|---------|------|
| `isBlank(CharSequence)` | `is_blank(&str)` | ✅ | ✅ | ✅ |
| `trim(CharSequence)` | `trim(&str)` | ✅ | ⚠️ 返回 &str | ✅ |
| `cleanBlank(CharSequence)` | `clean_blank(&str)` | ✅ | ✅ | ✅ |
| `contains(CharSequence, CharSequence)` | `contains(&str, &str)` | ✅ | ✅ | ✅ |
| `containsIgnoreCase(...)` | `contains_ignore_case(&str, &str)` | ✅ | ✅ | ✅ |
| `startWith(...)` | `start_with(&str, &str)` | ✅ | ✅ | ✅ |
| `endWith(...)` | `end_with(&str, &str)` | ✅ | ✅ | ✅ |
| `replace(...)` | `replace(&str, &str, &str)` | ✅ | ✅ | ✅ |
| `reverse(String)` | `reverse(&str)` | ✅ | ✅ | ✅ |
| `repeat(CharSequence, int)` | `repeat(&str, usize)` | ⚠️ int→usize | ✅ | ✅ |
| `equals(...)` | `equals(&str, &str)` | ✅ | ✅ | ✅ |
| `equalsIgnoreCase(...)` | `equals_ignore_case(&str, &str)` | ✅ | ✅ | ✅ |
| `upperFirst(...)` | `upper_first(&str)` | ✅ | ✅ | ✅ |
| `lowerFirst(...)` | `lower_first(&str)` | ✅ | ✅ | ✅ |
| `indexOfIgnoreCase(...)` | `index_of_ignore_case(&str, &str)` | ✅ | ⚠️ Option<usize> | ✅ |
| `lastIndexOf(...)` | `last_index_of(&str, &str)` | ✅ | ⚠️ Option<usize> | ✅ |
| `strip(...)` | `strip(&str, &str)` | ✅ | ✅ | ✅ |
| `cut(...)` | `cut(&str, usize)` | ⚠️ | ⚠️ Result<Vec> | ✅ |
| `indexedFormat(...)` | `indexed_format(&str, &[&dyn Display])` | ⚠️ | ⚠️ Result | ✅ |
| `subByCodePoint(...)` | `sub_by_code_point(&str, i32, i32)` | ✅ | ⚠️ Result | ✅ |
| `splitToArray(...)` | `split_to_array_limit(...)` | ⚠️ | ⚠️ Result<Vec> | ✅ |

### str_util.rs 桩方法 (33 个，全部 PendingEngine)

| Java 方法 | Rust 方法 | 状态 |
|-----------|----------|------|
| `isBlankIfStr(Object)` | `isBlankIfStr(*const ())` | ❌ 桩 |
| `isEmptyIfStr(Object)` | `isEmptyIfStr(*const ())` | ❌ 桩 |
| `utf8Str(Object)` | `utf8Str(*const ())` | ❌ 桩 |
| `str(Object, String)` | `str(*const (), *const ())` | ❌ 桩 |
| `str(byte[], Charset)` | `str_3(Vec<i8>, *const ())` | ❌ 桩 |
| `toString(Object)` | `toString(*const ())` | ❌ 桩 |
| `builder()` | `builder()` | ❌ 桩 |
| `strBuilder()` | `strBuilder()` | ❌ 桩 |
| `similar(String, String)` | `similar(*const (), *const ())` | ❌ 桩 |
| `uuid()` | `uuid()` | ❌ 桩 |
| `format(CharSequence, Map)` | `format(*const (), HashMap<OPAQUE, OPAQUE>)` | ❌ 桩 |
| ... | (共 33 个) | ❌ 桩 |

---

## 2. ObjectUtil

**Java**: `cn.hutool.core.util.ObjectUtil` (41 方法)
**Rust**: `object_util.rs` (41 桩，**0 实现**)

### 全部桩函数

| Java 方法 | Rust 方法 | 状态 | 备注 |
|-----------|----------|------|------|
| `equals(Object, Object)` | `equals(*const (), *const ())` | ❌ 桩 | |
| `equal(Object, Object)` | `equal(*const (), *const ())` | ❌ 桩 | |
| `notEqual(Object, Object)` | `notEqual(*const (), *const ())` | ❌ 桩 | |
| `length(Object)` | `length(*const ())` | ❌ 桩 | |
| `contains(Object, Object)` | `contains(*const (), *const ())` | ❌ 桩 | |
| `isNull(Object)` | `isNull(*const ())` | ❌ 桩 | |
| `isNotNull(Object)` | `isNotNull(*const ())` | ❌ 桩 | |
| `isEmpty(Object)` | `isEmpty(*const ())` | ❌ 桩 | |
| `isNotEmpty(Object)` | `isNotEmpty(*const ())` | ❌ 桩 | |
| `defaultIfNull(T, T)` | `defaultIfNull(T, T)` | ❌ 桩 | |
| `defaultIfNull(T, Supplier)` | `defaultIfNull_2(T, fn())` | ❌ 桩 | |
| `defaultIfEmpty(T, T)` | `defaultIfEmpty_3(T, T)` | ❌ 桩 | |
| `defaultIfBlank(T, T)` | `defaultIfBlank(T, T)` | ❌ 桩 | |
| `clone(T)` | `clone(T)` | ❌ 桩 | |
| `cloneIfPossible(T)` | `cloneIfPossible(T)` | ❌ 桩 | |
| `cloneByStream(T)` | `cloneByStream(T)` | ❌ 桩 | |
| `serialize(T)` | `serialize(T)` | ❌ 桩 | |
| `deserialize(byte[], Class...)` | `deserialize(Vec<i8>, &[Class])` | ❌ 桩 | |
| `isBasicType(Object)` | `isBasicType(*const ())` | ❌ 桩 | |
| `compare(T, T)` | `compare(T, T)` | ❌ 桩 | |
| `toString(Object)` | `toString(*const ())` | ❌ 桩 | |
| `emptyCount(Object...)` | `emptyCount(&[OPAQUE])` | ❌ 桩 | |
| `hasNull(Object...)` | `hasNull(&[OPAQUE])` | ❌ 桩 | |
| `hasEmpty(Object...)` | `hasEmpty(&[OPAQUE])` | ❌ 桩 | |
| `isAllEmpty(Object...)` | `isAllEmpty(&[OPAQUE])` | ❌ 桩 | |
| `isAllNotEmpty(Object...)` | `isAllNotEmpty(&[OPAQUE])` | ❌ 桩 | |
| ... | (共 41 个) | ❌ 桩 | |

---

## 3. ArrayUtil

**Java**: `cn.hutool.core.util.ArrayUtil` (99 方法，继承 PrimitiveArrayUtil ~80+ 方法)
**Rust**: `array_util.rs` (99 桩，**0 实现**)

### 全部桩函数

| Java 方法 | Rust 方法 | 状态 | 备注 |
|-----------|----------|------|------|
| `isEmpty(T[])` | `isEmpty(Vec<T>)` | ❌ 桩 | |
| `isNotEmpty(T[])` | `isNotEmpty(Vec<T>)` | ❌ 桩 | |
| `defaultIfEmpty(T[], T[])` | `defaultIfEmpty(Vec<T>, Vec<T>)` | ❌ 桩 | |
| `hasNull(T...)` | `hasNull(&[T])` | ❌ 桩 | |
| `contains(T[], T)` | `contains(Vec<T>, T)` | ❌ 桩 | |
| `containsAny(T[], T...)` | `containsAny(Vec<T>, &[T])` | ❌ 桩 | |
| `containsAll(T[], T...)` | `containsAll(Vec<T>, &[T])` | ❌ 桩 | |
| `indexOf(T[], Object)` | `indexOf(Vec<T>, *const ())` | ❌ 桩 | |
| `lastIndexOf(T[], Object)` | `lastIndexOf(Vec<T>, *const ())` | ❌ 桩 | |
| `append(T[], T...)` | `append(Vec<T>, &[T])` | ❌ 桩 | |
| `insert(T[], int, T...)` | `insert(Vec<T>, i32, &[T])` | ❌ 桩 | |
| `remove(T[], int)` | `remove(Vec<T>, i32)` | ❌ 桩 | |
| `removeEle(T[], T)` | `removeEle(Vec<T>, T)` | ❌ 桩 | |
| `reverse(T[])` | `reverse(Vec<T>)` | ❌ 桩 | |
| `min(T[])` | `min(Vec<T>)` | ❌ 桩 | |
| `max(T[])` | `max(Vec<T>)` | ❌ 桩 | |
| `shuffle(T[])` | `shuffle(Vec<T>)` | ❌ 桩 | |
| `swap(T[], int, int)` | `swap(Vec<T>, i32, i32)` | ❌ 桩 | |
| `distinct(T[])` | `distinct(Vec<T>)` | ❌ 桩 | |
| `filter(T[], Filter)` | `filter(Vec<T>, Filter)` | ❌ 桩 | |
| `map(T[], Function)` | `map(Vec<T>, fn(OPAQUE)->OPAQUE)` | ❌ 桩 | |
| `zip(K[], V[])` | `zip(Vec<T>, Vec<T>)` | ❌ 桩 | |
| `join(T[], CharSequence)` | `join(Vec<T>, *const ())` | ❌ 桩 | |
| `sub(T[], int, int)` | `sub(Vec<T>, i32, i32)` | ❌ 桩 | |
| `toString(Object)` | `toString(*const ())` | ❌ 桩 | |
| `length(Object)` | `length(*const ())` | ❌ 桩 | |
| `isArray(Object)` | `isArray(*const ())` | ❌ 桩 | |
| `equals(Object, Object)` | `equals(*const (), *const ())` | ❌ 桩 | |
| ... | (共 99 个) | ❌ 桩 | |

### PrimitiveArrayUtil 继承方法 (~80+)

Java ArrayUtil 继承 PrimitiveArrayUtil 的 80+ 个原始类型数组方法（如 `isEmpty(int[])`, `isEmpty(long[])`, `addAll(int[]...)` 等），Rust 未迁移。

---

## 4. DateUtil

**Java**: `cn.hutool.core.date.DateUtil` (~95 方法)
**Rust**: `date_util.rs` (~75 方法，~70 已实现)

### 已实现方法（~70 个）

| Java 方法 | Rust 方法 | 实现 | 备注 |
|-----------|----------|------|------|
| `date()` | `date()` | ✅ | |
| `current()` | `current()` | ✅ | |
| `now()` | `now()` | ✅ | |
| `today()` | `today()` | ✅ | |
| `year(Date)` | `year(DateTime)` | ✅ | Date→DateTime |
| `quarter(Date)` | `quarter(DateTime)` | ✅ | |
| `month(Date)` | `month(DateTime)` | ✅ | |
| `weekOfYear(Date)` | `week_of_year(DateTime)` | ✅ | |
| `dayOfMonth(Date)` | `day_of_month(DateTime)` | ✅ | |
| `dayOfWeek(Date)` | `day_of_week(DateTime)` | ✅ | |
| `hour(Date, boolean)` | `hour(DateTime, bool)` | ✅ | |
| `minute(Date)` | `minute(DateTime)` | ✅ | |
| `second(Date)` | `second(DateTime)` | ✅ | |
| `isWeekend(Date)` | `is_weekend(DateTime)` | ✅ | |
| `isAM(Date)` | `is_am(DateTime)` | ✅ | |
| `isPM(Date)` | `is_pm(DateTime)` | ✅ | |
| `format(Date, String)` | `format(DateTime, &str)` | ✅ | |
| `formatDateTime(Date)` | `format_datetime(DateTime)` | ✅ | |
| `formatDate(Date)` | `format_date(DateTime)` | ✅ | |
| `formatTime(Date)` | `format_time(DateTime)` | ✅ | |
| `parse(CharSequence, String)` | `parse_with_format(&str, &str)` | ✅ | |
| `parseDateTime(CharSequence)` | `parse_datetime(&str)` | ✅ | |
| `parseDate(CharSequence)` | `parse_date(&str)` | ✅ | |
| `parseUTC(String)` | `parse_utc(&str)` | ✅ | |
| `parseISO8601(String)` | `parse_iso8601(&str)` | ✅ | |
| `parse(CharSequence)` | `parse(&str)` | ✅ | 自动检测格式 |
| `truncate(Date, DateField)` | `truncate(DateTime, DateField)` | ✅ | |
| `beginOfDay(Date)` | `begin_of_day(DateTime)` | ✅ | |
| `endOfDay(Date)` | `end_of_day(DateTime)` | ✅ | |
| `beginOfMonth(Date)` | `begin_of_month(DateTime)` | ✅ | |
| `endOfMonth(Date)` | `end_of_month(DateTime)` | ✅ | |
| `beginOfYear(Date)` | `begin_of_year(DateTime)` | ✅ | |
| `endOfYear(Date)` | `end_of_year(DateTime)` | ✅ | |
| `yesterday()` | `yesterday()` | ✅ | |
| `tomorrow()` | `tomorrow()` | ✅ | |
| `offset(Date, DateField, int)` | `offset(DateTime, DateField, i64)` | ✅ | |
| `between(Date, Date, DateUnit)` | `between(DateTime, DateTime, DateUnit)` | ✅ | |
| `betweenMs(Date, Date)` | `between_ms(DateTime, DateTime)` | ✅ | |
| `betweenDay(Date, Date, boolean)` | `between_day(DateTime, DateTime, bool)` | ✅ | |
| `isSameDay(Date, Date)` | `is_same_day(DateTime, DateTime)` | ✅ | |
| `isSameMonth(Date, Date)` | `is_same_month(DateTime, DateTime)` | ✅ | |
| `range(Date, Date, DateField)` | `range(DateTime, DateTime, DateField)` | ✅ | |
| `age(Date, Date)` | `age(DateTime, DateTime)` | ✅ | |
| `isLeapYear(int)` | `is_leap_year(i32)` | ✅ | |
| `timer()` | `timer()` | ✅ | |
| `createStopWatch()` | `create_stop_watch()` | ✅ | |
| ... | (共 ~70 个) | ✅ | |

### 未迁移方法（~25 个）

| Java 方法 | 原因 |
|-----------|------|
| `date(Date)` / `date(Calendar)` / `date(TemporalAccessor)` | Rust 统一用 DateTime |
| `format(Date, DatePrinter)` / `format(Date, DateFormat)` | Java 特有格式化器 |
| `parse(CharSequence, DateFormat)` / `parse(CharSequence, DateParser)` | Java 特有解析器 |
| `parse(CharSequence, String, Locale)` | Locale 不适用 |
| `convertTimeZone(Date, ZoneId)` | 时区转换未迁移 |
| `newSimpleFormat(String)` | Java SimpleDateFormat |
| `getLastDayOfMonth(Date)` | 便捷方法遗漏 |
| `lengthOfYear(int)` | 便捷方法遗漏 |

---

## 5. NumberUtil

**Java**: `cn.hutool.core.util.NumberUtil` (~130 方法)
**Rust**: `number_util.rs` (~130 方法，~15 已实现)

### 已实现方法（~15 个）

| Java 方法 | Rust 方法 | 实现 | 备注 |
|-----------|----------|------|------|
| `add(double, double)` | `add_4(f64, f64)` | ✅ | ⚠️ 无 BigDecimal 精度 |
| `sub(double, double)` | `sub_4(f64, f64)` | ✅ | ⚠️ 无 BigDecimal 精度 |
| `mul(double, double)` | `mul_4(f64, f64)` | ✅ | ⚠️ 无 BigDecimal 精度 |
| `div(double, double)` | `div_4(f64, f64)` | ✅ | ⚠️ 无 BigDecimal 精度 |
| `compare(char, char)` | `compare(char, char)` | ✅ | |
| `compare(double, double)` | `compare_2(f64, f64)` | ✅ | |
| `compare(int, int)` | `compare_3(i32, i32)` | ✅ | |
| `compare(long, long)` | `compare_4(i64, i64)` | ✅ | |
| `equals(double, double)` | `equals(f64, f64)` | ✅ | epsilon 比较 |
| `equals(float, float)` | `equals_2(f32, f32)` | ✅ | epsilon 比较 |
| `equals(long, long)` | `equals_3(i64, i64)` | ✅ | |
| `min(long...)` | `min_2(&[i64])` | ✅ | iter().min() |
| `min(int...)` | `min_3(&[i32])` | ✅ | iter().min() |
| `max(long...)` | `max_2(&[i64])` | ✅ | iter().max() |
| `max(int...)` | `max_3(&[i32])` | ✅ | iter().max() |

### 桩方法（~115 个，全部 PendingEngine）

包括 add/sub/mul/div 的所有重载（12+9+10+22=53 个）、round 系列（8 个）、isXXX 系列（5 个）、generate 系列（3 个）、range 系列（3 个）、factorial 系列（4 个）、parseXXX 系列（10 个）、nullToZero 系列（8 个）等。

---

## 6. CollUtil

**Java**: `cn.hutool.core.collection.CollUtil` (~160 方法)
**Rust**: `coll_util.rs` (~60 方法，~45 已实现)

### 已实现方法（~45 个）

| Java 方法 | Rust 方法 | 实现 | 备注 |
|-----------|----------|------|------|
| `emptyIfNull(List)` | `empty_if_null(Option<&[T]>)` | ✅ | |
| `union(Collection, Collection)` | `union(&[T], &[T])` | ✅ | |
| `unionDistinct(Collection...)` | `union_distinct(&[&[T]])` | ✅ | |
| `intersection(Collection, Collection)` | `intersection(&[T], &[T])` | ✅ | |
| `disjunction(Collection, Collection)` | `disjunction(&[T], &[T])` | ✅ | |
| `subtract(Collection, Collection)` | `subtract(&[T], &[T])` | ✅ | |
| `contains(Collection, Object)` | `contains(&[T], &T)` | ✅ | |
| `containsAny(Collection, Collection)` | `contains_any(&[T], &[T])` | ✅ | |
| `containsAll(Collection, Collection)` | `contains_all(&[T], &[T])` | ✅ | |
| `newHashSet(T...)` | `new_hash_set(impl IntoIterator)` | ✅ | |
| `newArrayList(T...)` | `new_array_list(impl IntoIterator)` | ✅ | |
| `distinct(Collection)` | `distinct(impl IntoIterator)` | ✅ | |
| `filterNew(Collection, Filter)` | `filter_new(values, predicate)` | ✅ | |
| `filter(Collection, Filter)` | `filter(&mut Vec<T>, predicate)` | ✅ | 就地修改 |
| `edit(Collection, Editor)` | `edit(values, editor)` | ✅ | |
| `map(Iterable, Function, boolean)` | `map(values, mapper, ignore_null)` | ✅ | |
| `removeAny(Collection, E...)` | `remove_any(&mut Vec<T>, &[T])` | ✅ | |
| `removeNull(Collection)` | `remove_null(&mut Vec<Option<T>>)` | ✅ | |
| `sub(List, int, int, int)` | `sub(&[T], isize, isize, isize)` | ✅ | |
| `split(Collection, int)` | `split(&[T], usize)` | ✅ | |
| `page(int, int, List)` | `page(values, page_no, page_size)` | ✅ | |
| `sort(Collection, Comparator)` | `sort(values, compare)` | ✅ | |
| `countMap(Iterable)` | `count_map(impl IntoIterator)` | ✅ | |
| `join(Iterable, CharSequence)` | `join(values, delimiter)` | ✅ | |
| `anyMatch(Collection, Predicate)` | `any_match(&[T], matcher)` | ✅ | |
| `allMatch(Collection, Predicate)` | `all_match(&[T], matcher)` | ✅ | |
| `findOne(Iterable, Filter)` | `find_one(values, matcher)` | ✅ | |
| `indexOf(Collection, Matcher)` | `index_of(&[T], matcher)` | ✅ | |
| `get(Collection, int)` | `get(&[T], isize)` | ✅ | |
| `getFirst(Iterable)` | `get_first(&[T])` | ✅ | |
| `getLast(Collection)` | `get_last(&[T])` | ✅ | |
| `zip(Collection, Collection)` | `zip(keys, values)` | ✅ | |
| `addIfAbsent(Collection, Object)` | `add_if_absent(&mut Vec<T>, Option<T>)` | ✅ | |
| `isEmpty(Collection)` | `is_empty(Option<&[T]>)` | ✅ | |
| `isNotEmpty(Collection)` | `is_not_empty(Option<&[T]>)` | ✅ | |
| ... | (共 ~45 个) | ✅ | |

### 未迁移方法（~50+ 个）

主要包括：
- 反射相关：`getFieldValues`, `findOneByField`, `sortByProperty`, `groupByField`
- Java 特有类型：`Enumeration`, `Iterator`, `Stack`, `BlockingQueue` 适配器
- 已废弃方法：`splitList`, `getElementType`
- 拼音排序：`sortByPinyin`（需要拼音引擎）

---

## 7. MapUtil

**Java**: `cn.hutool.core.map.MapUtil` (~75 方法)
**Rust**: `map_util.rs` (~65 方法，~55 已实现)

### 已实现方法（~55 个）

| Java 方法 | Rust 方法 | 实现 | 备注 |
|-----------|----------|------|------|
| `isEmpty(Map)` | `is_empty(&HashMap)` | ✅ | |
| `isNotEmpty(Map)` | `is_not_empty(&HashMap)` | ✅ | |
| `emptyIfNull(Map)` | `empty_if_null(Option<HashMap>)` | ✅ | |
| `newHashMap()` | `new_hash_map()` | ✅ | |
| `newHashMap(int)` | `new_hash_map_sized(size)` | ✅ | |
| `of(K, V)` | `of_pair(key, value)` | ✅ | |
| `of(Object[])` | `of(&[(K,V)])` | ✅ | 元组切片 |
| `entry(K, V)` | `entry(key, value)` | ✅ | 返回元组 |
| `getStr(Map, Object)` | `get_str(map, key)` | ✅ | |
| `getInt(Map, Object)` | `get_int(map, key)` | ✅ | |
| `getLong(Map, Object)` | `get_long(map, key)` | ✅ | |
| `getDouble(Map, Object)` | `get_double(map, key)` | ✅ | |
| `getFloat(Map, Object)` | `get_float(map, key)` | ✅ | |
| `getBool(Map, Object)` | `get_bool(map, key)` | ✅ | |
| `getChar(Map, Object)` | `get_char(map, key)` | ✅ | |
| `getAny(Map, K...)` | `get_any(map, &[K])` | ✅ | |
| `removeAny(Map, K...)` | `remove_any(map, &[K])` | ✅ | |
| `renameKey(Map, K, K)` | `rename_key(map, old, new)` | ✅ | |
| `join(Map, String, String)` | `join(map, entry_delim, kv_delim)` | ✅ | |
| `filter(Map, Filter)` | `filter(map, predicate)` | ✅ | |
| `edit(Map, Editor)` | `edit(map, editor)` | ✅ | |
| `map(Map, BiFunction)` | `map_values(map, bi)` | ✅ | |
| `reverse(Map)` | `reverse(map)` | ✅ | |
| `inverse(Map)` | `inverse(map)` | ✅ | |
| `sort(Map)` | `sort(map)` | ✅ | 返回 BTreeMap |
| `sortByValue(Map, boolean)` | `sort_by_value(map, is_desc)` | ✅ | |
| `toListMap(Iterable)` | `to_list_map(&[HashMap])` | ✅ | |
| `toMapList(Map)` | `to_map_list(&HashMap<K, Vec<V>>)` | ✅ | |
| `grouping(Iterable<Entry>)` | `grouping(entries)` | ✅ | |
| `toCamelCaseMap(Map)` | `to_camel_case_map(&HashMap<String, V>)` | ✅ | |
| `valuesOfKeys(Map, Iterator)` | `values_of_keys(map, keys)` | ✅ | |
| `computeIfAbsent(Map, K, Function)` | `compute_if_absent(map, key, mapping)` | ✅ | |
| `partition(Map, int)` | `partition(Option<&HashMap>, i32)` | ✅ | |
| `flatten(Map)` | `flatten(&HashMap<K, NestedMapValue>)` | ✅ | |
| `empty()` | `empty_map()` | ✅ | |
| `builder()` | `builder()` | ✅ | |
| `wrap(Map)` | `wrap(map)` | ⚠️ | 直接透传 |
| `unmodifiable(Map)` | `unmodifiable(&HashMap)` | ⚠️ | 返回克隆快照 |
| `createProxy(Map)` | `create_proxy(map)` | ⚠️ | 直接透传 |
| ... | (共 ~55 个) | ✅ | |

### 未迁移方法（~10 个）

| Java 方法 | 原因 |
|-----------|------|
| `newTreeMap(Map, Comparator)` | Rust BTreeMap 用 Ord trait |
| `get(Map, Object, TypeReference)` | 泛型反射不适用 |
| `computeIfAbsentForJdk8(Map, K, Function)` | JDK 特有 |

---

## 差异分析

### 1. 类型擦除问题

Java 的 `Object` 参数在 Rust 中用 `*const ()` (原始指针) 或 `OPAQUE` 类型替代，导致：
- 无法安全调用
- 类型信息丢失
- 需要 FFI 层才能使用

### 2. 泛型映射差异

| Java 模式 | Rust 替代 | 影响 |
|-----------|----------|------|
| `<T extends Comparable>` | 无约束泛型 | 无法编译期保证可比较 |
| `<T, R>` 双类型参数 | 单类型 `T` | map 等方法丢失返回类型 |
| `Function<T, R>` | `fn(T) -> T` | 无法表达类型转换 |
| `Supplier<T>` | `fn() -> T` | 语义相同 |
| `Predicate<T>` | `FnMut(&T) -> bool` | 语义相同 |

### 3. Java 反射依赖

以下方法因依赖 Java 反射而无法迁移：
- ObjectUtil: `isBasicType`, `getTypeArgument`, `serialize/deserialize`
- ArrayUtil: `isEmpty(Object)`, `length(Object)`, `get(Object, int)`, `newArray(Class, int)`
- CollUtil: `getFieldValues`, `findOneByField`, `sortByProperty`, `groupByField`

### 4. BigDecimal 精度

Java NumberUtil 的 add/sub/mul/div 使用 BigDecimal 保证精度，Rust 版本使用原生 f64 运算，存在 IEEE 754 浮点精度问题。

### 5. 集合类型映射

| Java 类型 | Rust 替代 | 差异 |
|-----------|----------|------|
| `ArrayList<T>` | `Vec<T>` | 等价 |
| `LinkedList<T>` | `VecDeque<T>` | 近似 |
| `LinkedHashMap<K,V>` | `IndexMap<K,V>` | 近似 |
| `TreeMap<K,V>` | `BTreeMap<K,V>` | 等价 |
| `ConcurrentHashMap<K,V>` | `HashMap<K,V>` | 无并发保护 |
| `IdentityHashMap<K,V>` | `HashMap<K,V>` | 无身份相等 |
| `CopyOnWriteArrayList<T>` | `Arc<Vec<T>>` | 近似 |
| `HashSet<T>` | `HashSet<T>` | 等价 |
| `TreeSet<T>` | `BTreeSet<T>` | 等价 |
| `LinkedHashSet<T>` | `IndexSet<T>` | 近似 |

---

## 结论

### 整体完成度: ~40%

| 类别 | 状态 | 说明 |
|------|------|------|
| DateUtil | ✅ 74% | 核心日期操作全部实现 |
| MapUtil | ✅ 87% | Map 操作全面覆盖 |
| CollUtil | ⚠️ 38% | 集合操作部分实现，反射方法未迁移 |
| StrUtil | ⚠️ 9% | string.rs 有 25 个实现，str_util.rs 全桩 |
| NumberUtil | ⚠️ 12% | 基础运算已实现，大部分桩 |
| ObjectUtil | ❌ 0% | 全部桩函数 |
| ArrayUtil | ❌ 0% | 全部桩函数 |

### 优先实现建议

1. **ObjectUtil** — 实现 `isNull`, `isEmpty`, `equals`, `defaultIfNull`, `clone` 等核心方法
2. **ArrayUtil** — 实现 `isEmpty`, `contains`, `indexOf`, `reverse`, `sort`, `join` 等核心方法
3. **NumberUtil** — 补充 BigDecimal 精度运算，实现 `round`, `parseInt`, `formatPercent` 等
4. **StrUtil** — 将 str_util.rs 桩方法转为 string.rs 的真实实现
