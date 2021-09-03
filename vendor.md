<!-- file * -->
<!-- fn dbus_connection_setup_with_g_main -->
Sets the watch and timeout functions of a `DBusConnection`
to integrate the connection with the GLib main loop.
Pass in [`None`] for the `GMainContext` unless you're
doing something specialized.

If called twice for the same context, does nothing the second
time. If called once with context A and once with context B,
context B replaces context A as the context monitoring the
connection.
## `connection`
the connection
## `context`
the `GMainContext` or [`None`] for default context
<!-- fn deregister_device_event_listener -->
Removes a device event listener from the registry's listener queue,
 ceasing notification of events of the specified type.
## `listener`
a pointer to the `AtspiDeviceListener` for which
 device events are requested.
## `filter`
Unused parameter.

# Returns

[`true`] if successful, otherwise [`false`].
<!-- fn deregister_keystroke_listener -->
Removes a keystroke event listener from the registry's listener queue,
 ceasing notification of events with modifiers matching `modmask`.
## `listener`
a pointer to the `AtspiDeviceListener` for which
 keystroke events are requested.
## `key_set`
a pointer to the
 `AtspiKeyDefinition` array indicating which keystroke events are
 requested, or [`None`]
 to indicate that all keycodes and keyvals for the specified
 modifier set are to be included.
## `modmask`
the key modifier mask for which this listener is to be
 'deregistered' (of type `AtspiKeyMaskType`).
## `event_types`
an `AtspiKeyMaskType` mask indicating which
 types of key events were requested (`ATSPI_KEY_PRESSED`, etc.).

# Returns

[`true`] if successful, otherwise [`false`].
<!-- fn generate_keyboard_event -->
Synthesizes a keyboard event (as if a hardware keyboard event occurred in the
current UI context).
## `keyval`
a `gint` indicating the keycode or keysym or modifier mask of the
 key event being synthesized.
## `keystring`
an (optional) UTF-8 string which, if
 `synth_type` is `ATSPI_KEY_STRING`, indicates a 'composed'
 keyboard input string being synthesized; this type of
 keyboard event synthesis does not emulate hardware
 keypresses but injects the string as though a composing
 input method (such as XIM) were used.
## `synth_type`
an `AtspiKeySynthType` flag indicating whether `keyval`
 is to be interpreted as a keysym rather than a keycode
 (`ATSPI_KEY_SYM`) or a string (`ATSPI_KEY_STRING`) or a modifier
 mask (`ATSPI_KEY_LOCKMODIFIERS` and `ATSPI_KEY_UNLOCKMODIFIERS`), or
 whether to synthesize `ATSPI_KEY_PRESS`,
 `ATSPI_KEY_RELEASE`, or both (`ATSPI_KEY_PRESSRELEASE`).

# Returns

[`true`] if successful, otherwise [`false`].
<!-- fn desktop_list -->
Gets the list of virtual desktops. On return, `list` will point
 to a newly-created, NULL terminated array of virtual desktop
 pointers.
 It is the responsibility of the caller to free this array when
 it is no longer needed.
NOTE: currently multiple virtual desktops are not implemented;
this implementation always returns a `Garray` with a single
[`Accessible`][crate::Accessible] desktop.

# Returns

a [`glib::Array`][crate::glib::Array] of
desktops.
<!-- fn register_device_event_listener -->
Registers a listener for device events, for instance button events.
## `listener`
a pointer to the `AtspiDeviceListener` which requests
 the events.
## `event_types`
an `AtspiDeviceEventMask` mask indicating which
 types of key events are requested (`ATSPI_KEY_PRESSED`, etc.).
## `filter`
Unused parameter.

# Returns

[`true`] if successful, otherwise [`false`].
<!-- fn register_keystroke_listener -->
Registers a listener for keystroke events, either pre-emptively for
 all windows (`ATSPI_KEYLISTENER_ALL_WINDOWS`),
 non-preemptively (`ATSPI_KEYLISTENER_NOSYNC`), or
 pre-emptively at the toolkit level (`ATSPI_KEYLISTENER_CANCONSUME`).
 If ALL_WINDOWS or CANCONSUME are used, the event is consumed
 upon receipt if one of `listener`'s callbacks returns [`true`]
 (other sync_type values may be available in the future).
