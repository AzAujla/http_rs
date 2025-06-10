use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    // 1xx Informational
    Continue,           // 100
    SwitchingProtocols, // 101
    Processing,         // 102
    EarlyHints,         // 103

    // 2xx Success
    Ok,                          // 200
    Created,                     // 201
    Accepted,                    // 202
    NonAuthoritativeInformation, // 203
    NoContent,                   // 204
    ResetContent,                // 205
    PartialContent,              // 206
    MultiStatus,                 // 207
    AlreadyReported,             // 208
    ImUsed,                      // 226

    // 3xx Redirection
    MultipleChoices,   // 300
    MovedPermanently,  // 301
    Found,             // 302
    SeeOther,          // 303
    NotModified,       // 304
    UseProxy,          // 305
    TemporaryRedirect, // 307
    PermanentRedirect, // 308

    // 4xx Client Error
    BadRequest,                  // 400
    Unauthorized,                // 401
    PaymentRequired,             // 402
    Forbidden,                   // 403
    NotFound,                    // 404
    MethodNotAllowed,            // 405
    NotAcceptable,               // 406
    ProxyAuthenticationRequired, // 407
    RequestTimeout,              // 408
    Conflict,                    // 409
    Gone,                        // 410
    LengthRequired,              // 411
    PreconditionFailed,          // 412
    PayloadTooLarge,             // 413
    UriTooLong,                  // 414
    UnsupportedMediaType,        // 415
    RangeNotSatisfiable,         // 416
    ExpectationFailed,           // 417
    ImATeapot,                   // 418
    MisdirectedRequest,          // 421
    UnprocessableEntity,         // 422
    Locked,                      // 423
    FailedDependency,            // 424
    TooEarly,                    // 425
    UpgradeRequired,             // 426
    PreconditionRequired,        // 428
    TooManyRequests,             // 429
    RequestHeaderFieldsTooLarge, // 431
    UnavailableForLegalReasons,  // 451

    // 5xx Server Error
    InternalServerError,           // 500
    NotImplemented,                // 501
    BadGateway,                    // 502
    ServiceUnavailable,            // 503
    GatewayTimeout,                // 504
    HttpVersionNotSupported,       // 505
    VariantAlsoNegotiates,         // 506
    InsufficientStorage,           // 507
    LoopDetected,                  // 508
    NotExtended,                   // 510
    NetworkAuthenticationRequired, // 511
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.code(), self.reason())
    }
}

