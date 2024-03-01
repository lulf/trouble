use crate::byte_reader::ByteReader;
use core::convert::TryInto;

pub const ATT_READ_BY_GROUP_TYPE_REQUEST_OPCODE: u8 = 0x10;
pub const ATT_READ_BY_GROUP_TYPE_RESPONSE_OPCODE: u8 = 0x11;
pub const ATT_ERROR_RESPONSE_OPCODE: u8 = 0x01;
pub const ATT_READ_BY_TYPE_REQUEST_OPCODE: u8 = 0x08;
pub const ATT_READ_BY_TYPE_RESPONSE_OPCODE: u8 = 0x09;
pub const ATT_READ_REQUEST_OPCODE: u8 = 0x0a;
pub const ATT_READ_RESPONSE_OPCODE: u8 = 0x0b;
pub const ATT_WRITE_REQUEST_OPCODE: u8 = 0x12;
pub const ATT_WRITE_CMD_OPCODE: u8 = 0x52;
pub const ATT_WRITE_RESPONSE_OPCODE: u8 = 0x13;
pub const ATT_EXCHANGE_MTU_REQUEST_OPCODE: u8 = 0x02;
pub const ATT_EXCHANGE_MTU_RESPONSE_OPCODE: u8 = 0x03;
pub const ATT_FIND_BY_TYPE_VALUE_REQUEST_OPCODE: u8 = 0x06;
pub const ATT_FIND_BY_TYPE_VALUE_RESPONSE_OPCODE: u8 = 0x07;
pub const ATT_FIND_INFORMATION_REQ_OPCODE: u8 = 0x04;
pub const ATT_FIND_INFORMATION_RSP_OPCODE: u8 = 0x05;
pub const ATT_PREPARE_WRITE_REQ_OPCODE: u8 = 0x16;
pub const ATT_PREPARE_WRITE_RESP_OPCODE: u8 = 0x17;
pub const ATT_EXECUTE_WRITE_REQ_OPCODE: u8 = 0x18;
pub const ATT_EXECUTE_WRITE_RESP_OPCODE: u8 = 0x19;
pub const ATT_READ_BLOB_REQ_OPCODE: u8 = 0x0c;
pub const ATT_READ_BLOB_RESP_OPCODE: u8 = 0x0d;
pub const ATT_HANDLE_VALUE_NTF_OPTCODE: u8 = 0x1b;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Uuid {
    Uuid16([u8; 2]),
    Uuid128([u8; 16]),
}

impl Uuid {
    pub const fn new_short(val: u16) -> Self {
        Self::Uuid16(val.to_le_bytes())
    }

    pub const fn new_long(val: [u8; 16]) -> Self {
        Self::Uuid128(val)
    }

    pub fn bytes(&self, data: &mut [u8]) {
        match self {
            Uuid::Uuid16(uuid) => data.copy_from_slice(uuid),
            Uuid::Uuid128(uuid) => data.copy_from_slice(uuid),
        }
    }

    pub fn get_type(&self) -> u8 {
        match self {
            Uuid::Uuid16(_) => 0x01,
            Uuid::Uuid128(_) => 0x02,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Uuid::Uuid16(_) => 6,
            Uuid::Uuid128(_) => 20,
        }
    }

    pub fn as_raw(&self) -> &[u8] {
        match self {
            Uuid::Uuid16(uuid) => uuid,
            Uuid::Uuid128(uuid) => uuid,
        }
    }
}

impl From<u16> for Uuid {
    fn from(data: u16) -> Self {
        Uuid::Uuid16(data.to_le_bytes())
    }
}