## `listener`
a pointer to the `AtspiDeviceListener` for which
 keystroke events are requested.
## `key_set`
a pointer to the
 `AtspiKeyDefinition` array indicating which keystroke events are
 requested, or NULL
 to indicate that all keycodes and keyvals for the specified
 modifier set are to be included.
## `modmask`
an `AtspiKeyMaskType` mask indicating which
 key event modifiers must be set in combination with `keys`,
 events will only be reported for key events for which all
 modifiers in `modmask` are set. If you wish to listen for
 events with multiple modifier combinations, you must call
 `atspi_register_keystroke_listener` once for each
 combination.
## `event_types`
an `AtspiKeyMaskType` mask indicating which
 types of key events are requested (`ATSPI_KEY_PRESSED` etc.).
## `sync_type`
an `AtspiKeyListenerSyncType` parameter indicating
 the behavior of the notification/listener transaction.

# Returns

[`true`] if successful, otherwise [`false`].
<!-- fn set_main_context -->
Sets the main loop context that AT-SPI should assume is in use when
setting an idle callback.
This function should be called by application-side implementors (ie,
at-spi2-atk) when it is desirable to re-enter the main loop.
## `cnx`
The `GMainContext` to use.
<!-- const COMPONENTLAYER_COUNT -->
One higher than the highest valid value of [`ComponentLayer`][crate::ComponentLayer].
<!-- const COORD_TYPE_COUNT -->
One higher than the highest valid value of [`CoordType`][crate::CoordType].
<!-- const EVENTTYPE_COUNT -->
One higher than the highest valid value of `AtspiEventType`.
<!-- const KEYEVENTTYPE_COUNT -->
One higher than the highest valid value of `AtspiKeyEventType`.
<!-- const KEYSYNTHTYPE_COUNT -->
One higher than the highest valid value of `AtspiKeySynthType`.
<!-- const LOCALE_TYPE_COUNT -->
One higher than the highest valid value of `AtspiLocaleType`.
<!-- const MODIFIERTYPE_COUNT -->
One higher than the highest valid value of `AtspiModifierType`.
<!-- const RELATIONTYPE_COUNT -->
One higher than the highest valid value of `AtspiRelationType`.
<!-- const ROLE_COUNT -->
One higher than the highest valid value of [`Role`][crate::Role].
<!-- const SCROLLTYPE_COUNT -->
One higher than the highest valid value of [`ScrollType`][crate::ScrollType].
<!-- const SORTORDER_COUNT -->
One higher than the highest valid value of [`CollectionSortOrder`][crate::CollectionSortOrder].
<!-- const STATETYPE_COUNT -->
One higher than the highest valid value of [`StateType`][crate::StateType].
<!-- const TEXT_BOUNDARY_TYPE_COUNT -->
One higher than the highest valid value of [`TextBoundaryType`][crate::TextBoundaryType].
<!-- const TEXT_CLIP_TYPE_COUNT -->
One higher than the highest valid value of [`TextClipType`][crate::TextClipType].
<!-- const TREETRAVERSALTYPE_COUNT -->
One higher than the highest valid value of
`AtspiCollection_TreeTraversalType`.
<!-- struct Accessible -->


# Implements

[`AccessibleExt`][trait@crate::prelude::AccessibleExt], [`ActionExt`][trait@crate::prelude::ActionExt], [`CollectionExt`][trait@crate::prelude::CollectionExt], [`ComponentExt`][trait@crate::prelude::ComponentExt], [`DocumentExt`][trait@crate::prelude::DocumentExt], [`EditableTextExt`][trait@crate::prelude::EditableTextExt], [`HypertextExt`][trait@crate::prelude::HypertextExt], [`ImageExt`][trait@crate::prelude::ImageExt], [`SelectionExt`][trait@crate::prelude::SelectionExt], [`TableExt`][trait@crate::prelude::TableExt], [`TableCellExt`][trait@crate::prelude::TableCellExt], [`TextExt`][trait@crate::prelude::TextExt], [`ValueExt`][trait@crate::prelude::ValueExt]
<!-- trait AccessibleExt::fn get_attributes -->
Gets the `AttributeSet` representing any assigned
name-value pair attributes or annotations for this object.
For typographic, textual, or textually-semantic attributes, see
atspi_text_get_attributes instead.

