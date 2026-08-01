//! Binary serialization helpers aligned with Hutool's `SerializeUtil`.
//!
//! HiTool keeps the public API stable while optional engines provide the actual
//! encoding. Müsli formats are intentionally explicit: wire is for independently
//! upgraded peers, storage is for forward-evolving persisted data, packed is for
//! synchronized models, and descriptive is for inspection and migration.

/// 对齐: `cn.hutool.core.util.SerializeUtil`
/// 序列化工具类

use std::{fmt, ops::RangeInclusive};

#[cfg(feature = "serialization")]
use crc32fast::Hasher;
#[cfg(feature = "serialization-musli")]
pub use musli::{Decode, Encode};

/// Current HiTool serialization-envelope version.
pub const ENVELOPE_VERSION: u8 = 1;
/// Number of bytes before an encoded payload.
pub const ENVELOPE_HEADER_LEN: usize = 28;
/// Default upper bound for a serialized payload: 16 MiB.
pub const DEFAULT_MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

#[allow(dead_code)] // 对齐 Java 序列化封包常量，暂未接线，预留
const MAGIC: [u8; 4] = *b"HITL";
#[allow(dead_code)] // 对齐 Java 序列化封包常量，暂未接线，预留
const CHECKSUM_FLAG: u16 = 1;
#[allow(dead_code)] // 对齐 Java 序列化封包常量，暂未接线，预留
const SUPPORTED_FLAGS: u16 = CHECKSUM_FLAG;

/// Result returned by serialization operations.
pub type SerializeResult<T> = std::result::Result<T, SerializeError>;

/// Stable identifiers written into HiTool serialization envelopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SerializationCodec {
    /// Fully upgrade-stable Müsli wire encoding.
    MusliWire = 1,
    /// Partially upgrade-stable Müsli storage encoding.
    MusliStorage = 2,
    /// Compact Müsli packed encoding for synchronized models.
    MusliPacked = 3,
    /// Self-descriptive Müsli encoding.
    MusliDescriptive = 4,
}

impl TryFrom<u8> for SerializationCodec {
    type Error = SerializeError;

    fn try_from(value: u8) -> SerializeResult<Self> {
        match value {
            1 => Ok(Self::MusliWire),
            2 => Ok(Self::MusliStorage),
            3 => Ok(Self::MusliPacked),
            4 => Ok(Self::MusliDescriptive),
            codec => Err(SerializeError::UnsupportedCodec { codec }),
        }
    }
}

impl fmt::Display for SerializationCodec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MusliWire => "musli-wire",
            Self::MusliStorage => "musli-storage",
            Self::MusliPacked => "musli-packed",
            Self::MusliDescriptive => "musli-descriptive",
        })
    }
}

/// Options controlling envelope validation and resource usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvelopeOptions {
    schema_id: u32,
    schema_version: u16,
    minimum_compatible_version: u16,
    maximum_compatible_version: u16,
    max_payload_len: usize,
    checksum: bool,
    require_checksum: bool,
}

impl EnvelopeOptions {
    /// Creates options for one exact schema version.
    #[must_use]
    pub const fn new(schema_id: u32, schema_version: u16) -> Self {
        Self {
            schema_id,
            schema_version,
            minimum_compatible_version: schema_version,
            maximum_compatible_version: schema_version,
            max_payload_len: DEFAULT_MAX_PAYLOAD_LEN,
            checksum: true,
            require_checksum: true,
        }
    }

    /// Accepts frames whose schema versions fall in the inclusive range.
    #[must_use]
    pub fn with_compatible_versions(mut self, versions: RangeInclusive<u16>) -> Self {
        self.minimum_compatible_version = *versions.start();
        self.maximum_compatible_version = *versions.end();
        self
    }

    /// Sets the maximum accepted or emitted payload length.
    #[must_use]
    pub const fn with_max_payload_len(mut self, max_payload_len: usize) -> Self {
        self.max_payload_len = max_payload_len;
        self
    }

    /// Controls whether newly encoded frames contain a CRC32 checksum.
    #[must_use]
    pub const fn with_checksum(mut self, checksum: bool) -> Self {
        self.checksum = checksum;
        self
    }

    /// Controls whether decoding rejects frames without a checksum.
    #[must_use]
    pub const fn require_checksum(mut self, require_checksum: bool) -> Self {
        self.require_checksum = require_checksum;
        self
    }

    /// Returns the stable schema identifier.
    #[must_use]
    pub const fn schema_id(self) -> u32 {
        self.schema_id
    }

    /// Returns the schema version written when encoding.
    #[must_use]
    pub const fn schema_version(self) -> u16 {
        self.schema_version
    }

    /// Returns the maximum payload length.
    #[must_use]
    pub const fn max_payload_len(self) -> usize {
        self.max_payload_len
    }

