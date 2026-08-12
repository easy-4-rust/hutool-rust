# XML 支持设计规格

日期：2026-08-12
来源：重定位 `docs/xml.md`

## 1. 背景

`hutool-core` 使用 `quick-xml 0.41.0` 实现 XML。吞吐和分配结果取决于文档形状、输入源、启用 feature 和目标机器。

## 2. 架构

API 有三个刻意分层：

| 层 | API | 内存模型 | 用例 |
|---|---|---|---|
| 流式 | XmlEventReader、visit_xml、transform_xml、XmlEventWriter | 一个可复用事件缓冲区 | 大文件、过滤和早期查找 |
| DOM 兼容 | XmlUtil、XmlDocument、XmlNode | O(document size) | Hutool 兼容遍历和随机访问 |
| 直接 Serde | XmlSerde | 由 quick-xml 和目标类型控制 | 类型化请求和响应负载 |

`XmlUtil::read_xml` 通过 File 和 BufReader 解析；不再先读完整文件到 String。DOM 构建是迭代的，不在每层嵌套分配新解析器缓冲区。

## 3. 安全默认

`XmlParseOptions::default()` 应用：

| 策略 | 默认 |
|---|---:|
| 最大输入 | 16 MiB |
| 最大深度 | 128 |
| 最大元素 | 100,000 |
| 每元素最大属性 | 256 |
| 最大累积解码文本和 CDATA | 8 MiB |
| 命名空间处理 | 保留限定名 |
| DOCTYPE | 拒绝 |
| 未知命名通用引用 | 拒绝 |
| 无效字符净化 | 禁用 |

数值引用和五个预定义 XML 引用仍支持。允许未知命名引用永不启用 DTD 实体展开。

## 4. 可选 Feature

| Feature | 启用 | 默认 |
|---|---|---|
| xml-serde | 直接 XmlSerde::{from_str, from_reader, to_string, to_writer} | 关 |
| xml-encoding | 非 UTF-8 解码支持 | 关 |
| xml-async | 上游 Tokio 异步适配器 | 关 |

## 5. 验证边界

回归测试套件覆盖：
- 43 个已有 XmlUtil 兼容用例
- 输入、深度、节点、属性和文本限制
- 命名空间保留和显式本地名模式
- DTD、实体、畸形属性和无效字符行为
- 访问者早期终止、流式转换和 writer 转义
- 可选直接 Serde 字符串、reader 和 writer 往返

发布吞吐或内存数字前，在声明的机器上对小配置文件、大日志、深 SOAP 文档、宽节点文档和代表性 OOXML 部件做基准。

## 6. 参考

- 原始文档：`docs/xml.md`（已删除，内容已重定位至此）
