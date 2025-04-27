#[derive(Debug)]
pub enum Step<O, C> {
    ParsedRec(O),
    RequestRec(C),
}