# Returns

The name-value-pair
attributes assigned to this object.
<!-- trait AccessibleExt::fn attributes_as_array -->
Gets a [`glib::Array`][crate::glib::Array] representing any assigned
name-value pair attributes or annotations for this object.
For typographic, textual, or textually-semantic attributes, see
atspi_text_get_attributes_as_array instead.

# Returns

The name-value-pair
 attributes assigned to this object.
<!-- trait AccessibleExt::fn interfaces -->
A set of pointers to all interfaces supported by an [`Accessible`][crate::Accessible].

# Returns

A [`glib::Array`][crate::glib::Array] of strings
 describing the interfaces supported by the object. Interfaces are
 denoted in short-hand (i.e. "Component", "Text" etc.).
<!-- trait AccessibleExt::fn relation_set -->
Gets the set of `AtspiRelation` objects which describes this [`Accessible`][crate::Accessible] object's
relationships with other [`Accessible`][crate::Accessible] objects.

# Returns

a [`glib::Array`][crate::glib::Array] of
 `AtspiRelation` pointers or NULL on exception.
<!-- struct Action -->


# Implements

[`ActionExt`][trait@crate::prelude::ActionExt]
<!-- struct Collection -->


# Implements

[`CollectionExt`][trait@crate::prelude::CollectionExt]
<!-- trait CollectionExt::fn matches -->
Gets all [`Accessible`][crate::Accessible] objects from the `self` matching a given
`rule`.
## `rule`
An [`MatchRule`][crate::MatchRule] describing the match criteria.
## `sortby`
An [`CollectionSortOrder`][crate::CollectionSortOrder] specifying the way the results are to
 be sorted.
## `count`
The maximum number of results to return, or 0 for no limit.
## `traverse`
Not supported.

# Returns

All
 [`Accessible`][crate::Accessible] objects matching the given match rule.
<!-- trait CollectionExt::fn matches_from -->
Gets all [`Accessible`][crate::Accessible] objects from the `self`, before
`current_object`, matching a given `rule`.
## `current_object`
Upon reaching this object, searching should stop.
## `rule`
An [`MatchRule`][crate::MatchRule] describing the match criteria.
## `sortby`
An [`CollectionSortOrder`][crate::CollectionSortOrder] specifying the way the results are to
 be sorted.
## `tree`
An [`CollectionTreeTraversalType`][crate::CollectionTreeTraversalType] specifying restrictions on
 the objects to be traversed.
## `count`
The maximum number of results to return, or 0 for no limit.
## `traverse`
Not supported.

# Returns

All
 [`Accessible`][crate::Accessible] objects matching the given match rule that preceed
 `current_object`.
<!-- trait CollectionExt::fn matches_to -->
Gets all [`Accessible`][crate::Accessible] objects from the `self`, after
`current_object`, matching a given `rule`.
## `current_object`
The object at which to start searching.
## `rule`
An [`MatchRule`][crate::MatchRule] describing the match criteria.
## `sortby`
An [`CollectionSortOrder`][crate::CollectionSortOrder] specifying the way the results are to
 be sorted.
## `tree`
An [`CollectionTreeTraversalType`][crate::CollectionTreeTraversalType] specifying restrictions on
 the objects to be traversed.
## `limit_scope`
If [`true`], only descendants of `current_object`'s parent
 will be returned. Otherwise (if [`false`]), any accessible may be
 returned if it would preceed `current_object` in a flattened
 hierarchy.
## `count`
The maximum number of results to return, or 0 for no limit.
## `traverse`
Not supported.

