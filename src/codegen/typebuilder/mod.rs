pub enum Scalar {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Usize,
    F32,
    F64,
    Str,
    String,
}

pub enum Type {
    Scalar(Scalar),
    Union(Vec<(String, Type)>),
    Enum(Vec<(String, Type)>),
    Tuple(Vec<Type>),
    Set(Box<Type>, Option<usize>),
    Map(String, Box<Type>),
}

impl Type {
    /// Merge two types. The merging type will remain the same.
    pub fn merge(&mut self, ty: Type) {
        match (self, ty) {
            (Type::Scalar(l), Type::Scalar(r)) => {
                *l = r;
            }
            (Type::Scalar(_), Type::Union(_)) => {}
            (Type::Scalar(_), Type::Enum(_)) => {}
            (Type::Scalar(_), Type::Tuple(_)) => {}
            (Type::Scalar(_), Type::Set(_, _)) => {}
            (Type::Scalar(_), Type::Map(_, _)) => {}
            (Type::Union(_), Type::Scalar(_)) => {}
            (Type::Union(l), Type::Union(r)) => {
                l.extend(r);
            }
            (Type::Union(l), Type::Enum(r)) => {
                l.extend(r);
            }
            (Type::Union(_), Type::Tuple(_)) => {}
            (Type::Union(_), Type::Set(_, _)) => {}
            (Type::Union(_), Type::Map(_, _)) => {}
            (Type::Enum(_), Type::Scalar(_)) => {}
            (Type::Enum(_), Type::Union(_)) => {}
            (Type::Enum(_), Type::Enum(_)) => {}
            (Type::Enum(_), Type::Tuple(_)) => {}
            (Type::Enum(_), Type::Set(_, _)) => {}
            (Type::Enum(_), Type::Map(_, _)) => {}
            (Type::Tuple(_), Type::Scalar(_)) => {}
            (Type::Tuple(_), Type::Union(_)) => {}
            (Type::Tuple(_), Type::Enum(_)) => {}
            (Type::Tuple(_), Type::Tuple(_)) => {}
            (Type::Tuple(_), Type::Set(_, _)) => {}
            (Type::Tuple(_), Type::Map(_, _)) => {}
            (Type::Set(_, _), Type::Scalar(_)) => {}
            (Type::Set(_, _), Type::Union(_)) => {}
            (Type::Set(_, _), Type::Enum(_)) => {}
            (Type::Set(_, _), Type::Tuple(_)) => {}
            (Type::Set(_, _), Type::Set(_, _)) => {}
            (Type::Set(_, _), Type::Map(_, _)) => {}
            (Type::Map(_, _), Type::Scalar(_)) => {}
            (Type::Map(_, _), Type::Union(_)) => {}
            (Type::Map(_, _), Type::Enum(_)) => {}
            (Type::Map(_, _), Type::Tuple(_)) => {}
            (Type::Map(_, _), Type::Set(_, _)) => {}
            (Type::Map(_, _), Type::Map(_, _)) => {}
        }
    }
}
