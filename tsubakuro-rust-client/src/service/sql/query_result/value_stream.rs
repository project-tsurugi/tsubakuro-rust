use std::{collections::VecDeque, time::Duration};

use crate::{client_error, error::TgError};

use super::{byte_stream::ResultSetByteStream, variant::Base128Variant};

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/sql/impl/ValueInputBackedRelationCursor.java
#[derive(Debug)]
pub(crate) enum EntryKind {
    TopLevelRow,
    _RowValue,
    _ArrayValue,
}

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/sql/io/EntryType.java
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum EntryType {
    Nothing,

    /// Pseudo data type of end of relation mark.
    EndOfContents,

    /// Represents value is absent.
    Null,

    /// 64-bit signed integer.
    /// This represents {@code INT4}, {@code INT8} or {@code BOOLEAN}.
    Int,

    /// Fixed 32-bit floating point number.
    Float4,

    /// Fixed 64-bit floating point number.
    Float8,

    /// Fixed 128-bit floating point decimal.
    Ddecimal,

    /// Variable length character sequence.
    Character,

    /// Variable length octet sequence.
    Octet,

    /// Variable length bit sequence.
    Bit,

    /// Date value.
    Date,

    /// Time of day value.
    TimeOfDay,

    /// Time-point value.
    TimePoint,

    /// Time of day with timezone value.
    TimeOfDayWithTimeZone,

    /// Time-point with timezone value.
    TimePointWithTimeZone,

    /// Date-time interval value.
    DatetimeInterval,

    /// Rows.
    Row,

    /// Arrays.
    Array,

    /// Character large objects.
    _Clob,

    /// Binary large objects.
    _Blob,
}

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/sql/io/Constants.java
const HEADER_EMBED_POSITIVE_INT: i32 = 0x00;

const HEADER_EMBED_CHARACTER: i32 = 0x40;

const HEADER_EMBED_ROW: i32 = 0x80;

const HEADER_EMBED_ARRAY: i32 = 0xa0;

const HEADER_EMBED_NEGATIVE_INT: i32 = 0xc0;

const HEADER_EMBED_OCTET: i32 = 0xd0;

const HEADER_EMBED_BIT: i32 = 0xe0;

const HEADER_UNKNOWN: i32 = 0xe8;

const HEADER_INT: i32 = 0xe9;

// const HEADER_FLOAT4: i32 = 0xea;

// const HEADER_FLOAT8: i32 = 0xeb;

// const HEADER_DECIMAL_COMPACT: i32 = 0xec;

// const HEADER_DECIMAL: i32 = 0xed;

// const HEADER_TIME_OF_DAY_WITH_TIME_ZONE: i32 = 0xee;

// const HEADER_TIME_POINT_WITH_TIME_ZONE: i32 = 0xef;

const HEADER_CHARACTER: i32 = 0xf0;

// const HEADER_OCTET: i32 = 0xf1;

// const HEADER_BIT: i32 = 0xf2;

// const HEADER_DATE: i32 = 0xf3;

// const HEADER_TIME_OF_DAY: i32 = 0xf4;

// const HEADER_TIME_POINT: i32 = 0xf5;

// const HEADER_DATETIME_INTERVAL: i32 = 0xf6;

// const HEADER_RESERVED_F7: i32 = 0xf7;

const HEADER_ROW: i32 = 0xf8;

// const HEADER_ARRAY: i32 = 0xf9;

// const HEADER_CLOB: i32 = 0xfa;

// const HEADER_BLOB: i32 = 0xfb;

// const HEADER_RESERVED_FC: i32 = 0xfc;

// const HEADER_RESERVED_FD: i32 = 0xfd;

// const HEADER_END_OF_CONTENTS: i32 = 0xfe;

// const HEADER_RESERVED_FF: i32 = 0xff;

const MASK_EMBED_POSITIVE_INT: i32 = 0x3f;

const MASK_EMBED_CHARACTER: i32 = 0x3f;

const MASK_EMBED_ROW: i32 = 0x1f;

const MASK_EMBED_ARRAY: i32 = 0x1f;

const MASK_EMBED_NEGATIVE_INT: i32 = 0x0f;

const MASK_EMBED_OCTET: i32 = 0x0f;

const MASK_EMBED_BIT: i32 = 0x07;

const MIN_EMBED_POSITIVE_INT_VALUE: i32 = 0x00;

// const MAX_EMBED_POSITIVE_INT_VALUE: i32 = MASK_EMBED_POSITIVE_INT + MIN_EMBED_POSITIVE_INT_VALUE;

const MIN_EMBED_NEGATIVE_INT_VALUE: i32 = -(MASK_EMBED_NEGATIVE_INT + 1);

