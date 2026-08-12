//! Core utilities shared by the `HiTool` workspace.
//!
//! The crate intentionally avoids async runtimes, HTTP clients, and database
//! drivers. It provides small, deterministic building blocks with explicit
//! errors and allocation behavior.

#![forbid(unsafe_code)]

mod advanced_codec;
mod boolean_util;
mod builder;
mod byte_util;
mod char_util;
mod charset_util;
mod clone_support;
mod codec;
mod collection;
/// Hutool `cn.hutool.core.comparator` 对齐（Fn/Ord 包装；反射 Field 构造为 planned）。
pub mod comparator;
mod compiler;
mod compress;
mod coordinate_util;
#[path = "util/credit_code_util.rs"]
mod credit_code_util;
pub mod date;
mod desensitized_util;
mod error;
pub mod exceptions;
mod getter;
mod hash_util;
mod hex_util;
mod hutool_codec;
mod id;
mod idcard_util;
#[cfg(feature = "img")]
pub mod img;
/// Hutool `cn.hutool.core.io` 子包（File/Path 工具等）。
pub mod io;
mod iter_util;
mod lang;
mod list_util;
/// Hutool `cn.hutool.core.math` 对齐（排列组合 / Money / Calculator / BitStatus）。
pub mod math;
mod mutable;
mod net;
mod page_util;
mod phone_util;
mod radix_codec;
mod radix_util;
pub mod serialize_util;
mod stream;
#[path = "text/str_util.rs"]
mod string;
#[cfg(feature = "swing")]
pub mod swing;
pub mod text;
/// Hutool `cn.hutool.core.thread` 对齐（std::thread / 显式线程池构建；无 JVM ThreadLocal 全局）。
pub mod thread;
mod version_util;
// annotation 模块已迁移到独立 crate hutool-annotation。
// 当前 workspace 仍由 hutool-macro 承载实现，facade crate (hutool) 负责统一 re-export。