# Returns

All
 [`Accessible`][crate::Accessible] objects matching the given match rule after
 `current_object`.
<!-- struct Component -->


# Implements

[`ComponentExt`][trait@crate::prelude::ComponentExt]
<!-- struct Document -->


# Implements

[`DocumentExt`][trait@crate::prelude::DocumentExt]
<!-- trait DocumentExt::fn attributes -->
Gets all constant attributes for the document as a whole. For attributes
that change within the document content, see `atspi_text_get_attribute_run` instead.

# Deprecated since 2.10

Use atspi_document_get_document_attributes instead.

# Returns

a `GHashTable`
 containing the constant attributes of the document, as name-value pairs.
<!-- trait DocumentExt::fn document_attributes -->
Gets all constant attributes for the document as a whole. For attributes
that change within the document content, see `atspi_text_get_attribute_run` instead.

# Returns

a `GHashTable`
 containing the constant attributes of the document, as name-value pairs.
<!-- struct EditableText -->


# Implements

[`EditableTextExt`][trait@crate::prelude::EditableTextExt]
<!-- struct Hyperlink -->


# Implements

[`HyperlinkExt`][trait@crate::prelude::HyperlinkExt]
<!-- struct Hypertext -->


# Implements

[`HypertextExt`][trait@crate::prelude::HypertextExt]
<!-- struct Image -->


# Implements

[`ImageExt`][trait@crate::prelude::ImageExt]
<!-- struct MatchRule -->

<!-- impl MatchRule::fn new -->
Creates a new [`MatchRule`][crate::MatchRule] with specified `states`, `attributes`,
`interfaces`, and `roles`.
## `states`
An [`StateSet`][crate::StateSet] specifying the states to match or NULL if none.
## `statematchtype`
An [`CollectionMatchType`][crate::CollectionMatchType] specifying how to interpret
 `states`.
## `attributes`
A `GHashTable` specifying
 attributes to match. To specify multiple attribute values,
 separate each value with a :: If an attribute value contains a :,
 then it can be escaped by preceding it with a \. A backslash can
 likewise be escaped by inserting a double backslash.
## `attributematchtype`
An [`CollectionMatchType`][crate::CollectionMatchType] specifying how to
 interpret `attributes`.
## `roles`
A [`glib::Array`][crate::glib::Array] of roles to match, or NULL if
 not applicable.
## `rolematchtype`
An [`CollectionMatchType`][crate::CollectionMatchType] specifying how to
 interpret `roles`.
## `interfaces`
An array of interfaces to match, or
 NULL if not applicable. Interface names should be specified
 by their DBus names (org.a11y.Atspi.Accessible,
 org.a11y.Atspi.Component, etc).
## `interfacematchtype`
An [`CollectionMatchType`][crate::CollectionMatchType] specifying how to
 interpret `interfaces`.
## `invert`
if [`true`], the match rule should be denied (inverted); if [`false`],
 it should not. For example, if the match rule defines that a match is
 an object of ROLE_HEADING which has STATE_FOCUSABLE and a click action,
 inverting it would match all objects that are not of ROLE_HEADING,
 focusable and clickable at the same time.

# Returns

A new [`MatchRule`][crate::MatchRule].
<!-- impl Range::fn copy -->
Gets a copy of an [`Range`][crate::Range] object.

# Returns

the [`Range`][crate::Range] copy of an [`Range`][crate::Range] object.
<!-- struct Selection -->


# Implements

[`SelectionExt`][trait@crate::prelude::SelectionExt]
<!-- struct StateSet -->


# Implements

[`StateSetExt`][trait@crate::prelude::StateSetExt]
<!-- impl StateSet::fn new -->
Generates an [`StateSet`][crate::StateSet] with the given `states`.
## `states`
An array of states with which the
 method initializes the state set.

# Returns

A new [`StateSet`][crate::StateSet] with the given states.
<!-- trait StateSetExt::fn states -->
Returns the states in an [`StateSet`][crate::StateSet] as an array.

# Returns

