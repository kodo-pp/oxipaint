#![deny(
    warnings,
    non_ascii_idents,
    clippy::as_underscore,
    clippy::borrow_as_ptr,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::clone_on_ref_ptr,
    clippy::cloned_instead_of_copied,
    clippy::debug_assert_with_mut_call,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_link_with_quotes,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::flat_map_option,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::future_not_send,
    clippy::implicit_clone,
    clippy::implicit_hasher,
    clippy::imprecise_flops,
    clippy::inconsistent_struct_constructor,
    clippy::invalid_upcast_comparisons,
    clippy::large_stack_arrays,
    clippy::map_unwrap_or,
    clippy::mutex_atomic,
    clippy::semicolon_if_nothing_returned,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::undocumented_unsafe_blocks,
    clippy::used_underscore_binding
)]

mod canvas;
mod color;
mod render;
mod ui;

use gtk::prelude::*;
use gtk::Application;

/// Entry point to the editor application.
pub fn main_func() {
    let app = Application::builder()
        .application_id("me.kodopp.oxipaint")
        .build();

    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    app.run();
}