    #[allow(dead_code)] // 对齐 Java 封包校验逻辑，暂未接线，预留
    fn validate(self) -> SerializeResult<()> {
        if self.minimum_compatible_version > self.maximum_compatible_version {
            return Err(SerializeError::InvalidOptions(
                "minimum compatible schema version exceeds maximum",
            ));
        }

        if !self.compatible_versions().contains(&self.schema_version) {
            return Err(SerializeError::InvalidOptions(
                "encoded schema version is outside the compatible range",
            ));
        }

        Ok(())
    }

    #[allow(dead_code)] // 对齐 Java 封包校验逻辑，暂未接线，预留
    fn compatible_versions(self) -> RangeInclusive<u16> {
        self.minimum_compatible_version..=self.maximum_compatible_version
    }
}

impl Default for EnvelopeOptions {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

/// Metadata decoded from a validated HiTool envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameMetadata {
    /// Encoding used for the payload.
    pub codec: SerializationCodec,
    /// Application-defined schema identifier.
    pub schema_id: u32,
    /// Application-defined schema version.
    pub schema_version: u16,
    /// Encoded payload length.
    pub payload_len: usize,
    /// Whether the frame carries a CRC32 checksum.
    pub checksummed: bool,
}

/// A validated frame borrowing its payload from the input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame<'a> {
    /// Validated frame metadata.
    pub metadata: FrameMetadata,
    /// Exact payload bytes, with the envelope removed.
    pub payload: &'a [u8],
}

/// Errors emitted by HiTool serialization and envelope processing.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerializeError {
    /// The selected engine failed while encoding.
    #[error("{codec} encoding failed: {message}")]
    Encode {
        /// Selected codec.
        codec: SerializationCodec,
        /// Engine diagnostic isolated from the public error type.
        message: String,
    },
    /// The selected engine failed while decoding.
    #[error("{codec} decoding failed: {message}")]
    Decode {
        /// Selected codec.
        codec: SerializationCodec,
        /// Engine diagnostic isolated from the public error type.
        message: String,
    },
    /// A fixed output buffer could not hold the encoded value.
    #[error("{codec} output buffer is too small")]
    BufferTooSmall {
        /// Selected codec.
        codec: SerializationCodec,
    },
    /// Payload length exceeded the configured resource limit.
    #[error("payload exceeds limit: {actual} > {limit}")]
    PayloadTooLarge {
        /// Observed or requested payload length.
        actual: usize,
        /// Configured limit.
        limit: usize,
    },
    /// Input did not contain a complete envelope header.
    #[error("truncated serialization envelope: {actual} < {required}")]
    TruncatedEnvelope {
        /// Available byte count.
        actual: usize,
        /// Required byte count.
        required: usize,
    },
    /// Envelope magic did not identify HiTool data.
    #[error("invalid HiTool serialization magic")]
    InvalidMagic,
    /// Envelope version is not supported by this release.
    #[error("unsupported serialization envelope version: {version}")]
    UnsupportedEnvelopeVersion {
        /// Version found in the frame.
        version: u8,
    },
    /// Codec identifier is not known.
    #[error("unsupported serialization codec: {codec}")]
    UnsupportedCodec {
        /// Raw codec identifier.
        codec: u8,
    },
    /// Frame used a different codec from the explicitly selected facade.
    #[error("unexpected codec: expected {expected}, got {actual}")]
    UnexpectedCodec {
        /// Codec required by the facade.
        expected: SerializationCodec,
        /// Codec found in the frame.
        actual: SerializationCodec,
    },
    /// Schema identifier did not match the configured application schema.
    #[error("schema mismatch: expected {expected}, got {actual}")]
    SchemaMismatch {
        /// Configured schema identifier.
        expected: u32,
        /// Identifier found in the frame.
        actual: u32,
    },
    /// Schema version fell outside the configured compatibility range.
    #[error("incompatible schema version: {actual}, expected {minimum}..={maximum}")]
    IncompatibleSchemaVersion {
        /// Version found in the frame.
        actual: u16,
        /// Minimum supported version.
        minimum: u16,
        /// Maximum supported version.
        maximum: u16,
    },
    /// Declared payload length did not consume the whole input.
    #[error("frame length mismatch: declared {declared}, actual {actual}")]
    LengthMismatch {
        /// Length declared by the envelope.
        declared: usize,
        /// Bytes actually available after the header.
        actual: usize,
    },
    /// CRC32 validation failed.
    #[error("serialization checksum mismatch")]
    ChecksumMismatch,
    /// Policy required checksums but the frame did not contain one.
    #[error("serialization frame is missing a required checksum")]
    ChecksumRequired,
    /// A frame used flags not understood by this release.
    #[error("unsupported serialization envelope flags: {flags:#06x}")]
    UnsupportedFlags {
        /// Unknown flag bits.
        flags: u16,
    },
    /// A decoder left unconsumed bytes inside the declared payload.
    #[error("{codec} payload contains {remaining} trailing bytes")]
    TrailingBytes {
        /// Selected codec.
        codec: SerializationCodec,
        /// Number of unconsumed bytes.
        remaining: usize,
    },
    /// Envelope options were internally inconsistent.
    #[error("invalid serialization options: {0}")]
    InvalidOptions(&'static str),
}

