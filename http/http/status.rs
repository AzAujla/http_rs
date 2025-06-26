use std::fmt::Display;

/*
    Generated using :

    Array.from(new Set(Array.from(document.querySelectorAll('code'))
    .filter((d) => /^\d+$/.test(d.innerHTML.split(' ')[0]))
    .map(a => a.innerHTML)))
    .map(a => a.split(/ (.+)/).slice(0))
    .map(a => [a[0], a[1]])
    .sort((a, b) => parseInt(a[0]) - parseInt(b[0]))
    .filter(a => a[1]!=undefined)
    .map(([a, b]) => `${a} => Self::${b.split(' ').join('').split('-').join('').split('\'').join('')},`)

    at https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
*/

#[derive(Clone, Copy, Debug)]
pub enum HttpStatusCode {
    Continue,                      //100
    SwitchingProtocols,            //101
    Processing,                    //102
    EarlyHints,                    //103
    OK,                            //200
    Created,                       //201
    Accepted,                      //202
    NonAuthoritativeInformation,   //203
    NoContent,                     //204
    ResetContent,                  //205
    PartialContent,                //206
    MultiStatus,                   //207
    AlreadyReported,               //208
    IMUsed,                        //226
    MultipleChoices,               //300
    MovedPermanently,              //301
    Found,                         //302
    SeeOther,                      //303
    NotModified,                   //304
    UseProxy,                      //305
    Unused,                        //306
    TemporaryRedirect,             //307
    PermanentRedirect,             //308
    BadRequest,                    //400
    Unauthorized,                  //401
    PaymentRequired,               //402
    Forbidden,                     //403
    NotFound,                      //404
    MethodNotAllowed,              //405
    NotAcceptable,                 //406
    ProxyAuthenticationRequired,   //407
    RequestTimeout,                //408
    Conflict,                      //409
    Gone,                          //410
    LengthRequired,                //411
    PreconditionFailed,            //412
    ContentTooLarge,               //413
    URITooLong,                    //414
    UnsupportedMediaType,          //415
    RangeNotSatisfiable,           //416
    ExpectationFailed,             //417
    Imateapot,                     //418
    MisdirectedRequest,            //421
    UnprocessableContent,          //422
    Locked,                        //423
    FailedDependency,              //424
    TooEarly,                      //425
    UpgradeRequired,               //426
    PreconditionRequired,          //428
    TooManyRequests,               //429
    RequestHeaderFieldsTooLarge,   //431
    UnavailableForLegalReasons,    //451
    InternalServerError,           //500
    NotImplemented,                //501
    BadGateway,                    //502
    ServiceUnavailable,            //503
    GatewayTimeout,                //504
    HTTPVersionNotSupported,       //505
    VariantAlsoNegotiates,         //506
    InsufficientStorage,           //507
    LoopDetected,                  //508
    NotExtended,                   //510
    NetworkAuthenticationRequired, //511
}

impl Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Continue => write!(f, "100 Continue"),
            Self::SwitchingProtocols => write!(f, "101 Switching Protocols"),
            Self::Processing => write!(f, "102 Processing"),
            Self::EarlyHints => write!(f, "103 Early Hints"),
            Self::OK => write!(f, "200 OK"),
            Self::Created => write!(f, "201 Created"),
            Self::Accepted => write!(f, "202 Accepted"),
            Self::NonAuthoritativeInformation => write!(f, "203 Non-Authoritative Information"),
            Self::NoContent => write!(f, "204 No Content"),
            Self::ResetContent => write!(f, "205 Reset Content"),
            Self::PartialContent => write!(f, "206 Partial Content"),
            Self::MultiStatus => write!(f, "207 Multi-Status"),
            Self::AlreadyReported => write!(f, "208 Already Reported"),
            Self::IMUsed => write!(f, "226 IM Used"),
            Self::MultipleChoices => write!(f, "300 Multiple Choices"),
            Self::MovedPermanently => write!(f, "301 Moved Permanently"),
            Self::Found => write!(f, "302 Found"),
            Self::SeeOther => write!(f, "303 See Other"),
            Self::NotModified => write!(f, "304 Not Modified"),
            Self::UseProxy => write!(f, "305 Use Proxy"),
            Self::Unused => write!(f, "306 unused"),
            Self::TemporaryRedirect => write!(f, "307 Temporary Redirect"),
            Self::PermanentRedirect => write!(f, "308 Permanent Redirect"),
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::Unauthorized => write!(f, "401 Unauthorized"),
            Self::PaymentRequired => write!(f, "402 Payment Required"),
            Self::Forbidden => write!(f, "403 Forbidden"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            Self::NotAcceptable => write!(f, "406 Not Acceptable"),
            Self::ProxyAuthenticationRequired => write!(f, "407 Proxy Authentication Required"),
            Self::RequestTimeout => write!(f, "408 Request Timeout"),
            Self::Conflict => write!(f, "409 Conflict"),
            Self::Gone => write!(f, "410 Gone"),
            Self::LengthRequired => write!(f, "411 Length Required"),
            Self::PreconditionFailed => write!(f, "412 Precondition Failed"),
            Self::ContentTooLarge => write!(f, "413 Content Too Large"),
            Self::URITooLong => write!(f, "414 URI Too Long"),
            Self::UnsupportedMediaType => write!(f, "415 Unsupported Media Type"),
            Self::RangeNotSatisfiable => write!(f, "416 Range Not Satisfiable"),
            Self::ExpectationFailed => write!(f, "417 Expectation Failed"),
            Self::Imateapot => write!(f, "418 I'm a teapot"),
            Self::MisdirectedRequest => write!(f, "421 Misdirected Request"),
            Self::UnprocessableContent => write!(f, "422 Unprocessable Content"),
            Self::Locked => write!(f, "423 Locked"),
            Self::FailedDependency => write!(f, "424 Failed Dependency"),
            Self::TooEarly => write!(f, "425 Too Early"),
            Self::UpgradeRequired => write!(f, "426 Upgrade Required"),
            Self::PreconditionRequired => write!(f, "428 Precondition Required"),
            Self::TooManyRequests => write!(f, "429 Too Many Requests"),
            Self::RequestHeaderFieldsTooLarge => write!(f, "431 Request Header Fields Too Large"),
            Self::UnavailableForLegalReasons => write!(f, "451 Unavailable For Legal Reasons"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
            Self::BadGateway => write!(f, "502 Bad Gateway"),
            Self::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            Self::GatewayTimeout => write!(f, "504 Gateway Timeout"),
            Self::HTTPVersionNotSupported => write!(f, "505 HTTP Version Not Supported"),
            Self::VariantAlsoNegotiates => write!(f, "506 Variant Also Negotiates"),
            Self::InsufficientStorage => write!(f, "507 Insufficient Storage"),
            Self::LoopDetected => write!(f, "508 Loop Detected"),
            Self::NotExtended => write!(f, "510 Not Extended"),
            Self::NetworkAuthenticationRequired => write!(f, "511 Network Authentication Required"),
        }
    }
}

