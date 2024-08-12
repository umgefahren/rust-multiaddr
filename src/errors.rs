use alloc::string::{FromUtf8Error, String};
use core::{error, fmt, net, num, str};
use unsigned_varint::decode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum ParsingError {
    Multihash(multihash::Error),
    Multibase(multibase::Error),
    AddrParse(net::AddrParseError),
    ParseInt(num::ParseIntError),
    FromUtf8(FromUtf8Error),
    Utf8(str::Utf8Error),
    PeerId(libp2p_identity::ParseError),
}

impl From<multihash::Error> for ParsingError {
    fn from(err: multihash::Error) -> ParsingError {
        ParsingError::Multihash(err)
    }
}

impl From<multibase::Error> for ParsingError {
    fn from(err: multibase::Error) -> ParsingError {
        ParsingError::Multibase(err)
    }
}

impl From<net::AddrParseError> for ParsingError {
    fn from(err: net::AddrParseError) -> ParsingError {
        ParsingError::AddrParse(err)
    }
}

impl From<num::ParseIntError> for ParsingError {
    fn from(err: num::ParseIntError) -> ParsingError {
        ParsingError::ParseInt(err)
    }
}

impl From<FromUtf8Error> for ParsingError {
    fn from(err: FromUtf8Error) -> ParsingError {
        ParsingError::FromUtf8(err)
    }
}

impl From<str::Utf8Error> for ParsingError {
    fn from(err: str::Utf8Error) -> ParsingError {
        ParsingError::Utf8(err)
    }
}

impl From<libp2p_identity::ParseError> for ParsingError {
    fn from(err: libp2p_identity::ParseError) -> ParsingError {
        ParsingError::PeerId(err)
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::Multihash(e) => write!(f, "failed to parse multihash: {e}"),
            ParsingError::Multibase(e) => write!(f, "failed to parse multibase: {e}"),
            ParsingError::AddrParse(e) => write!(f, "failed to parse address: {e}"),
            ParsingError::ParseInt(e) => write!(f, "failed to parse integer: {e}"),
            ParsingError::Utf8(e) => write!(f, "failed to parse utf8: {e}"),
            ParsingError::FromUtf8(e) => write!(f, "failed to parse utf8: {e}"),
            ParsingError::PeerId(e) => write!(f, "failed to parse peer id: {e:?}"),
        }
    }
}

impl error::Error for ParsingError {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParsingError::AddrParse(e) => Some(e),
            ParsingError::ParseInt(e) => Some(e),
            ParsingError::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

/// Error types
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    DataLessThanLen,
    InvalidMultiaddr,
    InvalidProtocolString,
    InvalidUvar(decode::Error),
    ParsingError(ParsingError),
    UnknownProtocolId(u32),
    UnknownProtocolString(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DataLessThanLen => f.write_str("we have less data than indicated by length"),
            Error::InvalidMultiaddr => f.write_str("invalid multiaddr"),
            Error::InvalidProtocolString => f.write_str("invalid protocol string"),
            Error::InvalidUvar(e) => write!(f, "failed to decode unsigned varint: {e}"),
            Error::ParsingError(e) => write!(f, "failed to parse: {e}"),
            Error::UnknownProtocolId(id) => write!(f, "unknown protocol id: {id}"),
            Error::UnknownProtocolString(string) => {
                write!(f, "unknown protocol string: {string}")
            }
        }
    }
}

impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ParsingError(e) => e.source(),
            _ => None,
        }
    }
}

impl From<multihash::Error> for Error {
    fn from(err: multihash::Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<multibase::Error> for Error {
    fn from(err: multibase::Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::ParsingError(err.into())
    }
}

impl From<decode::Error> for Error {
    fn from(e: decode::Error) -> Error {
        Error::InvalidUvar(e)
    }
}
