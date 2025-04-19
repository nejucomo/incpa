#[cfg(feature = "tokio")]
use std::future::Future;

use crate::parsing::{BufferManager, ParserState};
use crate::Parser;

/// Any [ParserState] over `[u8]` input is a [ByteFormat] by blanket impl
pub trait ByteFormat: Parser<[u8]> {
    /// Consume and parse all of input from `r`
    fn parse_reader<R, E>(self, r: R) -> Result<Self::Output, E>
    where
        R: std::io::Read,
        E: From<Self::Error> + From<std::io::Error>,
    {
        self.parse_reader_with_initial_buffer_size(r, 1 << 12)
    }

    /// Consume and parse all of input from `r` using an initial buffer size
    fn parse_reader_with_initial_buffer_size<R, E>(
        self,
        mut r: R,
        bufsize: usize,
    ) -> Result<Self::Output, E>
    where
        R: std::io::Read,
        E: From<Self::Error> + From<std::io::Error>,
    {
        let mut bufmgr = BufferManager::with_initial_size(bufsize);

        self.into_parser().run_parser(|parser| {
            let writeslice = bufmgr.get_write_slice();
            let readcnt = r.read(writeslice)?;
            bufmgr.process_write(parser, readcnt)
        })
    }

    /// Consume and parse all of input from `r` asynchronously
    #[cfg(feature = "tokio")]
    fn parse_reader_async<R, E>(self, r: R) -> impl Future<Output = Result<Self::Output, E>>
    where
        R: tokio::io::AsyncRead,
        E: From<Self::Error> + From<std::io::Error>,
    {
        self.parse_reader_with_initial_buffer_size_async(r, 1 << 12)
    }

    /// Consume and parse all of input from `r` asynchronously while using an initial buffer size
    #[cfg(feature = "tokio")]
    fn parse_reader_with_initial_buffer_size_async<R, E>(
        self,
        r: R,
        bufsize: usize,
    ) -> impl Future<Output = Result<Self::Output, E>>
    where
        R: tokio::io::AsyncRead,
        E: From<Self::Error> + From<std::io::Error>,
    {
        use crate::parsing::OutcomeExt;
        use tokio::io::AsyncReadExt;

        self.into_parser().run_parser_async(
            (BufferManager::with_initial_size(bufsize), Box::pin(r)),
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

impl<S> ByteFormat for S where S: Parser<[u8]> {}