/// Validates and inspects an envelope without decoding its payload.
#[cfg(feature = "serialization")]
pub fn inspect_frame<'a>(bytes: &'a [u8], options: &EnvelopeOptions) -> SerializeResult<Frame<'a>> {
    parse_frame(bytes, None, options)
}

#[cfg(feature = "serialization")]
fn parse_frame<'a>(
    bytes: &'a [u8],
    expected_codec: Option<SerializationCodec>,
    options: &EnvelopeOptions,
) -> SerializeResult<Frame<'a>> {
    options.validate()?;

    if bytes.len() < ENVELOPE_HEADER_LEN {
        return Err(SerializeError::TruncatedEnvelope {
            actual: bytes.len(),
            required: ENVELOPE_HEADER_LEN,
        });
    }

    if bytes[..4] != MAGIC {
        return Err(SerializeError::InvalidMagic);
    }

    let version = bytes[4];
    if version != ENVELOPE_VERSION {
        return Err(SerializeError::UnsupportedEnvelopeVersion { version });
    }

    let codec = SerializationCodec::try_from(bytes[5])?;
    if let Some(expected) = expected_codec {
        if codec != expected {
            return Err(SerializeError::UnexpectedCodec {
                expected,
                actual: codec,
            });
        }
    }

    let flags = u16::from_be_bytes([bytes[6], bytes[7]]);
    if flags & !SUPPORTED_FLAGS != 0 {
        return Err(SerializeError::UnsupportedFlags {
            flags: flags & !SUPPORTED_FLAGS,
        });
    }

    let schema_id = u32::from_be_bytes(bytes[8..12].try_into().expect("fixed header range"));
    if schema_id != options.schema_id {
        return Err(SerializeError::SchemaMismatch {
            expected: options.schema_id,
            actual: schema_id,
        });
    }

    let schema_version = u16::from_be_bytes(bytes[12..14].try_into().expect("fixed header range"));
    if !options.compatible_versions().contains(&schema_version) {
        return Err(SerializeError::IncompatibleSchemaVersion {
            actual: schema_version,
            minimum: options.minimum_compatible_version,
            maximum: options.maximum_compatible_version,
        });
    }

    let payload_len_u64 = u64::from_be_bytes(bytes[16..24].try_into().expect("fixed header range"));
    #[cfg(target_pointer_width = "64")]
    let payload_len = payload_len_u64 as usize;
    #[cfg(not(target_pointer_width = "64"))]
    let payload_len =
        usize::try_from(payload_len_u64).map_err(|_| SerializeError::PayloadTooLarge {
            actual: usize::MAX,
            limit: options.max_payload_len,
        })?;
    if payload_len > options.max_payload_len {
        return Err(SerializeError::PayloadTooLarge {
            actual: payload_len,
            limit: options.max_payload_len,
        });
    }

    let actual = bytes.len() - ENVELOPE_HEADER_LEN;
    if actual != payload_len {
        return Err(SerializeError::LengthMismatch {
            declared: payload_len,
            actual,
        });
    }

    let checksummed = flags & CHECKSUM_FLAG != 0;
    if options.require_checksum && !checksummed {
        return Err(SerializeError::ChecksumRequired);
    }

    let payload = &bytes[ENVELOPE_HEADER_LEN..];
    if checksummed {
        let expected = u32::from_be_bytes(bytes[24..28].try_into().expect("fixed header range"));
        if crc32(payload) != expected {
            return Err(SerializeError::ChecksumMismatch);
        }
    }

    Ok(Frame {
        metadata: FrameMetadata {
            codec,
            schema_id,
            schema_version,
            payload_len,
            checksummed,
        },
        payload,
    })
}

#[cfg(feature = "serialization")]
fn encode_frame_with(
    output: &mut Vec<u8>,
    codec: SerializationCodec,
    options: &EnvelopeOptions,
    encode: impl FnOnce(&mut Vec<u8>) -> SerializeResult<()>,
) -> SerializeResult<()> {
    output.clear();
    options.validate()?;
    output.resize(ENVELOPE_HEADER_LEN, 0);

    if let Err(error) = encode(output) {
        output.clear();
        return Err(error);
    }

    let payload_len = output.len() - ENVELOPE_HEADER_LEN;
    if payload_len > options.max_payload_len {
        output.clear();
        return Err(SerializeError::PayloadTooLarge {
            actual: payload_len,
            limit: options.max_payload_len,
        });
    }

    let flags = if options.checksum { CHECKSUM_FLAG } else { 0 };
    output[..4].copy_from_slice(&MAGIC);
    output[4] = ENVELOPE_VERSION;
    output[5] = codec as u8;
    output[6..8].copy_from_slice(&flags.to_be_bytes());
    output[8..12].copy_from_slice(&options.schema_id.to_be_bytes());
    output[12..14].copy_from_slice(&options.schema_version.to_be_bytes());
    output[14..16].fill(0);
    output[16..24].copy_from_slice(&(payload_len as u64).to_be_bytes());

    let checksum = if options.checksum {
        crc32(&output[ENVELOPE_HEADER_LEN..])
    } else {
        0
    };
    output[24..28].copy_from_slice(&checksum.to_be_bytes());
    Ok(())
}

