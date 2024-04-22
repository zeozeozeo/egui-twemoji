mod exposed;

use std::sync::Arc;

use egui::{text::LayoutJob, ImageSource, Layout, RichText, Sense};
use exposed::ExposedRichText;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq, Clone)]
enum TextSegment {
    Text(RichText),
    Emoji(String),
}

#[inline]
fn is_emoji(text: &str) -> bool {
    #[cfg(feature = "svg")]
    return twemoji_assets::svg::SvgTwemojiAsset::from_emoji(text).is_some();

    #[cfg(feature = "png")]
    return twemoji_assets::png::PngTwemojiAsset::from_emoji(text).is_some();
}

fn segment_text(input: &RichText) -> Vec<TextSegment> {
    let mut result = Vec::new();
    let mut text = String::new();

    for grapheme in UnicodeSegmentation::graphemes(input.text(), true) {
        if is_emoji(grapheme) {
            if !text.is_empty() {
                result.push(TextSegment::Text(
                    ExposedRichText::new_keep_properties(text.clone(), input).into(),
                ));
                text.clear();
            }
            result.push(TextSegment::Emoji(grapheme.to_string()));
        } else {
            text.push_str(grapheme);
        }
    }

    if !text.is_empty() {
        result.push(TextSegment::Text(
            ExposedRichText::new_keep_properties(text.clone(), input).into(),
        ));
    }

    result
}

#[derive(Default, Clone)]
struct LabelState {
    segments: Vec<TextSegment>,
    is_saved: bool,
}

impl LabelState {
    fn from_text(text: impl Into<RichText>) -> Self {
        let rich_text = text.into();
        Self {
            segments: segment_text(&rich_text),
            is_saved: false,
        }
    }

    fn load(ctx: &egui::Context, id: egui::Id, text: &RichText) -> Self {
        ctx.data_mut(|d| {
            d.get_temp(id)
                .unwrap_or_else(|| Self::from_text(text.clone()))
        })
    }

    fn save(self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_temp(id, self));
    }
}

#[must_use = "You should put this widget in an ui by calling `.show(ui);`"]
pub struct EmojiLabel {
    text: RichText,
    wrap: Option<bool>,
    truncate: bool,
    sense: Option<Sense>,
    selectable: Option<bool>,
}

#[cfg(all(feature = "svg", feature = "png"))]
compile_error!("features 'svg' and 'png' are mutually exclusive and cannot be enabled together");

fn get_source_for_emoji(emoji: &str) -> Option<ImageSource> {
    #[cfg(feature = "svg")]
    {
        let svg_data = twemoji_assets::svg::SvgTwemojiAsset::from_emoji(emoji)?;
        let source = ImageSource::Bytes {
            uri: format!("{emoji}.svg").into(),
            bytes: egui::load::Bytes::Static(svg_data.as_bytes()),
        };
        Some(source)
    }

    #[cfg(feature = "png")]
    {
        let png_data: &[u8] = twemoji_assets::png::PngTwemojiAsset::from_emoji(emoji)?;
        let source = ImageSource::Bytes {
            uri: format!("{emoji}.png").into(),
            bytes: egui::load::Bytes::Static(png_data),
        };
        Some(source)
    }
}

#[inline]
fn empty_response(ctx: egui::Context) -> egui::Response {
    egui::Response {
        ctx,
        layer_id: egui::LayerId::background(),
        id: egui::Id::NULL,
        rect: egui::Rect::ZERO,
        interact_rect: egui::Rect::ZERO,
        sense: Sense::click(),
        enabled: false,
        contains_pointer: false,
        hovered: false,
        highlighted: false,
        clicked: false,
        fake_primary_click: false,
        long_touched: false,
        drag_started: false,
        dragged: false,
        drag_stopped: false,
        is_pointer_button_down_on: false,
        interact_pointer_pos: None,
        changed: false,
    }
}