pub use advanced_codec::{
    HashIds, MorseCodec, base32_decode, base32_encode, base32_hex_decode, base32_hex_encode,
    base58_decode, base58_decode_checked, base58_decode_checked_auto, base58_encode,
    base58_encode_checked, base62_decode, base62_encode, base62_inverted_decode,
    base62_inverted_encode, bcd_decode, bcd_encode, caesar_decode, caesar_encode,
    idna_decode_domain, idna_encode_domain, punycode_decode, punycode_encode,
    punycode_encode_prefixed, rot_decode, rot_encode,
};
pub use boolean_util::{BooleanError, BooleanUtil};
pub use builder::{
    Builder, BuilderError, CompareToBuilder, EqualsBuilder, GenericBuilder, HashCodeBuilder, IdKey,
};
pub use byte_util::{
    ByteOrder, ByteUtil, ByteUtilError, BytesToNumber, DoubleAdder, LongAdder, NumberToBytes,
};
pub use char_util::{CharError, CharUtil};
pub use charset_util::{Charset, CharsetError, CharsetUtil};
pub use clone_support::{CloneRuntimeException, CloneSupport, DefaultCloneable};
pub use codec::{
    base64_decode, base64_encode, base64_url_decode, base64_url_encode, hex_decode, hex_encode,
    percent_decode, percent_encode_component,
};
pub use collection::{
    ArrayIter, AvgPartition, BlockingQueue, BoundedPriorityQueue, CollStreamUtil, CollUtil,
    CollectionKind, CollectionUtil, ComputeIter, ConcurrentHashSet, CopiedIter, CreatedCollection,
    EnumerationIter, FilterIter, IterChain, IterableIter, IteratorEnumeration, LineIter,
    NodeListIter, Partition, PartitionIter, RandomAccessAvgPartition, RandomAccessPartition,
    ResettableIter, SpliteratorUtil, TransCollection, TransIter, TransSpliterator, UniqueKeySet,
    distinct, group_by, partition, ring_next_for_len, ring_next_index, ring_next_u64,
};
pub use compiler::{
    ClassFileManager, ClassFileObject, CompileOutput, CompilerException, DEFAULT_MAX_SOURCE_BYTES,
    RustSourceCompiler, SourceFileObject, SourceFileObjectUtil, diagnostic_messages,
};
pub use compress::{
    DEFAULT_MAX_SIZE_DIFF, Deflate, Gzip, ZipCopyVisitor, ZipEntry, ZipLimits, ZipReader,
    ZipWriter, memory_zip_writer,
};
pub use coordinate_util::{Coordinate, CoordinateUtil};
pub use credit_code_util::CreditCodeUtil;
pub use date::date_util::DateUtil;
pub use desensitized_util::{DesensitizedType, DesensitizedUtil};
pub use error::{CoreError, Result};
pub use getter::{
    ArrayTypeGetter, BasicTypeGetter, GroupedTypeGetter, ListTypeGetter, OptArrayTypeGetter,
    OptBasicTypeGetter, OptNullBasicTypeFromObjectGetter, OptNullBasicTypeFromStringGetter,
    OptNullBasicTypeGetter, StringMapGetter,
};
pub use hash_util::{HashError, HashUtil};
pub use hex_util::{HexUtil, HexUtilError, RgbColor};
pub use hutool_codec::{
    Base16Codec, Decoder, Encoder, PercentCodec, base64_decode_range_tolerant, base64_decode_text,
    base64_decode_to_file, base64_decode_to_writer, base64_decode_tolerant, base64_encode_config,
    base64_encode_file, base64_encode_reader, base64_encode_text, base64_encode_without_padding,
    encoding_for_label, is_base64, is_base64_code,
};
pub use id::IdUtil;
pub use idcard_util::{Card10Info, Idcard, IdcardError, IdcardUtil};
pub use iter_util::IterUtil;
pub use list_util::ListUtil;
pub use mutable::{
    Mutable, MutableBool, MutableByte, MutableDouble, MutableFloat, MutableInt, MutableLong,
    MutableObj, MutablePair, MutableShort,
};
pub use page_util::{PageError, PageUtil};
pub use phone_util::PhoneUtil;
pub use radix_codec::{
    Base32Decoder, Base32Encoder, Base58Decoder, Base58Encoder, Base62Decoder, Base62Encoder,
    base32_decode_text, base32_decode_to_file, base32_decode_to_writer, base32_encode_file,
    base32_encode_reader, base32_encode_text, base62_decode_text, base62_decode_text_gbk,
    base62_decode_to_file, base62_decode_to_writer, base62_encode_file, base62_encode_reader,
    base62_encode_text, bcd_encode_ascii_prefix,
};
pub use radix_util::{RadixError, RadixUtil};
pub use serialize_util::{
    EnvelopeOptions, Frame, FrameMetadata, MusliDescriptive, MusliPacked, MusliStorage, MusliWire,
    SerializationCodec, SerializeError, SerializeResult, SerializeUtil,
};
pub use stream::{CollectorCharacteristic, CollectorUtil, SimpleCollector, StreamUtil};
pub use string::{
    StrExt, clean_blank, contains, contains_ignore_case, cut, end_with, equals, equals_ignore_case,
    fill, fill_after, fill_before, format_map, format_map_optional, format_template,
    index_of_ignore_case, indexed_format, is_blank, last_index_of, last_index_of_ignore_case,
    length, levenshtein_distance, lower_first, remove_all, remove_chars, repeat, replace,
    replace_by_code_point, reverse, reverse_by_code_point, similarity, similarity_str, split,
    split_to_array, split_to_array_limit, start_with, str_or_empty, strip, strip_ignore_case,
    sub_by_code_point, trim, truncate_by_byte_length, truncate_utf8, upper_first,
};
pub use version_util::{VersionError, VersionUtil};

/// Common imports for applications using `hutool-core`.
pub mod prelude {
    pub use crate::{
        BooleanUtil, ByteOrder, ByteUtil, CharUtil, Charset, CharsetUtil, Coordinate,
        CoordinateUtil, CreditCodeUtil, DateUtil, DesensitizedType, DesensitizedUtil, HashUtil,
        HexUtil, IdUtil, Idcard, IdcardUtil, Mutable, MutableBool, MutableByte, MutableDouble,
        MutableFloat, MutableInt, MutableLong, MutableObj, MutablePair, MutableShort, PageUtil,
        PhoneUtil, RadixUtil, RgbColor, SerializationCodec, SerializeUtil, StrExt, VersionUtil,
    };
}

