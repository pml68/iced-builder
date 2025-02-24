mod padding;
mod rotation;

pub trait ValueFromStr: Sized {
    type Err;

    fn value_from_str(s: &str) -> Result<Self, Self::Err>;
}
