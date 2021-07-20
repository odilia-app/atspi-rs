// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

mod accessible;
pub use self::accessible::{Accessible, NONE_ACCESSIBLE};

mod action;
pub use self::action::{Action, NONE_ACTION};

mod collection;
pub use self::collection::{Collection, NONE_COLLECTION};

mod component;
pub use self::component::{Component, NONE_COMPONENT};

mod document;
pub use self::document::{Document, NONE_DOCUMENT};

mod editable_text;
pub use self::editable_text::{EditableText, NONE_EDITABLE_TEXT};

mod hyperlink;
pub use self::hyperlink::{Hyperlink, NONE_HYPERLINK};

mod hypertext;
pub use self::hypertext::{Hypertext, NONE_HYPERTEXT};

mod image;
pub use self::image::{Image, NONE_IMAGE};

mod selection;
pub use self::selection::{Selection, NONE_SELECTION};

mod table;
pub use self::table::{Table, NONE_TABLE};

mod table_cell;
pub use self::table_cell::{TableCell, NONE_TABLE_CELL};

mod text;
pub use self::text::{Text, NONE_TEXT};

mod value;
pub use self::value::{Value, NONE_VALUE};

mod enums;
pub use self::enums::Role;

#[doc(hidden)]
pub mod traits {
    pub use super::accessible::AccessibleExt;
    pub use super::action::ActionExt;
    pub use super::collection::CollectionExt;
    pub use super::component::ComponentExt;
    pub use super::document::DocumentExt;
    pub use super::editable_text::EditableTextExt;
    pub use super::hyperlink::HyperlinkExt;
    pub use super::hypertext::HypertextExt;
    pub use super::image::ImageExt;
    pub use super::selection::SelectionExt;
    pub use super::table::TableExt;
    pub use super::table_cell::TableCellExt;
    pub use super::text::TextExt;
    pub use super::value::ValueExt;
}