// const MAX_EMBED_NEGATIVE_INT_VALUE: u8 = 0;

const MIN_EMBED_CHARACTER_SIZE: i32 = 0x01;

// const MAX_EMBED_CHARACTER_SIZE: i32 = MASK_EMBED_CHARACTER + MIN_EMBED_CHARACTER_SIZE;

// const MIN_EMBED_OCTET_SIZE: i32 = 0x01;

// const MAX_EMBED_OCTET_SIZE: i32 = MASK_EMBED_OCTET + MIN_EMBED_OCTET_SIZE;

// const MIN_EMBED_BIT_SIZE: i32 = 0x01;

// const MAX_EMBED_BIT_SIZE: i32 = MASK_EMBED_BIT + MIN_EMBED_BIT_SIZE;

const MIN_EMBED_ROW_SIZE: i32 = 0x01;

// const MAX_EMBED_ROW_SIZE: i32 = MASK_EMBED_ROW + MIN_EMBED_ROW_SIZE;

// const MIN_EMBED_ARRAY_SIZE: i32 = 0x01;

// const MAX_EMBED_ARRAY_SIZE: i32 = MASK_EMBED_ARRAY + MIN_EMBED_ARRAY_SIZE;

// public static final BigInteger MIN_DECIMAL_COMPACT_COEFFICIENT = BigInteger.valueOf(Long.MIN_VALUE);

// public static final BigInteger MAX_DECIMAL_COMPACT_COEFFICIENT = BigInteger.valueOf(Long.MAX_VALUE);

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/sql/io/StreamBackedValueInput.java
const INDEPENDENT_ENTRY_TYPE: [EntryType; 24] = [
    // 0xe8
    EntryType::Null,
    // 0xe9
    EntryType::Int,
    // 0xea
    EntryType::Float4,
    // 0xeb
    EntryType::Float8,
    // 0xec
    EntryType::Ddecimal,
    // 0xed
    EntryType::Ddecimal,
    // 0xee
    EntryType::TimeOfDayWithTimeZone,
    // 0xef
    EntryType::TimePointWithTimeZone,
    // 0xf0
    EntryType::Character,
    // 0xf1
    EntryType::Octet,
    // 0xf2
    EntryType::Bit,
    // 0xf3
    EntryType::Date,
    // 0xf4
    EntryType::TimeOfDay,
    // 0xf5
    EntryType::TimePoint,
    // 0xf6
    EntryType::DatetimeInterval,
    // 0xf7
    EntryType::Nothing,
    // 0xf8
    EntryType::Row,
    // 0xf9
    EntryType::Array,
    // 0xfa
    EntryType::Nothing, // EntryType.CLOB,
    // 0xfb
    EntryType::Nothing, // EntryType.BLOB,
    // 0xfc
    EntryType::Nothing,
    // 0xfd
    EntryType::Nothing,
    // 0xfe
    EntryType::EndOfContents,
    // 0xff
    EntryType::Nothing,
];

const HEADER_HARD_EOF: i32 = -1;
const HEADER_UNGAINED: i32 = -2;
const OFFSET_INDEPENDENT_ENTRY_TYPE: i32 = -(HEADER_UNKNOWN as i32);

#[derive(Debug)]
pub(crate) struct ResultSetValueStream {
    byte_stream: ResultSetByteStream,
    saw_eof: bool,
    current_entry_type: EntryType,
    current_header_category: i32,
    current_header_payload: i32,
    kind_stack: VecDeque<(EntryKind, i32)>,
    current_column_type: EntryType,
}

impl ResultSetValueStream {
    pub(crate) fn new(byte_stream: ResultSetByteStream) -> ResultSetValueStream {
        ResultSetValueStream {
            byte_stream,
            saw_eof: false,
            current_entry_type: EntryType::Nothing,
            current_header_category: HEADER_UNGAINED,
            current_header_payload: 0,
            kind_stack: VecDeque::new(),
            current_column_type: EntryType::Nothing,
        }
    }

    pub async fn next_row(&mut self, timeout: Duration) -> Result<bool, TgError> {
        //TODO discardTopLevelRow()
        let entry_type = self.peek_entry_type(timeout).await?;
        match entry_type {
            EntryType::EndOfContents => Ok(false),
            EntryType::Row => {
                let elements = self.read_row_begin(timeout).await?;
                self.kind_stack_push(EntryKind::TopLevelRow, elements);
                Ok(true)
            }
            _ => Err(client_error!(format!(
                "next_row() illegal entry_type {entry_type:?}"
            ))),
        }
    }

