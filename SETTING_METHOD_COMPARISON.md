# hutool-setting Setting 类方法级别对比

> 对比 Java `cn.hutool.setting.Setting` 与 Rust `hutool_setting::Setting`

## 一、构造方法

| Java 方法 | Rust 方法 | 状态 | 说明 |
|-----------|-----------|------|------|
| `Setting()` | `Setting::new()` | ✅ | 创建空配置 |
| `Setting.create()` | `Setting::create()` | ✅ | 工厂方法 |
| `Setting(String path)` | `Setting::from_path(path)` | ✅ | 加载文件 |
| `Setting(String path, Charset, boolean)` | `Setting::from_path_with_options(path, charset, use_variable)` | ✅ | 带选项加载 |

## 二、加载/存储

| Java 方法 | Rust 方法 | 状态 | 说明 |
|-----------|-----------|------|------|
| `load()` | `load(&mut self) -> Result<bool>` | ✅ | 原子重载 |
| `autoLoad(boolean, Consumer)` | `auto_load(callback) -> AutoLoadHandle` | ✅ | RAII 自动重载 |
| `store()` | `store(path)` | ✅ | 存储到文件 |
| `store(String)` | `store(path)` | ✅ | 存储到指定路径 |

## 三、查询方法

| Java 方法 | Rust 方法 | 状态 | 说明 |
|-----------|-----------|------|------|
| `getByGroup(key, group)` | `get_by_group(key, group) -> Option<String>` | ✅ | 按分组取值 |
| `get(key)` | `get(key) -> Option<String>` | ✅ | 取默认分组值 |
| `getAndRemove(keys)` | `get_and_remove(keys) -> Option<String>` | ✅ | 取并删除 |
| `getMap(group)` | `get_map(group) -> IndexMap` | ✅ | 取分组快照 |
| `getSetting(group)` | `get_setting(group) -> Setting` | ✅ | 取子 Setting |
| `getProps(group)` | `get_props(group) -> Props` | ✅ | 取 Props |
| `getGroupedMap()` | `grouped_map() -> GroupedMap` | ✅ | 取底层数据 |

## 四、修改方法

| Java 方法 | Rust 方法 | 状态 | 说明 |
|-----------|-----------|------|------|
| `set(key, value)` | `set(key, value) -> &Self` | ✅ | 设置默认分组值 |
| `put(key, value, group)` | `put_by_group(key, group, value)` | ✅ | 设置分组值 |
| `clear()` | `clear()` | ✅ | 清空 |

## 五、信息方法

| Java 方法 | Rust 方法 | 状态 | 说明 |
|-----------|-----------|------|------|
| `size()` | `size() -> usize` | ✅ | 条目数 |
| `isEmpty()` | `is_empty() -> bool` | ✅ | 是否为空 |
| `containsKey(group, key)` | `contains_key(group, key) -> bool` | ✅ | 包含键 |
| `containsValue(group, value)` | `contains_value(group, value) -> bool` | ✅ | 包含值 |
| `getGroups()` | `groups() -> Vec<String>` | ✅ | 所有分组 |

## 六、Rust 新增方法

| Rust 方法 | 说明 |
|-----------|------|
| `get_or(key, group, default)` | 取值或默认 |
| `get_not_empty_or(key, group, default)` | 取非空值或默认 |
| `get_parse(key, group)` | 解析类型化值 |
| `get_strings(key, group, delimiter)` | 分割字符串 |
| `to_properties(group)` | 转为 IndexMap 快照 |
| `set_var_regex(prefix, suffix)` | 设置变量分隔符 |
| `set_charset(charset)` | 设置编码 |
| `add_setting(other)` | 合并另一个 Setting |

## 七、总结

- **Java 有**: 30+ 个方法
- **Rust 有**: 25+ 个方法
- **映射率**: ~83%（核心方法全部覆盖）
- **命名一致性**: camelCase → snake_case 转换正确
- **逻辑一致性**: 核心逻辑一致，Rust 增加了 RAII 和 Result 类型
