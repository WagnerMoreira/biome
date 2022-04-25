use crate::formatter::TrailingSeparator;
use crate::{join_elements, soft_line_break_or_space, token, Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsExportNamedSpecifierList;

impl Format for JsExportNamedSpecifierList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(self, || token(","), TrailingSeparator::default())?,
        ))
    }
}