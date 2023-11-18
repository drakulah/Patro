#[derive(Debug)]
pub enum ErrorKind {
  InvalidRequest,
  ResponseBodyError,
  BytesConversion,
}
