#[allow(unused_imports, dead_code)]
mod auto;
use auto::*;
pub use auto::{functions::*, Cache, Role, ScrollType};
mod init;
pub use init::*;
mod desktop;
pub use desktop::*;
pub mod accessible;

pub mod prelude {
    pub use crate::auto::{traits::*, Accessible, Role};
    pub use crate::text::TextExtManual;
}

pub mod action {
    pub use crate::auto::{traits::ActionExt, Action};
}

pub mod collection {
    pub use crate::auto::{
        traits::CollectionExt, Collection, CollectionMatchType, CollectionSortOrder,
        CollectionTreeTraversalType, MatchRule,
    };
}

pub mod component {
    pub use crate::auto::{traits::ComponentExt, Component, ComponentLayer};
}

pub mod document {
    pub use crate::auto::{traits::DocumentExt, Document};
}

pub mod editable_text {
    pub use crate::auto::{traits::EditableTextExt, EditableText};
}

pub mod hyperlink {
    pub use crate::auto::{traits::HyperlinkExt, Hyperlink};
}

pub mod hypertext {
    pub use crate::auto::{traits::HypertextExt, Hypertext};
}

pub mod image {
    pub use crate::auto::{traits::ImageExt, Image};
}

pub mod geometry {
    pub use crate::auto::{CoordType, Point, Rect};
}

pub mod range {
    pub use crate::auto::Range;
}

pub mod selection {
    pub use crate::auto::{traits::SelectionExt, Selection};
}

pub mod state {
    pub use crate::auto::{traits::StateSetExt, StateSet, StateType};
}

pub mod table {
    pub use crate::auto::{
        traits::{TableCellExt, TableExt},
        Table, TableCell,
    };
}

pub mod text;

pub mod value {
    pub use crate::auto::{traits::ValueExt, Value};
}