impl FromStr for HttpStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        for status in HttpStatus::all() {
            if status.reason().to_ascii_lowercase() == s {
                return Ok(status.to_owned());
            }
        }
        Err(())
    }
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        use HttpStatus::*;
        match self {
            Continue => 100,
            SwitchingProtocols => 101,
            Processing => 102,
            EarlyHints => 103,

            Ok => 200,
            Created => 201,
            Accepted => 202,
            NonAuthoritativeInformation => 203,
            NoContent => 204,
            ResetContent => 205,
            PartialContent => 206,
            MultiStatus => 207,
            AlreadyReported => 208,
            ImUsed => 226,

            MultipleChoices => 300,
            MovedPermanently => 301,
            Found => 302,
            SeeOther => 303,
            NotModified => 304,
            UseProxy => 305,
            TemporaryRedirect => 307,
            PermanentRedirect => 308,

            BadRequest => 400,
            Unauthorized => 401,
            PaymentRequired => 402,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            NotAcceptable => 406,
            ProxyAuthenticationRequired => 407,
            RequestTimeout => 408,
            Conflict => 409,
            Gone => 410,
            LengthRequired => 411,
            PreconditionFailed => 412,
            PayloadTooLarge => 413,
            UriTooLong => 414,
            UnsupportedMediaType => 415,
            RangeNotSatisfiable => 416,
            ExpectationFailed => 417,
            ImATeapot => 418,
            MisdirectedRequest => 421,
            UnprocessableEntity => 422,
            Locked => 423,
            FailedDependency => 424,
            TooEarly => 425,
            UpgradeRequired => 426,
            PreconditionRequired => 428,
            TooManyRequests => 429,
            RequestHeaderFieldsTooLarge => 431,
            UnavailableForLegalReasons => 451,

            InternalServerError => 500,
            NotImplemented => 501,
            BadGateway => 502,
            ServiceUnavailable => 503,
            GatewayTimeout => 504,
            HttpVersionNotSupported => 505,
            VariantAlsoNegotiates => 506,
            InsufficientStorage => 507,
            LoopDetected => 508,
            NotExtended => 510,
            NetworkAuthenticationRequired => 511,
        }
    }

    pub fn reason(&self) -> &'static str {
        use HttpStatus::*;
        match self {
            Continue => "Continue",
            SwitchingProtocols => "Switching Protocols",
            Processing => "Processing",
            EarlyHints => "Early Hints",

            Ok => "OK",
            Created => "Created",
            Accepted => "Accepted",
            NonAuthoritativeInformation => "Non-Authoritative Information",
            NoContent => "No Content",
            ResetContent => "Reset Content",
            PartialContent => "Partial Content",
            MultiStatus => "Multi-Status",
            AlreadyReported => "Already Reported",
            ImUsed => "IM Used",

            MultipleChoices => "Multiple Choices",
            MovedPermanently => "Moved Permanently",
            Found => "Found",
            SeeOther => "See Other",
            NotModified => "Not Modified",
            UseProxy => "Use Proxy",
            TemporaryRedirect => "Temporary Redirect",
            PermanentRedirect => "Permanent Redirect",

            BadRequest => "Bad Request",
            Unauthorized => "Unauthorized",
            PaymentRequired => "Payment Required",
            Forbidden => "Forbidden",
            NotFound => "Not Found",
            MethodNotAllowed => "Method Not Allowed",
            NotAcceptable => "Not Acceptable",
            ProxyAuthenticationRequired => "Proxy Authentication Required",
            RequestTimeout => "Request Timeout",
            Conflict => "Conflict",
            Gone => "Gone",
            LengthRequired => "Length Required",
            PreconditionFailed => "Precondition Failed",
            PayloadTooLarge => "Payload Too Large",
            UriTooLong => "URI Too Long",
            UnsupportedMediaType => "Unsupported Media Type",
            RangeNotSatisfiable => "Range Not Satisfiable",
            ExpectationFailed => "Expectation Failed",
            ImATeapot => "I'm a teapot",
            MisdirectedRequest => "Misdirected Request",
            UnprocessableEntity => "Unprocessable Entity",
            Locked => "Locked",
            FailedDependency => "Failed Dependency",
            TooEarly => "Too Early",
            UpgradeRequired => "Upgrade Required",
            PreconditionRequired => "Precondition Required",
            TooManyRequests => "Too Many Requests",
            RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            InternalServerError => "Internal Server Error",
            NotImplemented => "Not Implemented",
            BadGateway => "Bad Gateway",
            ServiceUnavailable => "Service Unavailable",
            GatewayTimeout => "Gateway Timeout",
            HttpVersionNotSupported => "HTTP Version Not Supported",
            VariantAlsoNegotiates => "Variant Also Negotiates",
            InsufficientStorage => "Insufficient Storage",
            LoopDetected => "Loop Detected",
            NotExtended => "Not Extended",
            NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }

    pub fn all() -> &'static [HttpStatus] {
        use HttpStatus::*;
        const ALL: &[HttpStatus] = &[
            Continue,
            SwitchingProtocols,
            Processing,
            EarlyHints,
            Ok,
            Created,
            Accepted,
            NonAuthoritativeInformation,
            NoContent,
            ResetContent,
            PartialContent,
            MultiStatus,
            AlreadyReported,
            ImUsed,
            MultipleChoices,
            MovedPermanently,
            Found,
            SeeOther,
            NotModified,
            UseProxy,
            TemporaryRedirect,
            PermanentRedirect,
            BadRequest,
            Unauthorized,
            PaymentRequired,
            Forbidden,
            NotFound,
            MethodNotAllowed,
            NotAcceptable,
            ProxyAuthenticationRequired,
            RequestTimeout,
            Conflict,
            Gone,
            LengthRequired,
            PreconditionFailed,
            PayloadTooLarge,
            UriTooLong,
            UnsupportedMediaType,
            RangeNotSatisfiable,
            ExpectationFailed,
            ImATeapot,
            MisdirectedRequest,
            UnprocessableEntity,
            Locked,
            FailedDependency,
            TooEarly,
            UpgradeRequired,
            PreconditionRequired,
            TooManyRequests,
            RequestHeaderFieldsTooLarge,
            UnavailableForLegalReasons,
            InternalServerError,
            NotImplemented,
            BadGateway,
            ServiceUnavailable,
            GatewayTimeout,
            HttpVersionNotSupported,
            VariantAlsoNegotiates,
            InsufficientStorage,
            LoopDetected,
            NotExtended,
            NetworkAuthenticationRequired,
        ];
        ALL
    }

    pub fn from_u16(code: u16) -> Option<HttpStatus> {
        Self::all().iter().copied().find(|s| s.code() == code)
    }
}
