mod padding;
mod rotation;

pub trait Value: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;

    // TODO remove this once RenderedElement's options field is redone
    #[allow(dead_code)]
    fn to_string(&self) -> String;
}