    pub async fn next_column(&mut self, timeout: Duration) -> Result<bool, TgError> {
        if self.kind_stack.is_empty() {
            // trace!("+++---next_column()=false kind_stack is empty");
            return Ok(false);
        }

        //TODO discardCurrentColumnIfExists()

        let rest = self.kind_stack_get_top().unwrap().1;
        if rest == 0 {
            // trace!("+++---next_column()=false rest==0");
            return Ok(false);
        }

        self.current_column_type = self.peek_entry_type(timeout).await?;
        if self.current_column_type == EntryType::EndOfContents {
            return Err(client_error!("saw unexpected end of contents"));
        }

        // trace!("+++---next_column()=true");
        Ok(true)
    }

    pub(crate) fn is_null(&mut self) -> Result<bool, TgError> {
        Ok(self.current_column_type == EntryType::Null)
    }

    pub(crate) async fn fetch_int4_value(&mut self, timeout: Duration) -> Result<i32, TgError> {
        self.require_column_type(EntryType::Int)?;
        let value = self.read_int(timeout).await?;
        self.column_consumed();

        if i32::MIN as i64 <= value && value <= i32::MAX as i64 {
            Ok(value as i32)
        } else {
            Err(client_error!(format!(
                "value is out of range for 'i32': value={value}"
            )))
        }
    }

    async fn read_int(&mut self, timeout: Duration) -> Result<i64, TgError> {
        self.require_column_type(EntryType::Int)?;
        self.read_int_body(timeout).await
    }

    async fn read_int_body(&mut self, timeout: Duration) -> Result<i64, TgError> {
        let category = self.current_header_category;
        let payload = self.current_header_payload;
        self.clear_header_info();

        let value = match category {
            HEADER_EMBED_POSITIVE_INT => (payload + MIN_EMBED_POSITIVE_INT_VALUE) as i64,
            HEADER_EMBED_NEGATIVE_INT => (payload + MIN_EMBED_NEGATIVE_INT_VALUE) as i64,
            _ => {
                debug_assert_eq!(category, HEADER_INT);
                Base128Variant::read_signed(&mut self.byte_stream, timeout).await?
            }
        };

        Ok(value)
    }

    pub(crate) async fn fetch_int8_value(&mut self, timeout: Duration) -> Result<i64, TgError> {
        self.require_column_type(EntryType::Int)?;
        let value = self.read_int(timeout).await?;
        self.column_consumed();
        Ok(value)
    }

    pub(crate) async fn fetch_character_value(
        &mut self,
        timeout: Duration,
    ) -> Result<String, TgError> {
        self.require_column_type(EntryType::Character)?;
        let value = self.read_character(timeout).await?;
        // trace!("+++---take_character_value=[{}]", value);
        self.column_consumed();
        Ok(value)
    }

    async fn read_character(&mut self, timeout: Duration) -> Result<String, TgError> {
        let size = self.read_character_size(timeout).await?;

        let buffer = {
            if let Some(buffer) = self.byte_stream.read_all(size as usize, timeout).await? {
                buffer
            } else {
                return Err(client_error!("saw unexpected eof"));
            }
        };

        let s = String::from_utf8_lossy(&buffer);
        let s = String::from(s);
        Ok(s)
    }

    async fn read_character_size(&mut self, timeout: Duration) -> Result<i32, TgError> {
        let category = self.current_header_category;
        let payload = self.current_header_payload;
        self.clear_header_info();

        if category == HEADER_EMBED_CHARACTER {
            return Ok(payload + MIN_EMBED_CHARACTER_SIZE);
        }

        debug_assert_eq!(category, HEADER_CHARACTER);
        return self.read_size(timeout).await;
    }

    pub(crate) async fn read_row_begin(&mut self, timeout: Duration) -> Result<i32, TgError> {
        self.require(EntryType::Row)?;

        let category = self.current_header_category;
        let payload = self.current_header_payload;
        self.clear_header_info();

        if category == HEADER_EMBED_ROW {
            return Ok(payload + MIN_EMBED_ROW_SIZE);
        }

        debug_assert_eq!(category, HEADER_ROW);
        return self.read_size(timeout).await;
    }
}

impl ResultSetValueStream {
    fn clear_header_info(&mut self) {
        self.current_entry_type = EntryType::Nothing;
        self.current_header_category = HEADER_UNGAINED;
        self.current_header_payload = 0;
    }

    async fn peek_entry_type(&mut self, timeout: Duration) -> Result<EntryType, TgError> {
        // trace!("+++---peek_entry_type={:?}", self.current_entry_type);
        if self.current_entry_type == EntryType::Nothing {
            self.fetch_header(timeout).await?;
        }
        Ok(self.current_entry_type)
    }

