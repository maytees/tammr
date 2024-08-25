#[derive(PartialOrd, PartialEq)]
pub(crate) enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
    Dot,
}
