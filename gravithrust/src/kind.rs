pub use crate::kind_generated::kindstr_to_kind;
pub use crate::kind_generated::Kind;
pub struct KindExpanded {
    kind: Kind,
    capacity: u32,
    soft_capacity: u32,
    is_static: bool,
}
