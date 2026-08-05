//! `cn.hutool.core.annotation` 剩余缺口 parity
//!
//! 对齐: `cn.hutool.core.annotation.*` 未覆盖 @Test
//! 来源: hutool-core/src/test/java/cn/hutool/core/annotation/**

mod annotation_common;

#[path = "annotation_parity_gap/gap_aggregate.rs"]
mod gap_aggregate;
#[path = "annotation_parity_gap/gap_attributes.rs"]
mod gap_attributes;
#[path = "annotation_parity_gap/gap_processors.rs"]
mod gap_processors;
#[path = "annotation_parity_gap/gap_scanners.rs"]
mod gap_scanners;
#[path = "annotation_parity_gap/gap_selectors.rs"]
mod gap_selectors;
#[path = "annotation_parity_gap/gap_util.rs"]
mod gap_util;