impl From<&[u8]> for Uuid {
    fn from(data: &[u8]) -> Self {
        match data.len() {
            2 => Uuid::Uuid16(data.try_into().unwrap()),
            16 => {
                let bytes: [u8; 16] = data.try_into().unwrap();
                Uuid::Uuid128(bytes)
            }
            _ => panic!(),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum AttErrorCode {
    /// Attempted to use an `Handle` that isn't valid on this server.
    InvalidHandle = 0x01,
    /// Attribute isn't readable.
    ReadNotPermitted = 0x02,
    /// Attribute isn't writable.
    WriteNotPermitted = 0x03,
    /// Attribute PDU is invalid.
    InvalidPdu = 0x04,
    /// Authentication needed before attribute can be read/written.
    InsufficientAuthentication = 0x05,
    /// Server doesn't support this operation.
    RequestNotSupported = 0x06,
    /// Offset was past the end of the attribute.
    InvalidOffset = 0x07,
    /// Authorization needed before attribute can be read/written.
    InsufficientAuthorization = 0x08,
    /// Too many "prepare write" requests have been queued.
    PrepareQueueFull = 0x09,
    /// No attribute found within the specified attribute handle range.
    AttributeNotFound = 0x0A,
    /// Attribute can't be read/written using *Read Key Blob* request.
    AttributeNotLong = 0x0B,
    /// The encryption key in use is too weak to access an attribute.
    InsufficientEncryptionKeySize = 0x0C,
    /// Attribute value has an incorrect length for the operation.
    InvalidAttributeValueLength = 0x0D,
    /// Request has encountered an "unlikely" error and could not be completed.
    UnlikelyError = 0x0E,
    /// Attribute cannot be read/written without an encrypted connection.
    InsufficientEncryption = 0x0F,
    /// Attribute type is an invalid grouping attribute according to a higher-layer spec.
    UnsupportedGroupType = 0x10,
    /// Server didn't have enough resources to complete a request.
    InsufficientResources = 0x11,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Att<'d> {
    ReadByGroupTypeReq {
        start: u16,
        end: u16,
        group_type: Uuid,
    },
    ReadByTypeReq {
        start: u16,
        end: u16,
        attribute_type: Uuid,
    },
    ReadReq {
        handle: u16,
    },
    WriteReq {
        handle: u16,
        data: &'d [u8],
    },
    WriteCmd {
        handle: u16,
        data: &'d [u8],
    },
    ExchangeMtu {
        mtu: u16,
    },
    FindByTypeValue {
        start_handle: u16,
        end_handle: u16,
        att_type: u16,
        att_value: u16,
    },
    FindInformation {
        start_handle: u16,
        end_handle: u16,
    },
    PrepareWriteReq {
        handle: u16,
        offset: u16,
        value: &'d [u8],
    },
    ExecuteWriteReq {
        flags: u8,
    },
    ReadBlobReq {
        handle: u16,
        offset: u16,
    },
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum AttDecodeError {
    Other,
    UnknownOpcode(u8),
    UnexpectedPayload,
}

impl<'d> Att<'d> {
    pub fn decode(packet: &'d [u8]) -> Result<Att<'d>, AttDecodeError> {
        let mut r = ByteReader::new(packet);
        let opcode = r.read_u8();
        let payload = r.consume();

        match opcode {
            ATT_READ_BY_GROUP_TYPE_REQUEST_OPCODE => {
                let start_handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let end_handle = (payload[2] as u16) + ((payload[3] as u16) << 8);

                let group_type = if payload.len() == 6 {
                    Uuid::Uuid16([payload[4], payload[5]])
                } else if payload.len() == 20 {
                    let uuid = payload[4..21].try_into().map_err(|_| AttDecodeError::Other)?;
                    Uuid::Uuid128(uuid)
                } else {
                    return Err(AttDecodeError::UnexpectedPayload);
                };

                Ok(Self::ReadByGroupTypeReq {
                    start: start_handle,
                    end: end_handle,
                    group_type,
                })
            }
            ATT_READ_BY_TYPE_REQUEST_OPCODE => {
                let start_handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let end_handle = (payload[2] as u16) + ((payload[3] as u16) << 8);

                let attribute_type = if payload.len() == 6 {
                    Uuid::Uuid16([payload[4], payload[5]])
                } else if payload.len() == 20 {
                    let uuid = payload[4..21].try_into().map_err(|_| AttDecodeError::Other)?;
                    Uuid::Uuid128(uuid)
                } else {
                    return Err(AttDecodeError::UnexpectedPayload);
                };

                Ok(Self::ReadByTypeReq {
                    start: start_handle,
                    end: end_handle,
                    attribute_type,
                })
            }
            ATT_READ_REQUEST_OPCODE => {
                let handle = (payload[0] as u16) + ((payload[1] as u16) << 8);

                Ok(Self::ReadReq { handle })
            }
            ATT_WRITE_REQUEST_OPCODE => {
                let handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let data = &payload[2..];

                Ok(Self::WriteReq { handle, data })
            }
            ATT_WRITE_CMD_OPCODE => {
                let handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let data = &payload[2..];

                Ok(Self::WriteCmd { handle, data })
            }
            ATT_EXCHANGE_MTU_REQUEST_OPCODE => {
                let mtu = (payload[0] as u16) + ((payload[1] as u16) << 8);
                Ok(Self::ExchangeMtu { mtu })
            }
            ATT_FIND_BY_TYPE_VALUE_REQUEST_OPCODE => {
                let start_handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let end_handle = (payload[2] as u16) + ((payload[3] as u16) << 8);
                let att_type = (payload[4] as u16) + ((payload[5] as u16) << 8);
                let att_value = (payload[6] as u16) + ((payload[7] as u16) << 8); // only U16 supported here

                Ok(Self::FindByTypeValue {
                    start_handle,
                    end_handle,
                    att_type,
                    att_value,
                })
            }
            ATT_FIND_INFORMATION_REQ_OPCODE => {
                let start_handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let end_handle = (payload[2] as u16) + ((payload[3] as u16) << 8);

                Ok(Self::FindInformation {
                    start_handle,
                    end_handle,
                })
            }
            ATT_PREPARE_WRITE_REQ_OPCODE => {
                let handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let offset = (payload[2] as u16) + ((payload[3] as u16) << 8);
                Ok(Self::PrepareWriteReq {
                    handle,
                    offset,
                    value: &payload[4..],
                })
            }
            ATT_EXECUTE_WRITE_REQ_OPCODE => {
                let flags = payload[0];
                Ok(Self::ExecuteWriteReq { flags })
            }
            ATT_READ_BLOB_REQ_OPCODE => {
                let handle = (payload[0] as u16) + ((payload[1] as u16) << 8);
                let offset = (payload[2] as u16) + ((payload[3] as u16) << 8);
                Ok(Self::ReadBlobReq { handle, offset })
            }
            _ => Err(AttDecodeError::UnknownOpcode(opcode)),
        }
    }
}