// ── 新增 util 模块 ──
mod number_util;
pub use number_util::NumberUtil;
mod reflect_util;
pub use reflect_util::ReflectUtil;
mod re_util;
pub use re_util::ReUtil;
mod array_util;
pub use array_util::ArrayUtil;
mod dict;
pub use dict::{Dict, DictUtil};
mod map_util;
pub use map_util::MapUtil;
mod escape_util;
pub use escape_util::EscapeUtil;
mod validator;
pub use validator::Validator;
mod object_util;
pub use object_util::ObjectUtil;
mod type_util;
pub use type_util::TypeUtil;
mod enum_util;
pub use enum_util::EnumUtil;
mod url_util;
pub use url_util::UrlUtil;
mod xml_util;
pub use xml_util::{XmlChild, XmlDocument, XmlNode, XmlUtil};
mod xml_stream;
pub use xml_stream::{
    NamespaceMode, XmlEventReader, XmlEventWriter, XmlParseOptions, XmlTransformAction,
    transform_xml, visit_xml,
};
#[cfg(feature = "xml-serde")]
mod xml_serde;
#[cfg(feature = "xml-serde")]
pub use xml_serde::XmlSerde;
mod file_util;
pub use file_util::FileUtil;
mod io_util;
pub use io_util::IoUtil;
mod random_util;
pub use net::rfc3986::Rfc3986;
pub use random_util::RandomUtil;

/// 对齐 `cn.hutool.core.util.StrUtil`（高阶便捷方法，委托 `crate::string`）。
#[path = "util/str_util.rs"]
pub mod str_util;

/// 对齐 `cn.hutool.core.bean` 子包（Bean 描述 / 属性拷贝 / 动态 Bean 等）。
pub mod bean;
pub use bean::copier::{
    AbsCopier, BeanCopier, BeanCopierException, BeanCopierFactory, BeanToBeanCopier,
    BeanToMapCopier, CopyOptions, IJSONTypeConverter, MapToBeanCopier, MapToMapCopier, ValueKind,
    ValueProvider, ValueProviderToBeanCopier,
};
pub use bean::{BeanException, BeanUtil};

// ── 补齐 1:1 API re-export（修复下游 crate 的 E0432 解析失败）──────────────
mod map;
pub use map::custom_key_map;
pub use map::{
    AbsEntry, BiMap, CamelCaseLinkedMap, CamelCaseMap, CaseInsensitiveLinkedMap,
    CaseInsensitiveMap, CaseInsensitiveTreeMap, CustomKeyMap, FixedLinkedHashMap, FuncKeyMap,
    FuncMap, LinkedForestMap, ListValueMap, MapBuilder, MapWrapper, RowKeyTable,
    SafeConcurrentHashMap, SetValueMap, TableMap, TolerantMap, TransMap,
};
mod clone;
pub mod convert;
pub use clone::cloneable::Cloneable;
mod zip_util;
pub use zip_util::ZipUtil;
mod runtime_util;
pub use runtime_util::RuntimeUtil;
mod primitive_array_util;
pub use comparator::CompareUtil;
pub use date::between_formatter::{BetweenFormatter, Level as BetweenFormatterLevel};
pub use date::date_between::DateBetween;
pub use date::date_field::DateField;
pub use date::date_pattern::DatePattern;
pub use date::date_range::DateRange;
pub use date::date_time::DateTime;
pub use date::date_unit::DateUnit;
pub use date::group_time_interval::GroupTimeInterval;
pub use date::local_date_time_util::LocalDateTimeUtil;
pub use date::month::Month;
pub use date::quarter::Quarter;
pub use date::stop_watch::StopWatch;
pub use date::temporal_accessor_util::TemporalAccessorUtil;
pub use date::temporal_util::TemporalUtil;
pub use date::time_interval::TimeInterval;
pub use date::week::Week;
pub use date::year_quarter::YearQuarter;
pub use date::zodiac::Zodiac;
pub use io::buffer_util::BufferUtil;
pub use io::fast_byte_array_output_stream::FastByteArrayOutputStream;
pub use io::fast_byte_buffer::FastByteBuffer;
pub use io::file::file_name_util::FileNameUtil;
pub use io::file::file_reader::FileReader;
pub use io::file::file_writer::FileWriter;
pub use io::file::line_separator::LineSeparator;
pub use io::file::path_util::PathUtil;
pub use io::io_runtime_exception::IORuntimeException;
pub use io::null_output_stream::NullOutputStream;
pub use io::unit::data_size::DataSize;
pub use io::unit::data_size_util::DataSizeUtil;
pub use io::unit::data_unit::DataUnit;
pub use map_util::{CreateMapKind, EmptyMapKind, LinkedOrHashMap, NestedMapValue};
pub use net::ipv4_util::Ipv4Util;
pub use net::local_port_generater::LocalPortGenerater;
pub use net::net_util::NetUtil;
pub use net::url::url_builder::UrlBuilder;
pub use net::url_decoder::UrlDecoder;
pub use net::url_encode_util::UrlEncodeUtil;
pub use net::url_encoder::UrlEncoder;
pub use object_util::CharSequenceElement;
pub use primitive_array_util::PrimitiveArrayUtil;
