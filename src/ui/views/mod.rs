mod form_view;
mod list_view;
mod detail_view;
mod stats_view;
mod help_view;
mod search_view;

// Only export what we're currently using
pub(super) use form_view::draw_form;
pub(super) use list_view::draw_log_list;

// Keep these private until they're implemented
pub(crate) use detail_view::draw_detail;
pub(crate) use stats_view::draw_stats;
pub(crate) use help_view::draw_help;
pub(crate) use search_view::draw_search;