impl EmojiLabel {
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            wrap: None,
            truncate: false,
            sense: None,
            selectable: None,
        }
    }

    pub fn text(&self) -> &str {
        self.text.text()
    }

    /// If `true`, the text will wrap to stay within the max width of the [`Ui`].
    ///
    /// Calling `wrap` will override [`Self::truncate`].
    ///
    /// By default [`Self::wrap`] will be `true` in vertical layouts
    /// and horizontal layouts with wrapping,
    /// and `false` on non-wrapping horizontal layouts.
    ///
    /// Note that any `\n` in the text will always produce a new line.
    ///
    /// You can also use [`egui::Style::wrap`].
    #[inline]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = Some(wrap);
        self.truncate = false;
        self
    }

    /// If `true`, the text will stop at the max width of the [`Ui`],
    /// and what doesn't fit will be elided, replaced with `…`.
    ///
    /// If the text is truncated, the full text will be shown on hover as a tool-tip.
    ///
    /// Default is `false`, which means the text will expand the parent [`Ui`],
    /// or wrap if [`Self::wrap`] is set.
    ///
    /// Calling `truncate` will override [`Self::wrap`].
    #[inline]
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.wrap = None;
        self.truncate = truncate;
        self
    }

    /// Can the user select the text with the mouse?
    ///
    /// Overrides [`egui::style::Interaction::selectable_labels`].
    #[inline]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }

    /// Make the label respond to clicks and/or drags.
    ///
    /// By default, a label is inert and does not respond to click or drags.
    /// By calling this you can turn the label into a button of sorts.
    /// This will also give the label the hover-effect of a button, but without the frame.
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = Some(sense);
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let id = egui::Id::new(self.text());
        let mut state = LabelState::load(ui.ctx(), id, &self.text);

        if !state.is_saved {
            state.is_saved = true;
            state.clone().save(ui.ctx(), id);
        }

        let font_height = ui.text_style_height(&egui::TextStyle::Body);
        let mut resp = empty_response(ui.ctx().clone());

        ui.with_layout(
            Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
            |ui| {
                for segment in &state.segments {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    match segment {
                        TextSegment::Text(text) => {
                            let mut label = egui::Label::new(text.clone()).truncate(self.truncate);
                            if let Some(wrap) = self.wrap {
                                label = label.wrap(wrap);
                            }
                            if let Some(selectable) = self.selectable {
                                label = label.selectable(selectable);
                            }
                            if let Some(sense) = self.sense {
                                label = label.sense(sense);
                            }
                            resp |= ui.add(label);
                        }
                        TextSegment::Emoji(emoji) => {
                            let Some(source) = get_source_for_emoji(emoji) else {
                                continue;
                            };

                            let image_rect = ui
                                .add(
                                    egui::Image::new(source)
                                        .fit_to_exact_size(egui::vec2(font_height, font_height)),
                                )
                                .rect;

                            // for emoji selection and copying:
                            resp |= ui.put(
                                image_rect,
                                egui::Label::new(
                                    RichText::new(emoji).color(egui::Color32::TRANSPARENT),
                                ),
                            );
                        }
                    }
                }
            },
        );

        resp
    }
}

