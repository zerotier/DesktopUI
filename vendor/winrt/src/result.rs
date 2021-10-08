use std::fmt;

/// Re-export from WinAPI crate
pub type HRESULT = ::w::um::winnt::HRESULT;

// TODO: add more codes from https://msdn.microsoft.com/en-us/library/windows/desktop/dd542643(v=vs.85).aspx, especially the `RO_`-prefixed

/// Represents an error as a result of a Windows Runtime method call.
#[derive(PartialEq, Eq)]
pub enum Error {
    OperationAborted,
    AccessDenied, // UnauthorizedAccessException in .NET (https://docs.microsoft.com/en-us/dotnet/standard/exceptions/handling-com-interop-exceptions)
    UnspecifiedFailure,
    InvalidHandle,
    InvalidArgument, // ArgumentException in .NET (https://docs.microsoft.com/en-us/dotnet/framework/interop/how-to-map-hresults-and-exceptions)
    NoSuchInterface,
    NotImplemented, // NotImplementedException in .NET (https://docs.microsoft.com/en-us/dotnet/framework/interop/how-to-map-hresults-and-exceptions)
    OutOfMemory, // OutOfMemoryException in .NET (https://docs.microsoft.com/en-us/dotnet/framework/interop/how-to-map-hresults-and-exceptions)
    InvalidPointer,
    UnexpectedFailure,
    OutOfBounds,
    IllegalMethodCall,
    ObjectClosed, // ObjectDisposedException in .NET
    Other(HRESULT)
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self { 
            OperationAborted => write!(f, "E_ABORT"),
            AccessDenied => write!(f, "E_ACCESSDENIED"),
            UnspecifiedFailure => write!(f, "E_FAIL"),
            InvalidHandle => write!(f, "E_HANDLE"),
            InvalidArgument => write!(f, "E_INVALIDARG"),
            NoSuchInterface => write!(f, "E_NOINTERFACE"),
            NotImplemented => write!(f, "E_NOTIMPL"),
            OutOfMemory => write!(f, "E_OUTOFMEMORY"),
            InvalidPointer => write!(f, "E_POINTER"),
            UnexpectedFailure => write!(f, "E_UNEXPECTED"),
            OutOfBounds => write!(f, "E_BOUNDS"),
            IllegalMethodCall => write!(f, "E_ILLEGAL_METHOD_CALL"),
            ObjectClosed => write!(f, "RO_E_CLOSED"),
            Other(hr) => write!(f, "0x{:X}", hr as u32),
        }
    }
}

impl Error {
    #[inline]
    pub fn from_hresult(hr: HRESULT) -> Error {
        use Error::*;
        use ::w::shared::winerror::*;

        match hr {
            E_ABORT => OperationAborted,
            E_ACCESSDENIED => AccessDenied,
            E_FAIL => UnspecifiedFailure,
            E_HANDLE => InvalidHandle,
            E_INVALIDARG => InvalidArgument,
            E_NOINTERFACE => NoSuchInterface,
            E_NOTIMPL => NotImplemented,
            E_OUTOFMEMORY => OutOfMemory,
            E_POINTER => InvalidPointer,
            E_UNEXPECTED => UnexpectedFailure,
            E_BOUNDS => OutOfBounds,
            E_ILLEGAL_METHOD_CALL => IllegalMethodCall,
            RO_E_CLOSED => ObjectClosed,
            _ => Other(hr)
        }
    }

    #[inline]
    pub fn as_hresult(&self) -> HRESULT {
        use Error::*;
        use ::w::shared::winerror::*;

        match *self { 
            OperationAborted => E_ABORT,
            AccessDenied => E_ACCESSDENIED,
            UnspecifiedFailure => E_FAIL,
            InvalidHandle => E_HANDLE,
            InvalidArgument => E_INVALIDARG,
            NoSuchInterface => E_NOINTERFACE,
            NotImplemented => E_NOTIMPL,
            OutOfMemory => E_OUTOFMEMORY,
            InvalidPointer => E_POINTER,
            UnexpectedFailure => E_UNEXPECTED,
            OutOfBounds => E_BOUNDS,
            IllegalMethodCall => E_ILLEGAL_METHOD_CALL,
            ObjectClosed => RO_E_CLOSED,
            Other(hr) => hr,
        }
    }
}

/// A specialized `Result` type for Windows Runtime method calls.
pub type Result<T> = ::std::result::Result<T, Error>;