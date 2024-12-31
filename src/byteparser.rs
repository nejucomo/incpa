mod bufmgr;

pub use self::bufmgr::BufferManager;

#[cfg(feature = "tokio")]
use std::future::Future;

use crate::{Error, Parser};

/// Any [Parser] over `[u8]` input is a [ByteParser] by blanket impl
pub trait ByteParser<O, E = Error>: Parser<[u8], O, E> {
    /// Consume and parse all of input from `r`
    fn parse_reader<R>(self, mut r: R) -> Result<O, E>
    where
        R: std::io::Read,
        E: From<Error> + From<std::io::Error>,
    {
        let mut bufmgr = BufferManager::default();

        self.run_parser(|parser| {
            let writeslice = bufmgr.get_write_slice();
            let readcnt = r.read(writeslice)?;
            bufmgr.process_write(parser, readcnt)
        })
    }

    /// Consume and parse all of input from `r` asynchronously
    #[cfg(feature = "tokio")]
    fn parse_reader_async<R>(self, r: R) -> impl Future<Output = Result<O, E>>
    where
        R: tokio::io::AsyncRead,
        E: From<Error> + From<std::io::Error>,
    {
        use crate::OutcomeExt;
        use tokio::io::AsyncReadExt;

        self.run_parser_async(
            (BufferManager::default(), Box::pin(r)),
            |parser, (mut bufmgr, mut r)| async {
                let writeslice = bufmgr.get_write_slice();
                let readcnt = r.read(writeslice).await?;
                bufmgr
                    .process_write(parser, readcnt)
                    .map_parser(|p| (p, (bufmgr, r)))
            },
        )
    }
}

impl<P, O, E> ByteParser<O, E> for P where P: Parser<[u8], O, E> {}