#[cfg(feature = "serialization")]
fn crc32(bytes: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(bytes);
    hasher.finalize()
}

/// Müsli wire facade for independently upgraded peers.
#[derive(Debug, Clone, Copy, Default)]
pub struct MusliWire;
/// Müsli storage facade for forward-evolving persisted data.
#[derive(Debug, Clone, Copy, Default)]
pub struct MusliStorage;
/// Müsli packed facade for synchronized models and hot paths.
#[derive(Debug, Clone, Copy, Default)]
pub struct MusliPacked;
/// Müsli descriptive facade for inspection and migration.
#[derive(Debug, Clone, Copy, Default)]
pub struct MusliDescriptive;

#[cfg(feature = "serialization-musli")]
macro_rules! define_musli_codec {
    ($type:ident, $feature:literal, $module:ident, $codec:expr) => {
        #[cfg(feature = $feature)]
        impl $type {
            /// Encodes a raw payload without a HiTool envelope.
            pub fn encode<T>(value: &T) -> SerializeResult<Vec<u8>>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                musli::$module::to_vec(value).map_err(|error| SerializeError::Encode {
                    codec: $codec,
                    message: error.to_string(),
                })
            }

            /// Encodes into a reusable vector, clearing it first.
            ///
            /// The vector is empty if encoding fails.
            pub fn encode_into<T>(output: &mut Vec<u8>, value: &T) -> SerializeResult<()>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                output.clear();
                if let Err(error) = musli::$module::Encoding::new().encode(&mut *output, value) {
                    output.clear();
                    return Err(SerializeError::Encode {
                        codec: $codec,
                        message: error.to_string(),
                    });
                }
                Ok(())
            }

            /// Encodes into caller-owned fixed storage.
            pub fn encode_to_slice<T>(output: &mut [u8], value: &T) -> SerializeResult<usize>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                musli::$module::to_slice(output, value).map_err(|error| {
                    let message = error.to_string();
                    let normalized = message.to_ascii_lowercase();
                    if normalized.contains("overflow")
                        || normalized.contains("full")
                        || normalized.contains("space")
                    {
                        SerializeError::BufferTooSmall { codec: $codec }
                    } else {
                        SerializeError::Encode {
                            codec: $codec,
                            message,
                        }
                    }
                })
            }

            /// Decodes one exact raw payload and rejects trailing bytes.
            pub fn decode<'de, T>(bytes: &'de [u8]) -> SerializeResult<T>
            where
                T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
            {
                decode_exact::<T, _>(bytes, $codec, |reader| {
                    musli::$module::decode(reader).map_err(|error| SerializeError::Decode {
                        codec: $codec,
                        message: error.to_string(),
                    })
                })
            }

            /// Encodes a checksummed, schema-aware HiTool frame.
            pub fn encode_frame<T>(value: &T, options: &EnvelopeOptions) -> SerializeResult<Vec<u8>>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                let mut output = Vec::new();
                Self::encode_frame_into(&mut output, value, options)?;
                Ok(output)
            }

            /// Encodes a frame into a reusable vector.
            ///
            /// The vector is empty if encoding or envelope validation fails.
            pub fn encode_frame_into<T>(
                output: &mut Vec<u8>,
                value: &T,
                options: &EnvelopeOptions,
            ) -> SerializeResult<()>
            where
                T: ?Sized + musli::Encode<musli::mode::Binary>,
            {
                encode_frame_with(output, $codec, options, |output| {
                    musli::$module::Encoding::new()
                        .encode(output, value)
                        .map(|_| ())
                        .map_err(|error| SerializeError::Encode {
                            codec: $codec,
                            message: error.to_string(),
                        })
                })
            }

            /// Validates an envelope and decodes its exact payload.
            pub fn decode_frame<'de, T>(
                bytes: &'de [u8],
                options: &EnvelopeOptions,
            ) -> SerializeResult<T>
            where
                T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
            {
                let frame = parse_frame(bytes, Some($codec), options)?;
                Self::decode(frame.payload)
            }
        }
    };
}