    fn require(&self, expected: EntryType) -> Result<EntryType, TgError> {
        let found = self.current_entry_type;
        if found != expected {
            return Err(client_error!(format!(
                "inconsistent value type: '{found:?}' is found, but'{expected:?}' was expected"
            )));
        }
        Ok(found)
    }

    fn require_column_type(&self, expected: EntryType) -> Result<(), TgError> {
        let found = self.current_column_type;
        if found == EntryType::Nothing {
            return Err(client_error!("invoke .nextColumn() before fetch value"));
        }

        if found != expected {
            return Err(client_error!(format!("value is type is inconsistent: found '{found:?}' but expected one is ''{expected:?}''")));
        }
        Ok(())
    }

    fn column_consumed(&mut self) {
        self.current_column_type = EntryType::Nothing;
        let entry = self.kind_stack_get_top().unwrap();
        debug_assert!(entry.1 > 0);
        entry.1 -= 1;
    }

    fn kind_stack_push(&mut self, kind: EntryKind, rest: i32) {
        self.kind_stack.push_back((kind, rest));
    }

    fn kind_stack_get_top(&mut self) -> Option<&mut (EntryKind, i32)> {
        self.kind_stack.back_mut()
    }

    async fn fetch_header(&mut self, timeout: Duration) -> Result<(), TgError> {
        if self.saw_eof {
            self.current_entry_type = EntryType::EndOfContents;
            self.current_header_category = HEADER_HARD_EOF;
            self.current_header_payload = 0;
            return Ok(());
        }

        if let Some(c) = self.byte_stream.read_u8(timeout).await? {
            self.fetch_header_internal(c as i32)?;
        } else {
            self.saw_eof = true;
            self.current_entry_type = EntryType::EndOfContents;
        }
        Ok(())
    }

    fn fetch_header_internal(&mut self, c: i32) -> Result<(), TgError> {
        if c <= (HEADER_EMBED_POSITIVE_INT | MASK_EMBED_POSITIVE_INT) {
            self.current_entry_type = EntryType::Int;
            self.current_header_category = HEADER_EMBED_POSITIVE_INT;
            self.current_header_payload = c & MASK_EMBED_POSITIVE_INT;
        } else if c <= (HEADER_EMBED_CHARACTER | MASK_EMBED_CHARACTER) {
            self.current_entry_type = EntryType::Character;
            self.current_header_category = HEADER_EMBED_CHARACTER;
            self.current_header_payload = c & MASK_EMBED_CHARACTER;
        } else if c <= (HEADER_EMBED_ROW | MASK_EMBED_ROW) {
            self.current_entry_type = EntryType::Row;
            self.current_header_category = HEADER_EMBED_ROW;
            self.current_header_payload = c & MASK_EMBED_ROW;
        } else if c <= (HEADER_EMBED_ARRAY | MASK_EMBED_ARRAY) {
            self.current_entry_type = EntryType::Array;
            self.current_header_category = HEADER_EMBED_ARRAY;
            self.current_header_payload = c & MASK_EMBED_ARRAY;
        } else if c <= (HEADER_EMBED_NEGATIVE_INT | MASK_EMBED_NEGATIVE_INT) {
            self.current_entry_type = EntryType::Int;
            self.current_header_category = HEADER_EMBED_NEGATIVE_INT;
            self.current_header_payload = c & MASK_EMBED_NEGATIVE_INT;
        } else if c <= (HEADER_EMBED_OCTET | MASK_EMBED_OCTET) {
            self.current_entry_type = EntryType::Octet;
            self.current_header_category = HEADER_EMBED_OCTET;
            self.current_header_payload = c & MASK_EMBED_OCTET;
        } else if c <= (HEADER_EMBED_BIT | MASK_EMBED_BIT) {
            self.current_entry_type = EntryType::Bit;
            self.current_header_category = HEADER_EMBED_BIT;
            self.current_header_payload = c & MASK_EMBED_BIT;
        } else {
            let index = (c + OFFSET_INDEPENDENT_ENTRY_TYPE) as usize;
            let entry_type = INDEPENDENT_ENTRY_TYPE[index];
            if entry_type == EntryType::Nothing {
                return Err(client_error!(format!("unrecognized entry error {c}")));
            }
            self.current_entry_type = entry_type;
            self.current_header_category = c;
            self.current_header_payload = 0;
        }
        Ok(())
    }

    async fn read_size(&mut self, timeout: Duration) -> Result<i32, TgError> {
        let value = Base128Variant::read_unsigned(&mut self.byte_stream, timeout).await?;
        if value < 0 || value > (i32::MAX as i64) {
            return Err(client_error!(format!("saw unsupported size {value}")));
        }
        Ok(value as i32)
    }
}
