//! `cn.hutool.core.bean.copier` 子包对齐
//!
//! 自动生成的模块入口,1:1 镜像 Java 包结构。
//! 每个子模块对应一个 Java 类(`.java` → `.rs`),命名遵循 snake_case。
//! 详细对齐信息见各 `.rs` 文件头注释。

pub mod abs_copier;
pub mod bean_copier;
pub mod bean_copier_exception;
pub mod bean_copier_factory;
pub mod bean_to_bean_copier;
pub mod bean_to_map_copier;
pub mod copy_options;
pub mod ijson_type_converter;
pub mod map_to_bean_copier;
pub mod map_to_map_copier;
pub mod provider;
pub mod value_provider;
pub mod value_provider_to_bean_copier;

// Re-export 核心类型，方便外部使用
pub use abs_copier::AbsCopier;
pub use bean_copier::{copy_bean_to_map, copy_map_to_bean, copy_map_to_map, copy_properties, BeanCopier};
pub use bean_copier_exception::BeanCopierException;
pub use bean_copier_factory::BeanCopierFactory;
pub use bean_to_bean_copier::BeanToBeanCopier;
pub use bean_to_map_copier::BeanToMapCopier;
pub use copy_options::CopyOptions;
pub use ijson_type_converter::IJSONTypeConverter;
pub use map_to_bean_copier::MapToBeanCopier;
pub use map_to_map_copier::MapToMapCopier;
pub use value_provider::{ValueKind, ValueProvider};
pub use value_provider_to_bean_copier::ValueProviderToBeanCopier;