impl HttpStatusCode {
    pub fn from_usize(n: usize) -> Result<Self, i8> {
        match n {
            100 => Ok(Self::Continue),
            101 => Ok(Self::SwitchingProtocols),
            102 => Ok(Self::Processing),
            103 => Ok(Self::EarlyHints),
            200 => Ok(Self::OK),
            201 => Ok(Self::Created),
            202 => Ok(Self::Accepted),
            203 => Ok(Self::NonAuthoritativeInformation),
            204 => Ok(Self::NoContent),
            205 => Ok(Self::ResetContent),
            206 => Ok(Self::PartialContent),
            207 => Ok(Self::MultiStatus),
            208 => Ok(Self::AlreadyReported),
            226 => Ok(Self::IMUsed),
            300 => Ok(Self::MultipleChoices),
            301 => Ok(Self::MovedPermanently),
            302 => Ok(Self::Found),
            303 => Ok(Self::SeeOther),
            304 => Ok(Self::NotModified),
            305 => Ok(Self::UseProxy),
            306 => Ok(Self::Unused),
            307 => Ok(Self::TemporaryRedirect),
            308 => Ok(Self::PermanentRedirect),
            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            402 => Ok(Self::PaymentRequired),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            405 => Ok(Self::MethodNotAllowed),
            406 => Ok(Self::NotAcceptable),
            407 => Ok(Self::ProxyAuthenticationRequired),
            408 => Ok(Self::RequestTimeout),
            409 => Ok(Self::Conflict),
            410 => Ok(Self::Gone),
            411 => Ok(Self::LengthRequired),
            412 => Ok(Self::PreconditionFailed),
            413 => Ok(Self::ContentTooLarge),
            414 => Ok(Self::URITooLong),
            415 => Ok(Self::UnsupportedMediaType),
            416 => Ok(Self::RangeNotSatisfiable),
            417 => Ok(Self::ExpectationFailed),
            418 => Ok(Self::Imateapot),
            421 => Ok(Self::MisdirectedRequest),
            422 => Ok(Self::UnprocessableContent),
            423 => Ok(Self::Locked),
            424 => Ok(Self::FailedDependency),
            425 => Ok(Self::TooEarly),
            426 => Ok(Self::UpgradeRequired),
            428 => Ok(Self::PreconditionRequired),
            429 => Ok(Self::TooManyRequests),
            431 => Ok(Self::RequestHeaderFieldsTooLarge),
            451 => Ok(Self::UnavailableForLegalReasons),
            500 => Ok(Self::InternalServerError),
            501 => Ok(Self::NotImplemented),
            502 => Ok(Self::BadGateway),
            503 => Ok(Self::ServiceUnavailable),
            504 => Ok(Self::GatewayTimeout),
            505 => Ok(Self::HTTPVersionNotSupported),
            506 => Ok(Self::VariantAlsoNegotiates),
            507 => Ok(Self::InsufficientStorage),
            508 => Ok(Self::LoopDetected),
            510 => Ok(Self::NotExtended),
            511 => Ok(Self::NetworkAuthenticationRequired),
            _ => Err(-1),
        }
    }

    pub fn usize(&self) -> usize {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,
            Self::OK => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::IMUsed => 226,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::UseProxy => 305,
            Self::Unused => 306,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::ContentTooLarge => 413,
            Self::URITooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::Imateapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableContent => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HTTPVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
        }
    }
}
