// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtspiMatchRule")]
    pub struct MatchRule(Object<ffi::AtspiMatchRule, ffi::AtspiMatchRuleClass>);

    match fn {
        type_ => || ffi::atspi_match_rule_get_type(),
    }
}

impl MatchRule {
    //#[doc(alias = "atspi_match_rule_new")]
    //pub fn new(states: &impl IsA<StateSet>, statematchtype: CollectionMatchType, attributes: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, attributematchtype: CollectionMatchType, roles: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 20 }, rolematchtype: CollectionMatchType, interfaces: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }, interfacematchtype: CollectionMatchType, invert: bool) -> MatchRule {
    //    unsafe { TODO: call ffi:atspi_match_rule_new() }
    //}
}

pub const NONE_MATCH_RULE: Option<&MatchRule> = None;

impl fmt::Display for MatchRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MatchRule")
    }
}