impl EmojiLabel {
    pub fn layout_in_ui(
        self,
        ui: &mut egui::Ui,
    ) -> (egui::Pos2, Arc<egui::Galley>, egui::Response) {
        let selectable = self
            .selectable
            .unwrap_or_else(|| ui.style().interaction.selectable_labels);

        let mut sense = self.sense.unwrap_or_else(|| {
            if ui.memory(|mem| mem.options.screen_reader) {
                // We only want to focus labels if the screen reader is on.
                Sense::focusable_noninteractive()
            } else {
                Sense::hover()
            }
        });

        if selectable {
            // On touch screens (e.g. mobile in `eframe` web), should
            // dragging select text, or scroll the enclosing [`ScrollArea`] (if any)?
            // Since currently copying selected text in not supported on `eframe` web,
            // we prioritize touch-scrolling:
            let allow_drag_to_select = ui.input(|i| !i.has_touch_screen());

            let mut select_sense = if allow_drag_to_select {
                Sense::click_and_drag()
            } else {
                Sense::click()
            };
            select_sense.focusable = false; // Don't move focus to labels with TAB key.

            sense = sense.union(select_sense);
        }

        let valign = ui.layout().vertical_align();
        let mut layout_job = LayoutJob::default();

        let id = egui::Id::new(self.text());
        let mut state = LabelState::load(ui.ctx(), id, &self.text);

        if !state.is_saved {
            state.is_saved = true;
            state.clone().save(ui.ctx(), id);
        }

        let font_height = ui.text_style_height(&egui::TextStyle::Body);

        for segment in &state.segments {
            match segment {
                TextSegment::Text(text) => {
                    text.clone().append_to(
                        &mut layout_job,
                        ui.style(),
                        egui::FontSelection::Default,
                        valign,
                    );
                }
                TextSegment::Emoji(emoji) => {
                    layout_job.append(emoji, -0.0, egui::TextFormat::default());
                }
            }
        }
        let truncate = self.truncate;
        let wrap = !truncate && self.wrap.unwrap_or_else(|| ui.wrap_text());
        let available_width = ui.available_width();

        if wrap
            && ui.layout().main_dir() == egui::Direction::LeftToRight
            && ui.layout().main_wrap()
            && available_width.is_finite()
        {
            // On a wrapping horizontal layout we want text to start after the previous widget,
            // then continue on the line below! This will take some extra work:

            let cursor = ui.cursor();
            let first_row_indentation = available_width - ui.available_size_before_wrap().x;
            egui::egui_assert!(first_row_indentation.is_finite());

            layout_job.wrap.max_width = available_width;
            layout_job.first_row_min_height = cursor.height();
            layout_job.halign = egui::Align::Min;
            layout_job.justify = false;
            if let Some(first_section) = layout_job.sections.first_mut() {
                first_section.leading_space = first_row_indentation;
            }
            let galley = ui.fonts(|fonts| fonts.layout_job(layout_job));

            let pos = egui::pos2(ui.max_rect().left(), ui.cursor().top());
            assert!(!galley.rows.is_empty(), "Galleys are never empty");
            // collect a response from many rows:
            let rect = galley.rows[0].rect.translate(egui::vec2(pos.x, pos.y));
            let mut response = ui.allocate_rect(rect, sense);
            for row in galley.rows.iter().skip(1) {
                let rect = row.rect.translate(egui::vec2(pos.x, pos.y));
                response |= ui.allocate_rect(rect, sense);
            }
            (pos, galley, response)
        } else {
            if truncate {
                layout_job.wrap.max_width = available_width;
                layout_job.wrap.max_rows = 1;
                layout_job.wrap.break_anywhere = true;
            } else if wrap {
                layout_job.wrap.max_width = available_width;
            } else {
                layout_job.wrap.max_width = f32::INFINITY;
            };

            // if ui.is_grid() {
            //     layout_job.halign = Align::LEFT;
            //     layout_job.justify = false;
            // } else {
            //     layout_job.halign = ui.layout().horizontal_placement();
            //     layout_job.justify = ui.layout().horizontal_justify();
            // };

            let galley = ui.fonts(|fonts| fonts.layout_job(layout_job));
            let (rect, response) = ui.allocate_exact_size(galley.size(), sense);
            let galley_pos = match galley.job.halign {
                egui::Align::LEFT => rect.left_top(),
                egui::Align::Center => rect.center_top(),
                egui::Align::RIGHT => rect.right_top(),
            };
            (galley_pos, galley, response)
        }
    }
}

impl egui::Widget for EmojiLabel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Interactive = the uses asked to sense interaction.
        // We DON'T want to have the color respond just because the text is selectable;
        // the cursor is enough to communicate that.
        let interactive = self.sense.map_or(false, |sense| sense != Sense::hover());

        let selectable = self.selectable;

        let (galley_pos, galley, mut response) = self.layout_in_ui(ui);
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, galley.text()));

        if ui.is_rect_visible(response.rect) {
            if galley.elided {
                // Show the full (non-elided) text on hover:
                response = response.on_hover_text(galley.text());
            }

            let response_color = if interactive {
                ui.style().interact(&response).text_color()
            } else {
                ui.style().visuals.text_color()
            };

            let underline = if response.has_focus() || response.highlighted() {
                egui::Stroke::new(1.0, response_color)
            } else {
                egui::Stroke::NONE
            };

            ui.painter().add(
                egui::epaint::TextShape::new(galley_pos, galley.clone(), response_color)
                    .with_underline(underline),
            );

            let selectable = selectable.unwrap_or_else(|| ui.style().interaction.selectable_labels);
            if selectable {
                egui::text_selection::LabelSelectionState::label_text_selection(
                    ui, &response, galley_pos, &galley,
                );
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_segmentation() {
        let text = "Hello😤world";
        let segments = segment_text(&RichText::new(text));
        assert!(
            segments
                == vec![
                    TextSegment::Text("Hello".into()),
                    TextSegment::Emoji("😤".to_owned()),
                    TextSegment::Text("world".into())
                ]
        );
        let text = "😅 2,*:привет|3 🤬";
        let segments = segment_text(&RichText::new(text));
        assert!(
            segments
                == vec![
                    TextSegment::Emoji("😅".to_owned()),
                    TextSegment::Text(" 2,*:привет|3 ".into()),
                    TextSegment::Emoji("🤬".to_owned()),
                ]
        );
        let text = "Hello world 🥰!";
        let segments = segment_text(&RichText::new(text));
        assert!(
            segments
                == vec![
                    TextSegment::Text("Hello world ".into()),
                    TextSegment::Emoji("🥰".to_owned()),
                    TextSegment::Text("!".into())
                ]
        );
    }
}
