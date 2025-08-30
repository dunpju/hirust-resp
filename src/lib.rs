use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, error};
use derive_more::derive::{Display, Error};
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     success(&req, Some(String::from("Hey test!"))).respond_to(req)
/// }
///```
///
pub fn success<T: Sized + Serialize + Default>(data: Option<T>) -> Response<T> {
    Response {
        data,
        msg: String::from("成功"),
        code: 200,
    }
}

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     success_respond_to(&req, Some(String::from("Hey test!")))
/// }
///```
///
pub fn success_respond_to<T: Sized + Serialize + Default>(
    req: &HttpRequest,
    data: Option<T>,
) -> HttpResponse<<Response<T> as Responder>::Body> {
    success(data).respond_to(req)
}

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     error(&req, Some(String::from("Hey test!"))).respond_to(req)
/// }
///```
///
pub fn error<T: Sized + Serialize + Default>(data: Option<T>) -> Response<T> {
    Response {
        data,
        msg: String::from("失败"),
        ..Default::default()
    }
}

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     error_respond_to(&req, Some(String::from("Hey test!")))
/// }
///```
///
pub fn error_respond_to<T: Sized + Serialize + Default>(
    req: &HttpRequest,
    data: Option<T>,
) -> HttpResponse<<Response<T> as Responder>::Body> {
    error(data).respond_to(req)
}

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     return throw(&req, errcode::VALID_CODE_ERROR);
/// }
///```
///
pub fn throw(
    req: &HttpRequest,
    ec: ErrorCode,
) -> HttpResponse<<Response<ErrorCode> as Responder>::Body> {
    Response::<ErrorCode> {
        data: None,
        msg: ec.message().parse().unwrap(),
        code: ec.code() as i32,
    }
    .respond_to(req)
}

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     return throw_tips(&req, errcode::NOT_EXIST, "tips msg");
/// }
///```
///
pub fn throw_tips(
    req: &HttpRequest,
    ec: ErrorCode,
    tips: &'static str,
) -> HttpResponse<<Response<ErrorCode> as Responder>::Body> {
    Response::<ErrorCode> {
        data: None,
        msg: ec.message().replace("%s", tips).parse().unwrap(),
        code: ec.code() as i32,
    }
    .respond_to(req)
}

///
/// Examples
///```text
/// async fn test(req: actix_web::HttpRequest) -> impl Responder {
///     return unauthorized::<String>(&req);
/// }
///```
///
pub fn unauthorized<T: Sized + Serialize + Default>(
    req: &HttpRequest,
) -> HttpResponse<<Response<T> as Responder>::Body> {
    let r: Response<String> = Response {
        data: None,
        msg: String::from("无权限访问"),
        ..Default::default()
    };
    r.respond_to(req)
}

#[derive(Serialize, Default)]
pub struct Response<T>
where
    T: Sized + Serialize,
{
    pub code: i32,
    pub data: Option<T>,
    pub msg: String,
}

// Responder
impl<T> Responder for Response<T>
where
    T: Sized + Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl<T> Response<T>
where
    T: Sized + Serialize,
{
    pub fn body_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl<T: Serialize> Display for Response<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap();
        write!(f, "{}", serialized)
    }
}

impl<T: Serialize> Debug for Response<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap();
        write!(f, "{}", serialized)
    }
}

impl<T: Serialize> error::ResponseError for Response<T> {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        // match *self {
        //     ErrorCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        //     ErrorCode::BadClientData => StatusCode::BAD_REQUEST,
        //     ErrorCode::Timeout => StatusCode::GATEWAY_TIMEOUT,
        // }
        StatusCode::OK
    }
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct ErrorCode {
    pub code: i64,
    pub message: &'static str,
}

impl ErrorCode {
    #[allow(dead_code)]
    pub fn new(code: i64, message: &'static str) -> ErrorCode {
        ErrorCode { code, message }
    }