A [`glib::Array`][crate::glib::Array] of state
 types representing the current state.
<!-- struct Table -->


# Implements

[`TableExt`][trait@crate::prelude::TableExt]
<!-- trait TableExt::fn selected_columns -->
Queries a table for a list of indices of columns which are currently
selected.

# Returns

an array of `gint` values,
 specifying which columns are currently selected.
<!-- trait TableExt::fn selected_rows -->
Queries a table for a list of indices of rows which are currently selected.

# Returns

an array of `gint` values,
 specifying which rows are currently selected.
<!-- struct TableCell -->


# Implements

[`TableCellExt`][trait@crate::prelude::TableCellExt]
<!-- struct Text -->


# Implements

[`TextExt`][trait@crate::prelude::TextExt]
<!-- trait TextExt::fn attribute_run -->
Gets a set of attributes applied to a range of text from an [`Text`][crate::Text] object, optionally
including its 'default' attributes.
## `offset`
a `gint` indicating the offset from which the attribute
 search is based.
## `include_defaults`
a `bool` that, when set as [`false`], indicates the call
should only return those attributes which are explicitly set on the current
attribute run, omitting any attributes which are inherited from the
default values.

# Returns

a `GHashTable` with attributes
 defined at the indicated offset, optionally including the 'default' ones.

## `start_offset`
a `gint` pointer indicating the start of the desired text
 range.

## `end_offset`
a `gint` pointer indicating the first character past the desired
 range.
<!-- trait TextExt::fn attributes -->
Gets the attributes applied to a range of text from an [`Text`][crate::Text]
object. The text attributes correspond to CSS attributes
where possible.
`<em>`DEPRECATED`</em>`

# Deprecated since 2.10

Use atspi_text_get_text_attributes instead.
## `offset`
a `gint` indicating the offset from which the attribute
 search is based.

# Returns

a `GHashTable`
describing the attributes at the given character offset.

## `start_offset`
a `gint` pointer indicating the start of the desired text
 range.

## `end_offset`
a `gint` pointer indicating the first character past the desired
 range.
<!-- trait TextExt::fn bounded_ranges -->
Gets the ranges of text from an [`Text`][crate::Text] object which lie within the
 bounds defined by (`x`, `y`) and (`x`+`width`, `y`+`height`).
## `x`
the 'starting' x coordinate of the bounding box.
## `y`
the 'starting' y coordinate of the bounding box.
## `width`
the x extent of the bounding box.
## `height`
the y extent of the bounding box.
## `type_`
an `AccessibleCoordType` indicating the coordinate system to use
 for the returned values.
## `clipTypeX`
an [`TextClipType`][crate::TextClipType] indicating how to treat characters that
 intersect the bounding box's x extents.
## `clipTypeY`
an [`TextClipType`][crate::TextClipType] indicating how to treat characters that
 intersect the bounding box's y extents.

# Returns

a null-terminated list of
 pointers to [`TextRange`][crate::TextRange] structs detailing the bounded text.
<!-- trait TextExt::fn default_attributes -->
Gets the default attributes applied to an [`Text`][crate::Text]
object. The text attributes correspond to CSS attributes
where possible. The combination of this attribute set and
the attributes reported by `atspi_text_get_attributes`
describes the entire set of text attributes over a range.

# Returns

a `GHashTable`
 containing the default attributes applied to a text object,
 (exclusive of explicitly-set attributes), encoded as UTF-8.
<!-- trait TextExt::fn text_attributes -->
Gets the attributes applied to a range of text from an [`Text`][crate::Text]
object. The text attributes correspond to CSS attributes
where possible.
`<em>`DEPRECATED`</em>`
## `offset`
a `gint` indicating the offset from which the attribute
 search is based.

# Returns

a `GHashTable`
describing the attributes at the given character offset.

## `start_offset`
a `gint` pointer indicating the start of the desired text
 range.

## `end_offset`
a `gint` pointer indicating the first character past the desired
 range.
<!-- struct Value -->


# Implements

[`ValueExt`][trait@crate::prelude::ValueExt]