#[cfg(feature = "serialization-musli")]
fn decode_exact<'de, T, F>(
    bytes: &'de [u8],
    codec: SerializationCodec,
    decode: F,
) -> SerializeResult<T>
where
    F: FnOnce(&mut &'de [u8]) -> SerializeResult<T>,
{
    let mut reader = bytes;
    let value = decode(&mut reader)?;
    if !reader.is_empty() {
        return Err(SerializeError::TrailingBytes {
            codec,
            remaining: reader.len(),
        });
    }
    Ok(value)
}

#[cfg(feature = "serialization-musli")]
define_musli_codec!(MusliWire, "musli-wire", wire, SerializationCodec::MusliWire);
#[cfg(feature = "serialization-musli")]
define_musli_codec!(
    MusliStorage,
    "musli-storage",
    storage,
    SerializationCodec::MusliStorage
);
#[cfg(feature = "serialization-musli")]
define_musli_codec!(
    MusliPacked,
    "musli-packed",
    packed,
    SerializationCodec::MusliPacked
);
#[cfg(feature = "serialization-musli")]
define_musli_codec!(
    MusliDescriptive,
    "musli-descriptive",
    descriptive,
    SerializationCodec::MusliDescriptive
);

/// Hutool-aligned convenience facade.
///
/// Its compatibility methods deliberately use Müsli wire frames, so the
/// serialized bytes are self-identifying and tolerate independently upgraded
/// Rust models. New code may use [`MusliWire`] directly to make the format
/// choice visible at the call site.
#[derive(Debug, Clone, Copy, Default)]
pub struct SerializeUtil;

#[cfg(feature = "musli-wire")]
#[allow(clippy::should_implement_trait)]
impl SerializeUtil {
    /// Deep-clones an owned value through a wire round trip.
    pub fn clone<T>(value: &T) -> SerializeResult<T>
    where
        T: musli::Encode<musli::mode::Binary>,
        T: for<'de> musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
    {
        let bytes = MusliWire::encode(value)?;
        MusliWire::decode(&bytes)
    }

    /// Serializes using a checksummed default wire envelope.
    pub fn serialize<T>(value: &T) -> SerializeResult<Vec<u8>>
    where
        T: ?Sized + musli::Encode<musli::mode::Binary>,
    {
        MusliWire::encode_frame(value, &EnvelopeOptions::default())
    }

    /// Deserializes a checksummed default wire envelope.
    pub fn deserialize<'de, T>(bytes: &'de [u8]) -> SerializeResult<T>
    where
        T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
    {
        MusliWire::decode_frame(bytes, &EnvelopeOptions::default())
    }

    /// Serializes using caller-defined schema and resource policies.
    pub fn serialize_with_options<T>(
        value: &T,
        options: &EnvelopeOptions,
    ) -> SerializeResult<Vec<u8>>
    where
        T: ?Sized + musli::Encode<musli::mode::Binary>,
    {
        MusliWire::encode_frame(value, options)
    }

    /// Deserializes using caller-defined schema and resource policies.
    pub fn deserialize_with_options<'de, T>(
        bytes: &'de [u8],
        options: &EnvelopeOptions,
    ) -> SerializeResult<T>
    where
        T: musli::Decode<'de, musli::mode::Binary, musli::alloc::Global>,
    {
        MusliWire::decode_frame(bytes, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_error_kind<T: fmt::Debug>(result: SerializeResult<T>, expected: SerializeError) {
        let actual = result.unwrap_err();
        assert_eq!(
            std::mem::discriminant(&actual),
            std::mem::discriminant(&expected),
            "unexpected error: {actual}"
        );
    }

    #[test]
    fn codec_ids_options_and_errors_are_stable() {
        assert_eq!(
            SerializationCodec::try_from(1).unwrap(),
            SerializationCodec::MusliWire
        );
        assert_eq!(
            SerializationCodec::try_from(2).unwrap(),
            SerializationCodec::MusliStorage
        );
        assert_eq!(
            SerializationCodec::try_from(3).unwrap(),
            SerializationCodec::MusliPacked
        );
        assert_eq!(
            SerializationCodec::try_from(4).unwrap(),
            SerializationCodec::MusliDescriptive
        );
        assert_eq!(SerializationCodec::MusliWire.to_string(), "musli-wire");
        assert_eq!(
            SerializationCodec::MusliStorage.to_string(),
            "musli-storage"
        );
        assert_eq!(SerializationCodec::MusliPacked.to_string(), "musli-packed");
        assert_eq!(
            SerializationCodec::MusliDescriptive.to_string(),
            "musli-descriptive"
        );
        assert_error_kind(
            SerializationCodec::try_from(99),
            SerializeError::UnsupportedCodec { codec: 99 },
        );

        let options = EnvelopeOptions::new(7, 2)
            .with_compatible_versions(1..=3)
            .with_max_payload_len(512)
            .with_checksum(false)
            .require_checksum(false);
        assert_eq!(options.schema_id(), 7);
        assert_eq!(options.schema_version(), 2);
        assert_eq!(options.max_payload_len(), 512);
        assert_eq!(
            EnvelopeOptions::default().max_payload_len(),
            DEFAULT_MAX_PAYLOAD_LEN
        );
        assert!(format!("{}", SerializeError::InvalidMagic).contains("magic"));
        assert_error_kind(
            EnvelopeOptions::new(1, 2)
                .with_compatible_versions(3..=1)
                .validate(),
            SerializeError::InvalidOptions("test"),
        );
        assert_error_kind(
            EnvelopeOptions::new(1, 2)
                .with_compatible_versions(3..=4)
                .validate(),
            SerializeError::InvalidOptions("test"),
        );
    }

    #[cfg(feature = "serialization")]
    #[test]
    fn envelope_validation_rejects_malformed_frames() {
        let options = EnvelopeOptions::new(7, 2).with_compatible_versions(1..=3);
        assert_error_kind(
            inspect_frame(&[], &options),
            SerializeError::TruncatedEnvelope {
                actual: 0,
                required: ENVELOPE_HEADER_LEN,
            },
        );

        let mut bytes = vec![0_u8; ENVELOPE_HEADER_LEN];
        assert_eq!(
            inspect_frame(&bytes, &options),
            Err(SerializeError::InvalidMagic)
        );
        bytes[..4].copy_from_slice(&MAGIC);
        bytes[4] = 9;
        assert_error_kind(
            inspect_frame(&bytes, &options),
            SerializeError::UnsupportedEnvelopeVersion { version: 9 },
        );
    }

    #[cfg(feature = "musli-wire")]
    mod wire {
        use super::*;
        use musli::{Context, Encoder};

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Version1 {
            #[musli(Binary, name = 0)]
            name: String,
        }

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Version2 {
            #[musli(Binary, name = 0)]
            name: String,
            #[musli(Binary, name = 1)]
            #[musli(default)]
            age: Option<u32>,
        }

        #[derive(Debug, Decode)]
        struct FailingEncode;

        #[derive(Encode)]
        struct SuccessfulUnitEncode;

        impl<M: 'static> musli::Encode<M> for FailingEncode {
            type Encode = Self;

            fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
            where
                E: Encoder<Mode = M>,
            {
                Err(encoder.cx().message("intentional encoding failure"))
            }

            fn as_encode(&self) -> &Self::Encode {
                self
            }
        }

        #[test]
        fn wire_round_trip_reuses_buffers_and_supports_schema_evolution() {
            let current = Version2 {
                name: "Aristotle".into(),
                age: Some(61),
            };
            let mut raw = vec![0xff; 64];
            MusliWire::encode_into(&mut raw, &current).unwrap();
            let old: Version1 = MusliWire::decode(&raw).unwrap();
            assert_eq!(old.name, "Aristotle");

            let mut fixed = [0_u8; 128];
            let len = MusliWire::encode_to_slice(&mut fixed, &current).unwrap();
            assert_eq!(
                MusliWire::decode::<Version2>(&fixed[..len]).unwrap(),
                current
            );
            assert_error_kind(
                MusliWire::decode::<Version2>(&[raw.as_slice(), &[0]].concat()),
                SerializeError::TrailingBytes {
                    codec: SerializationCodec::MusliWire,
                    remaining: 1,
                },
            );
            assert_error_kind(
                MusliWire::decode::<Version2>(&[]),
                SerializeError::Decode {
                    codec: SerializationCodec::MusliWire,
                    message: String::new(),
                },
            );
            assert_error_kind(
                MusliWire::encode_to_slice(&mut [], &current),
                SerializeError::BufferTooSmall {
                    codec: SerializationCodec::MusliWire,
                },
            );
        }

        #[test]
        fn encoding_failures_clear_reused_buffers() {
            let failing = FailingEncode;
            assert!(std::ptr::eq(
                <FailingEncode as musli::Encode<musli::mode::Binary>>::as_encode(&failing),
                &failing
            ));
            let valid_unit = MusliWire::encode(&SuccessfulUnitEncode).unwrap();
            let _: FailingEncode = MusliWire::decode(&valid_unit).unwrap();
            assert_error_kind(
                MusliWire::encode(&failing),
                SerializeError::Encode {
                    codec: SerializationCodec::MusliWire,
                    message: String::new(),
                },
            );
            assert_error_kind(
                MusliWire::encode_frame(&failing, &EnvelopeOptions::default()),
                SerializeError::Encode {
                    codec: SerializationCodec::MusliWire,
                    message: String::new(),
                },
            );
            assert_error_kind(
                SerializeUtil::clone(&failing),
                SerializeError::Encode {
                    codec: SerializationCodec::MusliWire,
                    message: String::new(),
                },
            );

            let mut output = vec![1, 2, 3];
            assert_error_kind(
                MusliWire::encode_into(&mut output, &FailingEncode),
                SerializeError::Encode {
                    codec: SerializationCodec::MusliWire,
                    message: String::new(),
                },
            );
            assert!(output.is_empty());

            output.extend_from_slice(&[1, 2, 3]);
            assert_error_kind(
                MusliWire::encode_frame_into(
                    &mut output,
                    &FailingEncode,
                    &EnvelopeOptions::default(),
                ),
                SerializeError::Encode {
                    codec: SerializationCodec::MusliWire,
                    message: String::new(),
                },
            );
            assert!(output.is_empty());

            assert_error_kind(
                MusliWire::encode_to_slice(&mut [0_u8; 32], &FailingEncode),
                SerializeError::Encode {
                    codec: SerializationCodec::MusliWire,
                    message: String::new(),
                },
            );
        }

        #[test]
        fn wire_frames_validate_codec_schema_length_checksum_and_limits() {
            let value = Version2 {
                name: "Plato".into(),
                age: None,
            };
            let options = EnvelopeOptions::new(0x4854_4f4f, 2)
                .with_compatible_versions(1..=2)
                .with_max_payload_len(1024);
            let mut frame = vec![0xff; 32];
            MusliWire::encode_frame_into(&mut frame, &value, &options).unwrap();

            let metadata = inspect_frame(&frame, &options).unwrap().metadata;
            assert_eq!(metadata.codec, SerializationCodec::MusliWire);
            assert_eq!(metadata.schema_version, 2);
            assert!(metadata.checksummed);
            assert_eq!(
                MusliWire::decode_frame::<Version2>(&frame, &options).unwrap(),
                value
            );

            let mut corrupted = frame.clone();
            *corrupted.last_mut().unwrap() ^= 1;
            assert_eq!(
                MusliWire::decode_frame::<Version2>(&corrupted, &options),
                Err(SerializeError::ChecksumMismatch)
            );

            let wrong_schema = EnvelopeOptions::new(1, 2);
            assert_error_kind(
                MusliWire::decode_frame::<Version2>(&frame, &wrong_schema),
                SerializeError::SchemaMismatch {
                    expected: 0,
                    actual: 0,
                },
            );

            let too_small = options.with_max_payload_len(1);
            let mut output = vec![1, 2, 3];
            assert_error_kind(
                MusliWire::encode_frame_into(&mut output, &value, &too_small),
                SerializeError::PayloadTooLarge {
                    actual: 0,
                    limit: 0,
                },
            );
            assert!(output.is_empty());
        }

        #[test]
        fn invalid_options_fail_before_encoding_and_clear_output() {
            let invalid = EnvelopeOptions::new(1, 2).with_compatible_versions(3..=1);
            assert_error_kind(
                inspect_frame(&[], &invalid),
                SerializeError::InvalidOptions("test"),
            );

            let mut output = vec![1, 2, 3];
            assert_error_kind(
                MusliWire::encode_frame_into(&mut output, &1_u8, &invalid),
                SerializeError::InvalidOptions("test"),
            );
            assert!(output.is_empty());
        }

        #[test]
        fn frame_policy_rejects_every_unsupported_header_combination() {
            let value = Version2 {
                name: "Plato".into(),
                age: None,
            };
            let options = EnvelopeOptions::new(7, 2).with_compatible_versions(1..=3);
            let frame = MusliWire::encode_frame(&value, &options).unwrap();

            let mut wrong_codec = frame.clone();
            wrong_codec[5] = SerializationCodec::MusliStorage as u8;
            assert_error_kind(
                MusliWire::decode_frame::<Version2>(&wrong_codec, &options),
                SerializeError::UnexpectedCodec {
                    expected: SerializationCodec::MusliWire,
                    actual: SerializationCodec::MusliStorage,
                },
            );

            let mut unsupported_codec = frame.clone();
            unsupported_codec[5] = 99;
            assert_error_kind(
                inspect_frame(&unsupported_codec, &options),
                SerializeError::UnsupportedCodec { codec: 99 },
            );

            let mut unsupported_flags = frame.clone();
            unsupported_flags[6..8].copy_from_slice(&2_u16.to_be_bytes());
            assert_error_kind(
                inspect_frame(&unsupported_flags, &options),
                SerializeError::UnsupportedFlags { flags: 2 },
            );

            let mut incompatible_version = frame.clone();
            incompatible_version[12..14].copy_from_slice(&4_u16.to_be_bytes());
            assert_error_kind(
                inspect_frame(&incompatible_version, &options),
                SerializeError::IncompatibleSchemaVersion {
                    actual: 4,
                    minimum: 0,
                    maximum: 0,
                },
            );

            let mut oversized = frame.clone();
            oversized[16..24].copy_from_slice(&2048_u64.to_be_bytes());
            assert_error_kind(
                inspect_frame(&oversized, &options.with_max_payload_len(1024)),
                SerializeError::PayloadTooLarge {
                    actual: 2048,
                    limit: 0,
                },
            );

            let mut wrong_length = frame.clone();
            wrong_length[16..24].copy_from_slice(&1_u64.to_be_bytes());
            assert_error_kind(
                inspect_frame(&wrong_length, &options),
                SerializeError::LengthMismatch {
                    declared: 0,
                    actual: 0,
                },
            );

            let unchecked_options = options.with_checksum(false).require_checksum(false);
            let unchecked = MusliWire::encode_frame(&value, &unchecked_options).unwrap();
            assert!(
                !inspect_frame(&unchecked, &unchecked_options)
                    .unwrap()
                    .metadata
                    .checksummed
            );
            assert_eq!(
                inspect_frame(&unchecked, &options),
                Err(SerializeError::ChecksumRequired)
            );
        }

        #[test]
        fn serialize_util_is_a_real_wire_backed_facade() {
            let value = Version2 {
                name: "Socrates".into(),
                age: Some(70),
            };
            assert_eq!(SerializeUtil::clone(&value).unwrap(), value);
            let bytes = SerializeUtil::serialize(&value).unwrap();
            assert_eq!(
                SerializeUtil::deserialize::<Version2>(&bytes).unwrap(),
                value
            );

            let options = EnvelopeOptions::new(9, 2).with_compatible_versions(1..=2);
            let bytes = SerializeUtil::serialize_with_options(&value, &options).unwrap();
            assert_eq!(
                SerializeUtil::deserialize_with_options::<Version2>(&bytes, &options).unwrap(),
                value
            );
        }
    }

    #[cfg(feature = "musli-storage")]
    #[test]
    fn storage_round_trip_uses_an_explicit_codec_id() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Stored {
            value: u32,
        }

        let options = EnvelopeOptions::new(1, 1);
        let value = Stored { value: 42 };
        let raw = MusliStorage::encode(&value).unwrap();
        assert_eq!(MusliStorage::decode::<Stored>(&raw).unwrap(), value);
        let mut output = Vec::new();
        MusliStorage::encode_into(&mut output, &value).unwrap();
        let mut fixed = [0_u8; 64];
        let len = MusliStorage::encode_to_slice(&mut fixed, &value).unwrap();
        assert_eq!(
            MusliStorage::decode::<Stored>(&fixed[..len]).unwrap(),
            value
        );
        let frame = MusliStorage::encode_frame(&value, &options).unwrap();
        MusliStorage::encode_frame_into(&mut output, &value, &options).unwrap();
        assert_eq!(output, frame);
        assert_eq!(
            MusliStorage::decode_frame::<Stored>(&frame, &options).unwrap(),
            value
        );
        assert_eq!(
            inspect_frame(&frame, &options).unwrap().metadata.codec,
            SerializationCodec::MusliStorage
        );
    }

    #[cfg(feature = "musli-packed")]
    #[test]
    fn packed_round_trip_requires_an_explicit_packed_model() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        #[musli(packed)]
        struct Packed {
            left: u32,
            right: u32,
        }

        let value = Packed { left: 1, right: 2 };
        let bytes = MusliPacked::encode(&value).unwrap();
        assert_eq!(MusliPacked::decode::<Packed>(&bytes).unwrap(), value);
        let mut output = Vec::new();
        MusliPacked::encode_into(&mut output, &value).unwrap();
        let mut fixed = [0_u8; 64];
        let len = MusliPacked::encode_to_slice(&mut fixed, &value).unwrap();
        assert_eq!(MusliPacked::decode::<Packed>(&fixed[..len]).unwrap(), value);
        let options = EnvelopeOptions::new(2, 1);
        let frame = MusliPacked::encode_frame(&value, &options).unwrap();
        MusliPacked::encode_frame_into(&mut output, &value, &options).unwrap();
        assert_eq!(output, frame);
        assert_eq!(
            MusliPacked::decode_frame::<Packed>(&frame, &options).unwrap(),
            value
        );
    }

    #[cfg(feature = "musli-descriptive")]
    #[test]
    fn descriptive_round_trip_is_available_for_migration_tools() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Described {
            name: String,
        }

        let value = Described {
            name: "HiTool".into(),
        };
        let bytes = MusliDescriptive::encode(&value).unwrap();
        assert_eq!(
            MusliDescriptive::decode::<Described>(&bytes).unwrap(),
            value
        );
        let mut output = Vec::new();
        MusliDescriptive::encode_into(&mut output, &value).unwrap();
        let mut fixed = [0_u8; 128];
        let len = MusliDescriptive::encode_to_slice(&mut fixed, &value).unwrap();
        assert_eq!(
            MusliDescriptive::decode::<Described>(&fixed[..len]).unwrap(),
            value
        );
        let options = EnvelopeOptions::new(3, 1);
        let frame = MusliDescriptive::encode_frame(&value, &options).unwrap();
        MusliDescriptive::encode_frame_into(&mut output, &value, &options).unwrap();
        assert_eq!(output, frame);
        assert_eq!(
            MusliDescriptive::decode_frame::<Described>(&frame, &options).unwrap(),
            value
        );
    }
}