    #[allow(dead_code)]
    pub fn code(&self) -> i64 {
        self.code
    }

    #[allow(dead_code)]
    pub fn code_string(&self) -> Option<String> {
        Some(format!("{}", self.code))
    }

    #[allow(dead_code)]
    pub fn message(&self) -> &'static str {
        &self.message
    }

    ///
    /// Examples
    ///```text
    /// async fn test(req: actix_web::HttpRequest) -> impl Responder {
    ///     let tips = errcode::NOT_EXIST.tips("gg");
    ///     return ...;
    /// }
    ///```
    ///
    #[allow(dead_code)]
    pub fn tips(&self, tips: &'static str) -> String {
        self.message().replace("%s", tips).parse().unwrap()
    }

    ///
    /// Examples
    ///```text
    /// async fn test(req: actix_web::HttpRequest) -> impl Responder {
    ///     return errcode::NOT_EXIST.throw_tips(&req, "gg");
    /// }
    ///```
    ///
    #[allow(dead_code)]
    pub fn throw_tips(
        &self,
        req: &HttpRequest,
        tips: &'static str,
    ) -> HttpResponse<<Response<ErrorCode> as Responder>::Body> {
        Response::<ErrorCode> {
            data: None,
            msg: self.message().replace("%s", tips).parse().unwrap(),
            code: self.code() as i32,
        }
        .respond_to(req)
    }

    ///
    /// Examples
    ///```text
    /// async fn test(req: actix_web::HttpRequest) -> impl Responder {
    ///     return errcode::VALID_CODE_ERROR.throw(&req);
    /// }
    ///```
    ///
    #[allow(dead_code)]
    pub fn throw(
        &self,
        req: &HttpRequest,
    ) -> HttpResponse<<Response<ErrorCode> as Responder>::Body> {
        Response::<ErrorCode> {
            data: None,
            msg: self.message().parse().unwrap(),
            code: self.code() as i32,
        }
        .respond_to(req)
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.code, self.message)
    }
}

impl error::ResponseError for ErrorCode {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        // match *self {
        //     ErrorCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        //     ErrorCode::BadClientData => StatusCode::BAD_REQUEST,
        //     ErrorCode::Timeout => StatusCode::GATEWAY_TIMEOUT,
        // }
        StatusCode::BAD_REQUEST
    }
}

#[allow(dead_code)]
pub trait Auth {
    fn response(&self) -> Response<Vec<i32>>;
    fn ok(&self) -> bool;
}

// 拦截器
#[allow(dead_code)]
#[deprecated]
pub fn interceptor<A: Auth>(a: A) -> Option<Response<Vec<i32>>> {
    if !a.ok() {
        return Some(a.response());
    }
    None
}

#[derive(Debug, Display, Error)]
#[display("{file}:{line} {message}")]
#[allow(unused)]
pub struct Error {
    pub file: &'static str,
    pub line: u32,
    pub message: String,
}

impl Error {
    ///
    /// Examples:
    /// Registration middleware
    ///```text
    /// App::new().wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server::handler))
    ///```
    ///
    /// Define middleware handler
    ///```text
    /// pub fn handler<B>(mut res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    ///     println!("{}", "add_internal_server_error_header");
    ///     Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    /// }
    ///```
    ///
    /// Return an error
    ///```text
    /// #[get(path = "/index")]
    /// #[allow(unused)]
    /// async fn index(req: HttpRequest) -> Result<impl Responder, Error> {
    ///     if true {
    ///         let message = String::from("测试");
    ///         Err::<HttpResponse, Error>(Error::new(file!(), line!(), message))
    ///     } else {
    ///         Ok(success_respond_to(&req, Some("测试")))
    ///     }
    /// }
    ///```
    ///
    #[allow(unused)]
    pub fn new(file: &'static str, line: u32, message: String) -> Self {
        Self {
            file,
            line,
            message,
        }
    }
}

// Use default implementation for `error_response()` method
impl error::ResponseError for Error {}
