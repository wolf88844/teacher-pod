use salvo::{
    Depot, Request, Response, Writer, async_trait, hyper::StatusCode, prelude::StatusError,
};

#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    QueryNotFound(String),
    DataNotFound,
    AuthorizationFailed(String),
    Unauthorized,
    DataExists,
}

pub type ApiResult = Result<(), Error>;

#[async_trait]
impl Writer for Error {
    async fn write(mut self, _: &mut Request, _: &mut Depot, res: &mut Response) {
        let info: (String, StatusCode) = match self {
            Error::Database(e) => (e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
            Error::QueryNotFound(s) => {
                let message = format!("Query Not Found: {}", s);
                (message, StatusCode::BAD_REQUEST)
            }
            Error::DataNotFound => (String::from("Data Not Found"), StatusCode::NOT_FOUND),
            Error::AuthorizationFailed(s) => (
                format!("account authenticaton failed:{}", s),
                StatusCode::UNAUTHORIZED,
            ),
            Error::Unauthorized => (String::from("Unauthorized"), StatusCode::UNAUTHORIZED),
            Error::DataExists => (String::from("Data Exists"), StatusCode::BAD_REQUEST),
        };
        res.status_code(info.1);
        res.render(&info.0);
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e)
    }
}
