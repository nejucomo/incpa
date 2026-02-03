use incpa_parser::Parser;
use incpa_state::ParserState;

use crate::BufferManager;

/// Every `Parser<[u8]>` is a [ByteParser]
impl<S> ByteParser for S where S: Parser<[u8]> {}

/// Any [Parser] over `[u8]` input
pub trait ByteParser: Parser<[u8]> {
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

        self.start_parser().run_parser(|parser| {
            let writeslice = bufmgr.get_write_slice();
            let readcnt = r.read(writeslice)?;
            bufmgr.process_write(parser, readcnt)
        })
    }
}
