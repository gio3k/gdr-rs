use crate::ast::core::literals::Literal;

pub struct Annotation {
    pub name: Literal,
    pub arguments: Vec<Literal>,
}

// pub fn absorb_annotation() -> Annotation {}