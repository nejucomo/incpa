mod bufmgr;

pub use self::bufmgr::BufferManager;

use std::io::Read;

use crate::{Error, Parser};

/// Any [Parser] over `[u8]` input is a [ByteParser] by blanket impl
pub trait ByteParser<O, E = Error>: Parser<[u8], O, E> {
    /// Consume and parse all of input from `r`
    fn parse_reader<R>(self, mut r: R) -> Result<O, E>
    where
        R: Read,
        E: From<Error> + From<std::io::Error>,
    {
        let mut bufmgr = BufferManager::default();

        self.run_parser(|parser| {
            let writeslice = bufmgr.get_write_slice();
            let readcnt = r.read(writeslice)?;
            bufmgr.process_write(parser, readcnt)
        })
    }
}

impl<P, O, E> ByteParser<O, E> for P where P: Parser<[u8], O, E> {}
