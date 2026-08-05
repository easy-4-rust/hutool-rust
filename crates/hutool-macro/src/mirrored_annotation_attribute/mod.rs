//! 对齐: `cn.hutool.core.annotation.MirroredAnnotationAttribute`

use std::sync::Arc;

use crate::annotation_attribute::AnnotationAttribute;
use crate::mirror::AnnotationValue;

mod mirror_value_conflict_error;
mod mirrored_annotation_attribute;

pub use mirror_value_conflict_error::MirrorValueConflictError;
pub use mirrored_annotation_attribute::MirroredAnnotationAttribute;

fn mirror_value(
    original: &Arc<dyn AnnotationAttribute>,
    linked: &Arc<dyn AnnotationAttribute>,
) -> AnnotationValue {
    mirror_value_result(original, linked).unwrap_or_else(|e| panic!("{}", e.message))
}

fn mirror_value_result(
    original: &Arc<dyn AnnotationAttribute>,
    linked: &Arc<dyn AnnotationAttribute>,
) -> Result<AnnotationValue, MirrorValueConflictError> {
    let origin_default = original.is_value_equivalent_to_default_value();
    let target_default = linked.is_value_equivalent_to_default_value();
    let origin_value = original.get_value();
    let target_value = linked.get_value();

    if origin_default == target_default {
        if origin_value != target_value {
            return Err(MirrorValueConflictError {
                message: format!(
                    "mirror values differ: {:?} <==> {:?}",
                    origin_value, target_value
                ),
            });
        }
        return Ok(origin_value);
    }
    Ok(if origin_default {
        target_value
    } else {
        origin_value
    })
}
