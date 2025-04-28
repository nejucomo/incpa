/// A [Step] is the output produced by a recursive parser
#[derive(Debug)]
pub enum Step<O, C> {
    /// A recursive parser produced an inner value `O`
    ParsedRec(O),
    /// A recursive parser produced the continuation `C`
    RequestRec(C),
}
