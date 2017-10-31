#![forbid(
    exceeding_bitshifts, mutable_transmutes, no_mangle_const_items, unknown_crate_types, warnings
)]
#![deny(
    deprecated, improper_ctypes, /*missing_docs,*/
    non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
    private_no_mangle_fns, private_no_mangle_statics, stable_features,
    unconditional_recursion, unknown_lints, unsafe_code, unused, unused_allocation,
    unused_attributes, unused_comparisons, unused_features, unused_parens, while_true
)]
#![warn(
    trivial_casts, trivial_numeric_casts, unused_import_braces,
    unused_extern_crates, unused_qualifications, unused_results
)]

extern crate tomorrow_core;

mod recuperator;
pub use self::recuperator::*;