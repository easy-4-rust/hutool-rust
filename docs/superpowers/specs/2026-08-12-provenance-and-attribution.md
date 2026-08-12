# 来源与归属设计规格

日期：2026-08-12
来源：重定位 `docs/provenance.md`

## 1. 规则

1. 成熟 Rust crate 优先作为执行引擎
2. Hutool Java 源码定义能力覆盖和兼容语义；Java 实现不做机械翻译
3. 从 yimi-rutool 适配的代码必须经过审查、归属和实质性修订，以适应 Hutool-Rust 的 crate 边界、错误模型和安全默认
4. 安全敏感代码通过已审计上游 crate 实现，永不仅为 API 对等而复制

## 2. 初始来源

| 来源 | 版本 | 许可证 | 用途 |
|---|---:|---|---|
| yimi-rutool | 0.2.5 | Apache-2.0（发行版） | Core/JSON 行为和测试思路 |
| Hutool | 本地 checkout | 木兰 PSL v2 仓库许可证 | 能力和行为参考 |

## 3. 参考

- 原始文档：`docs/provenance.md`（已删除，内容已重定位至此）
