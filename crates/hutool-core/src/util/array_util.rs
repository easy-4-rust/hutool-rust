//! 对齐: `cn.hutool.core.util.ArrayUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ArrayUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数;
//! Java 反射相关方法 (`*const ()` 签名) 保留为 `PendingEngine` 桩。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    non_snake_case
)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ArrayUtil`
/// 数组工具类
#[derive(Debug, Clone, Copy, Default)]
pub struct ArrayUtil;

impl ArrayUtil {
    // ── 空值判断 ──

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isEmpty#boolean (T[] array)`
    /// 判断数组是否为空
    pub fn isEmpty<T>(array: Vec<T>) -> Result<bool> {
        Ok(array.is_empty())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isEmpty#boolean (Object array)`
    pub fn isEmpty_2(_array: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isNotEmpty#boolean (T[] array)`
    /// 判断数组是否非空
    pub fn isNotEmpty<T>(array: Vec<T>) -> Result<bool> {
        Ok(!array.is_empty())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isNotEmpty#boolean (Object array)`
    pub fn isNotEmpty_2(_array: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isNotEmpty"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::defaultIfEmpty#T[] (T[] array, T[] defaultArray)`
    /// 如果数组为空则返回默认数组
    pub fn defaultIfEmpty<T: Clone>(array: Vec<T>, defaultArray: Vec<T>) -> Result<Vec<T>> {
        if array.is_empty() {
            Ok(defaultArray)
        } else {
            Ok(array)
        }
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::hasNull#boolean (T... array)`
    /// 判断数组中是否包含 null 元素
    pub fn hasNull<T>(array: &[Option<T>]) -> Result<bool> {
        Ok(array.iter().any(Option::is_none))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllNull#boolean (T... array)`
    /// 判断数组中是否所有元素都为 null
    pub fn isAllNull<T>(array: &[Option<T>]) -> Result<bool> {
        Ok(array.iter().all(Option::is_none))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::firstNonNull#T (T... array)`
    /// 返回数组中第一个非 null 的元素
    pub fn firstNonNull<T: Clone>(array: &[Option<T>]) -> Result<Option<T>> {
        Ok(array.iter().find_map(|v| v.clone()))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::firstMatch#T (Matcher<T> matcher, T... array)`
    pub fn firstMatch<T: Clone>(matcher: fn(&T) -> bool, array: &[T]) -> Result<Option<T>> {
        Ok(array.iter().find(|v| matcher(v)).cloned())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::matchIndex#int (Matcher<T> matcher, T... array)`
    pub fn matchIndex<T>(matcher: fn(&T) -> bool, array: &[T]) -> Result<i32> {
        Ok(array
            .iter()
            .position(|v| matcher(v))
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::matchIndex#int (Matcher<T> matcher, int beginIndexInclude, T... array)`
    pub fn matchIndex_2<T>(
        matcher: fn(&T) -> bool,
        beginIndexInclude: usize,
        array: &[T],
    ) -> Result<i32> {
        Ok(array
            .iter()
            .enumerate()
            .skip(beginIndexInclude)
            .find(|(_, v)| matcher(v))
            .map(|(idx, _)| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::newArray#T[] (Class<?> componentType, int newSize)`
    pub fn newArray<T: Default + Clone>(componentType: (), newSize: usize) -> Result<Vec<T>> {
        Ok(vec![T::default(); newSize])
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::newArray#Object[] (int newSize)`
    pub fn newArray_2(newSize: i32) -> Result<Vec<()>> {
        Err(CoreError::PendingEngine("newArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getComponentType#Class<?> (Object array)`
    pub fn getComponentType(_array: *const ()) -> Result<()> {
        Err(CoreError::PendingEngine("getComponentType"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getComponentType#Class<?> (Class<?> arrayClass)`
    pub fn getComponentType_2(arrayClass: ()) -> Result<()> {
        Err(CoreError::PendingEngine("getComponentType"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getArrayType#Class<?> (Class<?> componentType)`
    pub fn getArrayType(componentType: ()) -> Result<()> {
        Err(CoreError::PendingEngine("getArrayType"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::cast#Object[] (Class<?> type, Object arrayObj)`
    pub fn cast<T: Clone>(type_: (), arrayObj: &[T]) -> Result<Vec<T>> {
        Ok(arrayObj.to_vec())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::append#T[] (T[] buffer, T... newElements)`
    /// 将新元素追加到数组末尾
    pub fn append<T: Clone>(buffer: Vec<T>, newElements: &[T]) -> Result<Vec<T>> {
        let mut result = buffer;
        result.extend_from_slice(newElements);
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::append#Object (Object array, T... newElements)`
    pub fn append_2(_array: *const (), newElements: &[()]) -> Result<()> {
        Err(CoreError::PendingEngine("append"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::setOrAppend#T[] (T[] buffer, int index, T value)`
    /// 设置指定索引的值,如果索引超出范围则追加
    pub fn setOrAppend<T>(mut buffer: Vec<T>, index: usize, value: T) -> Result<Vec<T>> {
        if index < buffer.len() {
            buffer[index] = value;
        } else {
            buffer.push(value);
        }
        Ok(buffer)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::setOrAppend#Object (Object array, int index, Object value)`
    pub fn setOrAppend_2(_array: *const (), index: i32, _value: *const ()) -> Result<()> {
        Err(CoreError::PendingEngine("setOrAppend"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::replace#T[] (T[] buffer, int index, T... values)`
    /// 替换指定位置的元素
    pub fn replace<T: Clone>(buffer: Vec<T>, index: isize, values: &[T]) -> Result<Vec<T>> {
        if values.is_empty() {
            return Ok(buffer);
        }
        if buffer.is_empty() {
            return Ok(values.to_vec());
        }
        if index < 0 {
            let mut result = Vec::with_capacity(values.len() + buffer.len());
            result.extend_from_slice(values);
            result.extend_from_slice(&buffer);
            return Ok(result);
        }
        let index = index as usize;
        if index >= buffer.len() {
            let mut result = buffer;
            result.extend_from_slice(values);
            return Ok(result);
        }
        if buffer.len() >= values.len() + index {
            let mut result = buffer;
            result[index..index + values.len()].clone_from_slice(values);
            return Ok(result);
        }
        let mut result = Vec::with_capacity(index + values.len());
        result.extend_from_slice(&buffer[..index]);
        result.extend_from_slice(values);
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::insert#T[] (T[] buffer, int index, T... newElements)`
    /// 在指定位置插入新元素
    pub fn insert<T: Clone>(buffer: Vec<T>, index: usize, newElements: &[T]) -> Result<Vec<T>> {
        let mut result = Vec::with_capacity(buffer.len() + newElements.len());
        let idx = index.min(buffer.len());
        result.extend_from_slice(&buffer[..idx]);
        result.extend_from_slice(newElements);
        result.extend_from_slice(&buffer[idx..]);
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::insert#Object (Object array, int index, T... newElements)`
    pub fn insert_2(_array: *const (), index: i32, newElements: &[()]) -> Result<()> {
        Err(CoreError::PendingEngine("insert"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::resize#T[] (T[] data, int newSize, Class<?> componentType)`
    pub fn resize<T: Clone + Default>(
        data: Vec<T>,
        newSize: usize,
        componentType: (),
    ) -> Result<Vec<T>> {
        let mut out = data;
        out.resize(newSize, T::default());
        Ok(out)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::resize#Object (Object array, int newSize)`
    pub fn resize_2(_array: *const (), newSize: i32) -> Result<()> {
        Err(CoreError::PendingEngine("resize"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::resize#T[] (T[] buffer, int newSize)`
    pub fn resize_3<T: Clone + Default>(buffer: Vec<T>, newSize: usize) -> Result<Vec<T>> {
        let mut out = buffer;
        out.resize(newSize, T::default());
        Ok(out)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::addAll#T[] (T[]... arrays)`
    /// 合并多个数组为一个
    pub fn addAll<T: Clone>(arrays: &[Vec<T>]) -> Result<Vec<T>> {
        let total_len: usize = arrays.iter().map(|a| a.len()).sum();
        let mut result = Vec::with_capacity(total_len);
        for array in arrays {
            result.extend_from_slice(array);
        }
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::copy#Object (Object src, int srcPos, Object dest, int destPos, int length)`
    pub fn copy<T: Clone>(
        src: &[T],
        srcPos: usize,
        dest: &mut [T],
        destPos: usize,
        length: usize,
    ) -> Result<()> {
        if srcPos >= src.len() || destPos >= dest.len() {
            return Ok(());
        }
        let n = length
            .min(src.len().saturating_sub(srcPos))
            .min(dest.len().saturating_sub(destPos));
        dest[destPos..destPos + n].clone_from_slice(&src[srcPos..srcPos + n]);
        Ok(())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::copy#Object (Object src, Object dest, int length)`
    pub fn copy_2<T: Clone>(src: &[T], dest: &mut [T], length: usize) -> Result<()> {
        Self::copy(src, 0, dest, 0, length)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::clone#T[] (T[] array)`
    /// 克隆数组
    pub fn clone<T: Clone>(array: Vec<T>) -> Result<Vec<T>> {
        Ok(array)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::clone#T (final T obj)`
    pub fn clone_2<T: Clone>(obj: T) -> Result<T> {
        Ok(obj)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::edit#T[] (T[] array, Editor<T> editor)`
    pub fn edit<T: Clone>(array: Vec<T>, editor: fn(T) -> Option<T>) -> Result<Vec<T>> {
        Ok(array.into_iter().filter_map(|v| editor(v)).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::filter#T[] (T[] array, Filter<T> filter)`
    /// 过滤数组,保留满足条件的元素
    pub fn filter<T: Clone>(array: Vec<T>, filter_fn: fn(&T) -> bool) -> Result<Vec<T>> {
        Ok(array.into_iter().filter(|v| filter_fn(v)).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeNull#T[] (T[] array)`
    /// 移除数组中的 null 元素
    pub fn removeNull<T: Clone>(array: Vec<Option<T>>) -> Result<Vec<T>> {
        Ok(array.into_iter().filter_map(|v| v).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeEmpty#T[] (T[] array)`
    pub fn removeEmpty<T: Clone + AsRef<str>>(array: Vec<Option<T>>) -> Result<Vec<T>> {
        Ok(array
            .into_iter()
            .filter_map(|v| v)
            .filter(|s| !s.as_ref().is_empty())
            .collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeBlank#T[] (T[] array)`
    pub fn removeBlank<T: Clone + AsRef<str>>(array: Vec<Option<T>>) -> Result<Vec<T>> {
        Ok(array
            .into_iter()
            .filter_map(|v| v)
            .filter(|s| !s.as_ref().trim().is_empty())
            .collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::nullToEmpty#String[] (String[] array)`
    pub fn nullToEmpty(array: Vec<Option<String>>) -> Result<Vec<String>> {
        Ok(array
            .into_iter()
            .map(|item| item.unwrap_or_default())
            .collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::zip#Map<K, V> (K[] keys, V[] values, boolean isOrder)`
    /// 将两个数组合并为 Map
    pub fn zip<K, V>(keys: Vec<K>, values: Vec<V>, _isOrder: bool) -> Result<HashMap<K, V>>
    where
        K: Eq + std::hash::Hash,
    {
        Ok(keys.into_iter().zip(values.into_iter()).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::zip#Map<K, V> (K[] keys, V[] values)`
    pub fn zip_2<K, V>(keys: Vec<K>, values: Vec<V>) -> Result<HashMap<K, V>>
    where
        K: Eq + std::hash::Hash,
    {
        Self::zip(keys, values, false)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOf#int (T[] array, Object value, int beginIndexInclude)`
    /// 查找元素在数组中的位置
    pub fn indexOf<T: PartialEq>(
        array: Vec<T>,
        value: &T,
        beginIndexInclude: usize,
    ) -> Result<i32> {
        Ok(array
            .iter()
            .enumerate()
            .skip(beginIndexInclude)
            .find(|(_, v)| *v == value)
            .map(|(idx, _)| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOf#int (T[] array, Object value)`
    pub fn indexOf_2<T: PartialEq>(array: Vec<T>, value: &T) -> Result<i32> {
        Ok(array
            .iter()
            .position(|v| v == value)
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOfIgnoreCase#int (CharSequence[] array, CharSequence value)`
    pub fn indexOfIgnoreCase(array: Vec<String>, value: &str) -> Result<i32> {
        let value_lower = value.to_lowercase();
        Ok(array
            .iter()
            .position(|v| v.to_lowercase() == value_lower)
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOf#int (T[] array, Object value)`
    /// 查找元素在数组中最后出现的位置
    pub fn lastIndexOf<T: PartialEq>(array: Vec<T>, value: &T) -> Result<i32> {
        Ok(array
            .iter()
            .rposition(|v| v == value)
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOf#int (T[] array, Object value, int endInclude)`
    pub fn lastIndexOf_2<T: PartialEq>(array: Vec<T>, value: &T, endInclude: usize) -> Result<i32> {
        let end = endInclude.min(array.len().saturating_sub(1));
        Ok(array[..=end]
            .iter()
            .rposition(|v| v == value)
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::contains#boolean (T[] array, T value)`
    /// 判断数组是否包含指定元素
    pub fn contains<T: PartialEq>(array: Vec<T>, value: &T) -> Result<bool> {
        Ok(array.contains(value))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::containsAny#boolean (T[] array, T... values)`
    /// 判断数组是否包含任意一个指定元素
    pub fn containsAny<T: PartialEq>(array: Vec<T>, values: &[T]) -> Result<bool> {
        Ok(values.iter().any(|v| array.contains(v)))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::containsAll#boolean (T[] array, T... values)`
    /// 判断数组是否包含所有指定元素
    pub fn containsAll<T: PartialEq>(array: Vec<T>, values: &[T]) -> Result<bool> {
        Ok(values.iter().all(|v| array.contains(v)))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::containsIgnoreCase#boolean (CharSequence[] array, CharSequence value)`
    pub fn containsIgnoreCase(array: Vec<String>, value: &str) -> Result<bool> {
        let value_lower = value.to_lowercase();
        Ok(array.iter().any(|v| v.to_lowercase() == value_lower))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::wrap#Object[] (Object obj)`
    pub fn wrap<T>(obj: T) -> Result<Vec<T>> {
        Ok(vec![obj])
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isArray#boolean (Object obj)`
    pub fn isArray(_obj: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::get#T (Object array, int index)`
    pub fn get<T: Clone>(array: &[T], index: isize) -> Result<Option<T>> {
        let len = array.len() as isize;
        let resolved = if index < 0 { len + index } else { index };
        Ok(usize::try_from(resolved)
            .ok()
            .and_then(|idx| array.get(idx))
            .cloned())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::getAny#T[] (Object array, int... indexes)`
    pub fn getAny<T: Clone>(array: &[T], indexes: &[isize]) -> Result<Vec<T>> {
        Ok(indexes
            .iter()
            .filter_map(|&index| Self::get(array, index).ok().flatten())
            .collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::sub#T[] (T[] array, int start, int end)`
    /// 截取子数组
    pub fn sub<T: Clone>(array: Vec<T>, start: usize, end: usize) -> Result<Vec<T>> {
        let start = start.min(array.len());
        let end = end.min(array.len());
        if start >= end {
            return Ok(Vec::new());
        }
        Ok(array[start..end].to_vec())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::sub#Object[] (Object array, int start, int end)`
    pub fn sub_2(_array: *const (), start: i32, end: i32) -> Result<Vec<()>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::sub#Object[] (Object array, int start, int end, int step)`
    pub fn sub_3(_array: *const (), start: i32, end: i32, step: i32) -> Result<Vec<()>> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toString#String (Object obj)`
    pub fn toString(_obj: *const ()) -> Result<String> {
        Err(CoreError::PendingEngine("toString"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::length#int (Object array)`
    pub fn length<T>(array: &[T]) -> Result<usize> {
        Ok(array.len())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (T[] array, CharSequence conjunction)`
    /// 将数组元素连接为字符串
    pub fn join<T: std::fmt::Display>(array: Vec<T>, delimiter: &str) -> Result<String> {
        let parts: Vec<String> = array.iter().map(|x| x.to_string()).collect();
        Ok(parts.join(delimiter))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (T[] array, CharSequence delimiter, String prefix, String suffix)`
    pub fn join_2<T: std::fmt::Display>(
        array: Vec<T>,
        delimiter: &str,
        prefix: &str,
        suffix: &str,
    ) -> Result<String> {
        let inner = Self::join(array, delimiter)?;
        Ok(format!("{prefix}{inner}{suffix}"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (T[] array, CharSequence conjunction, Editor<T> editor)`
    pub fn join_3<T: std::fmt::Display>(
        array: Vec<T>,
        delimiter: &str,
        editor: fn(&T) -> String,
    ) -> Result<String> {
        let parts: Vec<String> = array.iter().map(|x| editor(x)).collect();
        Ok(parts.join(delimiter))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::join#String (Object array, CharSequence conjunction)`
    pub fn join_4(_array: *const (), _conjunction: *const ()) -> Result<String> {
        Err(CoreError::PendingEngine("join"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#byte[] (ByteBuffer bytebuffer)`
    pub fn toArray(_bytebuffer: *const ()) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("toArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#T[] (Iterator<T> iterator, Class<T> componentType)`
    pub fn toArray_2(_componentType: ()) -> Result<Vec<()>> {
        Err(CoreError::PendingEngine("toArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#T[] (Iterable<T> iterable, Class<T> componentType)`
    pub fn toArray_3<T>(iterable: Vec<T>, _componentType: ()) -> Result<Vec<T>> {
        Ok(iterable)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::toArray#T[] (Collection<T> collection, Class<T> componentType)`
    pub fn toArray_4<T>(collection: Vec<T>, _componentType: ()) -> Result<Vec<T>> {
        Ok(collection)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::remove#T[] (T[] array, int index)`
    /// 移除指定索引的元素
    pub fn remove<T: Clone>(array: Vec<T>, index: usize) -> Result<Vec<T>> {
        let mut result = array;
        if index < result.len() {
            result.remove(index);
        }
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::removeEle#T[] (T[] array, T element)`
    /// 移除数组中第一个匹配的元素
    pub fn removeEle<T: Clone + PartialEq>(array: Vec<T>, element: &T) -> Result<Vec<T>> {
        let mut result = Vec::with_capacity(array.len());
        let mut removed = false;
        for item in array {
            if !removed && item == *element {
                removed = true;
                continue;
            }
            result.push(item);
        }
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::reverse#T[] (T[] array, final int startIndexInclusive, final int endIndexExclusive)`
    /// 反转指定范围内的元素
    pub fn reverse<T>(
        mut array: Vec<T>,
        startIndexInclusive: usize,
        endIndexExclusive: usize,
    ) -> Result<Vec<T>> {
        let end = endIndexExclusive.min(array.len());
        let start = startIndexInclusive.min(end);
        if start < end {
            array[start..end].reverse();
        }
        Ok(array)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::reverse#T[] (T[] array)`
    /// 反转整个数组
    pub fn reverse_2<T>(mut array: Vec<T>) -> Result<Vec<T>> {
        array.reverse();
        Ok(array)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::min#T (T[] numberArray)`
    /// 返回数组中的最小值
    pub fn min<T: Ord + Clone>(numberArray: Vec<T>) -> Result<Option<T>> {
        Ok(numberArray.iter().min().cloned())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::min#T (T[] numberArray, Comparator<T> comparator)`
    pub fn min_2<T: Clone>(
        numberArray: Vec<T>,
        comparator: fn(&T, &T) -> Ordering,
    ) -> Result<Option<T>> {
        Ok(numberArray.iter().min_by(|a, b| comparator(a, b)).cloned())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::max#T (T[] numberArray)`
    /// 返回数组中的最大值
    pub fn max<T: Ord + Clone>(numberArray: Vec<T>) -> Result<Option<T>> {
        Ok(numberArray.iter().max().cloned())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::max#T (T[] numberArray, Comparator<T> comparator)`
    pub fn max_2<T: Clone>(
        numberArray: Vec<T>,
        comparator: fn(&T, &T) -> Ordering,
    ) -> Result<Option<T>> {
        Ok(numberArray.iter().max_by(|a, b| comparator(a, b)).cloned())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::shuffle#T[] (T[] array)`
    /// 随机打乱数组
    pub fn shuffle<T>(mut array: Vec<T>) -> Result<Vec<T>> {
        use rand::seq::SliceRandom;
        array.shuffle(&mut rand::thread_rng());
        Ok(array)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::shuffle#T[] (T[] array, Random random)`
    pub fn shuffle_2<T>(array: Vec<T>, _random: *const ()) -> Result<Vec<T>> {
        Self::shuffle(array)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::swap#T[] (T[] array, int index1, int index2)`
    /// 交换数组中两个位置的元素
    pub fn swap<T>(mut array: Vec<T>, index1: usize, index2: usize) -> Result<Vec<T>> {
        if index1 < array.len() && index2 < array.len() {
            array.swap(index1, index2);
        }
        Ok(array)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::swap#Object (Object array, int index1, int index2)`
    pub fn swap_2(_array: *const (), index1: i32, index2: i32) -> Result<()> {
        Err(CoreError::PendingEngine("swap"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::emptyCount#int (Object... args)`
    pub fn emptyCount(args: &[Option<&str>]) -> Result<i32> {
        Ok(args
            .iter()
            .filter(|v| v.map_or(true, |s| s.is_empty()))
            .count() as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::hasEmpty#boolean (Object... args)`
    pub fn hasEmpty(args: &[Option<&str>]) -> Result<bool> {
        Ok(args.iter().any(|v| v.map_or(true, |s| s.is_empty())))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllEmpty#boolean (Object... args)`
    pub fn isAllEmpty(args: &[Option<&str>]) -> Result<bool> {
        Ok(args.iter().all(|v| v.map_or(true, |s| s.is_empty())))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllNotEmpty#boolean (Object... args)`
    pub fn isAllNotEmpty(args: &[Option<&str>]) -> Result<bool> {
        Ok(args.iter().all(|v| v.map_or(false, |s| !s.is_empty())))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isAllNotNull#boolean (T... array)`
    /// 判断数组中是否所有元素都不为 null
    pub fn isAllNotNull<T>(array: &[Option<T>]) -> Result<bool> {
        Ok(array.iter().all(Option::is_some))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::distinct#T[] (T[] array)`
    /// 去重,保持原始顺序
    pub fn distinct<T: Eq + std::hash::Hash + Clone>(array: Vec<T>) -> Result<Vec<T>> {
        let mut seen = HashSet::new();
        Ok(array
            .into_iter()
            .filter(|item| seen.insert(item.clone()))
            .collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::distinct#T[] (T[] array, Function<T, K> uniqueGenerator, boolean override)`
    pub fn distinct_2<T: Clone, K: Eq + std::hash::Hash>(
        array: Vec<T>,
        uniqueGenerator: fn(&T) -> K,
        override_: bool,
    ) -> Result<Vec<T>> {
        if override_ {
            let mut order = Vec::new();
            let mut map: HashMap<K, T> = HashMap::new();
            for item in array {
                let key = uniqueGenerator(&item);
                if !map.contains_key(&key) {
                    order.push(key.clone());
                }
                map.insert(key, item);
            }
            Ok(order.into_iter().filter_map(|k| map.remove(&k)).collect())
        } else {
            let mut seen = HashSet::new();
            Ok(array
                .into_iter()
                .filter(|item| seen.insert(uniqueGenerator(item)))
                .collect())
        }
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::map#R[] (T[] array, Class<R> targetComponentType, Function<? super T, ? extends R> func)`
    pub fn map<T, R>(array: Vec<T>, _targetComponentType: (), func: fn(T) -> R) -> Result<Vec<R>> {
        Ok(array.into_iter().map(func).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::map#R[] (Object array, Class<R> targetComponentType, Function<? super T, ? extends R> func)`
    pub fn map_2(
        _array: *const (),
        _targetComponentType: (),
        _func: fn(()) -> (),
    ) -> Result<Vec<()>> {
        Err(CoreError::PendingEngine("map"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::map#List<R> (T[] array, Function<? super T, ? extends R> func)`
    pub fn map_3<T, R>(array: Vec<T>, func: fn(T) -> R) -> Result<Vec<R>> {
        Ok(array.into_iter().map(func).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::mapToSet#Set<R> (T[] array, Function<? super T, ? extends R> func)`
    pub fn mapToSet<T, R: Eq + std::hash::Hash>(
        array: Vec<T>,
        func: fn(T) -> R,
    ) -> Result<HashSet<R>> {
        Ok(array.into_iter().map(func).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::equals#boolean (Object array1, Object array2)`
    /// 判断两个数组是否相等
    pub fn equals<T: PartialEq>(array1: &[T], array2: &[T]) -> Result<bool> {
        Ok(array1 == array2)
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSub#boolean (T[] array, T[] subArray)`
    /// 判断 sub 是否为 array 的子序列
    pub fn isSub<T: PartialEq>(array: Vec<T>, subArray: Vec<T>) -> Result<bool> {
        if subArray.is_empty() {
            return Ok(true);
        }
        if subArray.len() > array.len() {
            return Ok(false);
        }
        Ok(array
            .windows(subArray.len())
            .any(|w| w == subArray.as_slice()))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOfSub#int (T[] array, T[] subArray)`
    /// 查找子数组在数组中的位置
    pub fn indexOfSub<T: PartialEq>(array: Vec<T>, subArray: Vec<T>) -> Result<i32> {
        if subArray.is_empty() {
            return Ok(-1);
        }
        if array.len() < subArray.len() {
            return Ok(-1);
        }
        Ok(array
            .windows(subArray.len())
            .position(|window| window == subArray.as_slice())
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::indexOfSub#int (T[] array, int beginInclude, T[] subArray)`
    pub fn indexOfSub_2<T: PartialEq>(
        array: Vec<T>,
        beginInclude: usize,
        subArray: Vec<T>,
    ) -> Result<i32> {
        if subArray.is_empty() || beginInclude >= array.len() {
            return Ok(-1);
        }
        let search_slice = &array[beginInclude..];
        Ok(search_slice
            .windows(subArray.len())
            .position(|window| window == subArray.as_slice())
            .map(|idx| (idx + beginInclude) as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOfSub#int (T[] array, T[] subArray)`
    /// 查找子数组在数组中最后出现的位置
    pub fn lastIndexOfSub<T: PartialEq>(array: Vec<T>, subArray: Vec<T>) -> Result<i32> {
        if subArray.is_empty() {
            return Ok(-1);
        }
        if array.len() < subArray.len() {
            return Ok(-1);
        }
        Ok(array
            .windows(subArray.len())
            .rposition(|window| window == subArray.as_slice())
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::lastIndexOfSub#int (T[] array, int endInclude, T[] subArray)`
    pub fn lastIndexOfSub_2<T: PartialEq>(
        array: Vec<T>,
        endInclude: usize,
        subArray: Vec<T>,
    ) -> Result<i32> {
        if subArray.is_empty() || endInclude >= array.len() {
            return Ok(-1);
        }
        let search_slice = &array[..=endInclude];
        Ok(search_slice
            .windows(subArray.len())
            .rposition(|window| window == subArray.as_slice())
            .map(|idx| idx as i32)
            .unwrap_or(-1))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSorted#boolean (T[] array, Comparator<? super T> comparator)`
    pub fn isSorted<T>(array: Vec<T>, comparator: fn(&T, &T) -> Ordering) -> Result<bool> {
        Ok(array
            .windows(2)
            .all(|w| comparator(&w[0], &w[1]) != Ordering::Greater))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSorted#boolean (T[] array)`
    pub fn isSorted_2<T: Ord>(array: Vec<T>) -> Result<bool> {
        Ok(array.windows(2).all(|w| w[0] <= w[1]))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSortedASC#boolean (T[] array)`
    /// 判断数组是否升序排列
    pub fn isSortedASC<T: Ord>(array: Vec<T>) -> Result<bool> {
        Ok(array.windows(2).all(|w| w[0] <= w[1]))
    }

    /// 对齐 Java: `cn.hutool.core.util::ArrayUtil::isSortedDESC#boolean (T[] array)`
    /// 判断数组是否降序排列
    pub fn isSortedDESC<T: Ord>(array: Vec<T>) -> Result<bool> {
        Ok(array.windows(2).all(|w| w[0] >= w[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isEmpty() {
        assert!(ArrayUtil::isEmpty(Vec::<i32>::new()).unwrap());
        assert!(!ArrayUtil::isEmpty(vec![1, 2, 3]).unwrap());
    }

    #[test]
    fn test_isNotEmpty() {
        assert!(!ArrayUtil::isNotEmpty(Vec::<i32>::new()).unwrap());
        assert!(ArrayUtil::isNotEmpty(vec![1, 2, 3]).unwrap());
    }

    #[test]
    fn test_defaultIfEmpty() {
        assert_eq!(
            ArrayUtil::defaultIfEmpty(Vec::<i32>::new(), vec![4, 5]).unwrap(),
            vec![4, 5]
        );
        assert_eq!(
            ArrayUtil::defaultIfEmpty(vec![1, 2, 3], vec![4, 5]).unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_hasNull() {
        assert!(ArrayUtil::hasNull(&[Some(1), None, Some(3)]).unwrap());
        assert!(!ArrayUtil::hasNull(&[Some(1), Some(2)]).unwrap());
    }

    #[test]
    fn test_contains() {
        assert!(ArrayUtil::contains(vec![1, 2, 3], &2).unwrap());
        assert!(!ArrayUtil::contains(vec![1, 2, 3], &5).unwrap());
    }

    #[test]
    fn test_containsAny() {
        assert!(ArrayUtil::containsAny(vec![1, 2, 3], &[2, 5]).unwrap());
        assert!(!ArrayUtil::containsAny(vec![1, 2, 3], &[4, 5]).unwrap());
    }

    #[test]
    fn test_containsAll() {
        assert!(ArrayUtil::containsAll(vec![1, 2, 3], &[1, 2]).unwrap());
        assert!(!ArrayUtil::containsAll(vec![1, 2, 3], &[1, 4]).unwrap());
    }

    #[test]
    fn test_indexOf() {
        assert_eq!(ArrayUtil::indexOf(vec![1, 2, 3, 2], &2, 0).unwrap(), 1);
        assert_eq!(ArrayUtil::indexOf(vec![1, 2, 3, 2], &2, 2).unwrap(), 3);
        assert_eq!(ArrayUtil::indexOf_2(vec![1, 2, 3], &5).unwrap(), -1);
    }

    #[test]
    fn test_lastIndexOf() {
        assert_eq!(ArrayUtil::lastIndexOf(vec![1, 2, 3, 2], &2).unwrap(), 3);
        assert_eq!(ArrayUtil::lastIndexOf(vec![1, 2, 3], &5).unwrap(), -1);
    }

    #[test]
    fn test_append() {
        assert_eq!(
            ArrayUtil::append(vec![1, 2], &[3, 4]).unwrap(),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_insert() {
        assert_eq!(
            ArrayUtil::insert(vec![1, 2, 3], 1, &[10, 20]).unwrap(),
            vec![1, 10, 20, 2, 3]
        );
    }

    #[test]
    fn test_remove() {
        assert_eq!(
            ArrayUtil::remove(vec![1, 2, 3, 4], 2).unwrap(),
            vec![1, 2, 4]
        );
    }

    #[test]
    fn test_removeEle() {
        assert_eq!(
            ArrayUtil::removeEle(vec![1, 2, 3, 2, 1], &2).unwrap(),
            vec![1, 3, 2, 1]
        );
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            ArrayUtil::reverse_2(vec![1, 2, 3, 4, 5]).unwrap(),
            vec![5, 4, 3, 2, 1]
        );
        assert_eq!(
            ArrayUtil::reverse(vec![1, 2, 3, 4, 5], 1, 4).unwrap(),
            vec![1, 4, 3, 2, 5]
        );
    }

    #[test]
    fn test_min_max() {
        assert_eq!(ArrayUtil::min(vec![3, 1, 4, 1, 5]).unwrap(), Some(1));
        assert_eq!(ArrayUtil::max(vec![3, 1, 4, 1, 5]).unwrap(), Some(5));
    }

    #[test]
    fn test_swap() {
        assert_eq!(ArrayUtil::swap(vec![1, 2, 3], 0, 2).unwrap(), vec![3, 2, 1]);
    }

    #[test]
    fn test_distinct() {
        assert_eq!(
            ArrayUtil::distinct(vec!["aa", "bb", "cc", "bb", "dd"]).unwrap(),
            vec!["aa", "bb", "cc", "dd"]
        );
    }

    #[test]
    fn test_filter() {
        assert_eq!(
            ArrayUtil::filter(vec![1, 2, 3, 4, 5, 6], |x| x % 2 == 0).unwrap(),
            vec![2, 4, 6]
        );
    }

    #[test]
    fn test_map() {
        assert_eq!(
            ArrayUtil::map_3(vec![1, 2, 3], |x| x * 10).unwrap(),
            vec![10, 20, 30]
        );
    }

    #[test]
    fn test_zip() {
        let map = ArrayUtil::zip_2(vec!["a", "b", "c"], vec![1, 2, 3]).unwrap();
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));
    }

    #[test]
    fn test_join() {
        assert_eq!(ArrayUtil::join(vec![1, 2, 3], ", ").unwrap(), "1, 2, 3");
        assert_eq!(
            ArrayUtil::join_2(vec![1, 2, 3], ", ", "[", "]").unwrap(),
            "[1, 2, 3]"
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            ArrayUtil::sub(vec![1, 2, 3, 4, 5], 1, 3).unwrap(),
            vec![2, 3]
        );
    }

    #[test]
    fn test_clone() {
        let arr = vec![1, 2, 3];
        let cloned = ArrayUtil::clone(arr.clone()).unwrap();
        assert_eq!(arr, cloned);
    }

    #[test]
    fn test_equals() {
        assert!(ArrayUtil::equals(&[1, 2, 3], &[1, 2, 3]).unwrap());
        assert!(!ArrayUtil::equals(&[1, 2, 3], &[1, 2, 4]).unwrap());
    }

    #[test]
    fn test_isSub() {
        assert!(ArrayUtil::isSub(vec![1, 2, 3, 4], vec![2, 3]).unwrap());
        assert!(!ArrayUtil::isSub(vec![1, 2, 3, 4], vec![2, 4]).unwrap());
    }

    #[test]
    fn test_removeNull() {
        assert_eq!(
            ArrayUtil::removeNull(vec![Some(1), None, Some(3), None]).unwrap(),
            vec![1, 3]
        );
    }

    #[test]
    fn test_indexOfSub() {
        assert_eq!(
            ArrayUtil::indexOfSub(vec![1, 2, 3, 4, 5], vec![3, 4]).unwrap(),
            2
        );
        assert_eq!(
            ArrayUtil::lastIndexOfSub(vec![1, 2, 3, 2, 1], vec![2, 1]).unwrap(),
            3
        );
    }

    #[test]
    fn test_isSorted() {
        assert!(ArrayUtil::isSortedASC(vec![1, 2, 3, 4]).unwrap());
        assert!(ArrayUtil::isSortedDESC(vec![4, 3, 2, 1]).unwrap());
        assert!(!ArrayUtil::isSortedASC(vec![1, 3, 2]).unwrap());
    }
}
