use std::io;

use rustc_serialize::json::DecoderError;
use rustc_serialize::json::ParserError;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DuplicateKeys {
            description("Duplicate keys")
        }
        ExpectedJsonObject {
            description("Expected JSON object")
        }
        IOError(err: io::Error) {
            from()
            description("IO error")
            cause(err)
        }
        DecodeError(err: DecoderError) {
            from()
            description("Decoder error")
            cause(err)
        }
        ParserError(err: ParserError) {
            from()
            description("Parser error")
            cause(err)
        }
    }
}
