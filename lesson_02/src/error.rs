use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;

pub enum Error {
   Str(String),
   WindowBuildError,
   IntegerOrSdlError,
}

impl From<String> for Error {
   fn from(err: String) -> Error {
      Error::Str(err)
   }
}

impl From<WindowBuildError> for Error {
   fn from(_: WindowBuildError) -> Error {
      Error::WindowBuildError
   }
}

impl From<IntegerOrSdlError> for Error {
   fn from(_: IntegerOrSdlError) -> Error {
      Error::IntegerOrSdlError
   }
}
