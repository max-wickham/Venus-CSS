mod casandra;
// mod css_rules;
use css_rules::{assert_valid_color, assert_valid_dimensions, assert_valid_length, check_string_in_list};
// use casandra::style_sheet;
use std::collections::HashMap;
extern crate proc_macro;



macro_rules! css_key {
    (accent_color, $dict:expr, $value:expr) => {
        assert_valid_color!($value,{"initial","inherit","auto"});
        $dict.insert(String::from("accent-color"), ($value).to_string().clone())
    };
    (align_content, $dict:expr, $value:expr) => {
        check_string_in_list!($value, {"stretch","center","flex-start","flex-end","space-between","space-around","space-evenly","initial","inherit"});
        $dict.insert(String::from("align-content"), ($value).to_string().clone())
    };
    (align_items, $dict:expr, $value:expr) => {
        check_string_in_list!($value, {"normal","stretch","center","flex-start","flex-end","start","end","baseline","initial","inherit"});
        $dict.insert(String::from("align-items"), ($value).to_string().clone())
    };
    (align_self, $dict:expr, $value:expr) => {
        check_string_in_list!($value, {"auto","stretch","center","flex-start","flex-end","baseline","initial","inherit"});
        $dict.insert(String::from("align-self"), ($value).to_string().clone())
    };
    (alignment_adjust, $dict:expr, $value:expr) => {
        assert_valid_length!($value, {"auto","baseline","before-edge","text-before-edge","middle","central","after-edge","text-after-edge","ideographic","alphabetic","hanging","mathematical"});
        $dict.insert(
            String::from("alignment-adjust"),
            ($value).to_string().clone(),
        )
    };
    (alignment_baseline, $dict:expr, $value:expr) => {
        check_string_in_list!($value, {"auto","baseline","before-edge","text-before-edge","middle","central","after-edge","text-after-edge","ideographic","alphabetic","hanging","mathematical","top","center","bottom"});
        $dict.insert(
            String::from("alignment-baseline"),
            ($value).to_string().clone(),
        )
    };
    (all, $dict:expr, $value:expr) => {
        check_string_in_list!($value, {"initial","inherit","unset"});
        $dict.insert(String::from("all"), ($value).to_string().clone())
    };
    // (alt, $dict:expr, $value:expr) => {
    //     $dict.insert(String::from("alt"), ($value).to_string().clone())
    // };
    // TODO
    (animation, $dict:expr, $value:expr) => {
        $dict.insert(String::from("animation"), ($value).to_string().clone())
    };
    // TODO
    (animation_delay, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("animation-delay"),
            ($value).to_string().clone(),
        )
    };
    // TODO
    (animation_direction, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("animation-direction"),
            ($value).to_string().clone(),
        )
    };
    // TODO
    (animation_duration, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("animation-duration"),
            ($value).to_string().clone(),
        )
    };
    // TODO
    (animation_fill_mode, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("animation-fill-mode"),
            ($value).to_string().clone(),
        )
    };
    // TODO
    (animation_iteration_count, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("animation-iteration-count"),
            ($value).to_string().clone(),
        )
    };
    // TODO
    (animation_name, $dict:expr, $value:expr) => {
        $dict.insert(String::from("animation-name"), ($value).to_string().clone())
    };
    // TODO
    (animation_play_state, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("animation-play-state"),
            ($value).to_string().clone(),
        )
    };
    // TODO
    (animation_timing_function, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("animation-timing-function"),
            ($value).to_string().clone(),
        )
    };
    (azimuth, $dict:expr, $value:expr) => {
        $dict.insert(String::from("azimuth"), ($value).to_string().clone())
    };
    (backface_visibility, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("backface-visibility"),
            ($value).to_string().clone(),
        )
    };
    (background, $dict:expr, $value:expr) => {
        $dict.insert(String::from("background"), ($value).to_string().clone())
    };
    (background_attachment, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-attachment"),
            ($value).to_string().clone(),
        )
    };
    (background_clip, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-clip"),
            ($value).to_string().clone(),
        )
    };
    (background_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-color"),
            ($value).to_string().clone(),
        )
    };
    (background_image, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-image"),
            ($value).to_string().clone(),
        )
    };
    (background_origin, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-origin"),
            ($value).to_string().clone(),
        )
    };
    (background_position, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-position"),
            ($value).to_string().clone(),
        )
    };
    (background_repeat, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-repeat"),
            ($value).to_string().clone(),
        )
    };
    (background_size, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-size"),
            ($value).to_string().clone(),
        )
    };
    (background_blend_mode, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("background-blend-mode"),
            ($value).to_string().clone(),
        )
    };
    (baseline_shift, $dict:expr, $value:expr) => {
        $dict.insert(String::from("baseline-shift"), ($value).to_string().clone())
    };
    (bleed, $dict:expr, $value:expr) => {
        $dict.insert(String::from("bleed"), ($value).to_string().clone())
    };
    (bookmark_label, $dict:expr, $value:expr) => {
        $dict.insert(String::from("bookmark-label"), ($value).to_string().clone())
    };
    (bookmark_level, $dict:expr, $value:expr) => {
        $dict.insert(String::from("bookmark-level"), ($value).to_string().clone())
    };
    (bookmark_state, $dict:expr, $value:expr) => {
        $dict.insert(String::from("bookmark-state"), ($value).to_string().clone())
    };
    (border, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border"), ($value).to_string().clone())
    };
    (border_color, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-color"), ($value).to_string().clone())
    };
    (border_style, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-style"), ($value).to_string().clone())
    };
    (border_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-width"), ($value).to_string().clone())
    };
    (border_bottom, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-bottom"), ($value).to_string().clone())
    };
    (border_bottom_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-bottom-color"),
            ($value).to_string().clone(),
        )
    };
    (border_bottom_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-bottom-style"),
            ($value).to_string().clone(),
        )
    };
    (border_bottom_width, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-bottom-width"),
            ($value).to_string().clone(),
        )
    };
    (border_left, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-left"), ($value).to_string().clone())
    };
    (border_left_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-left-color"),
            ($value).to_string().clone(),
        )
    };
    (border_left_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-left-style"),
            ($value).to_string().clone(),
        )
    };
    (border_left_width, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-left-width"),
            ($value).to_string().clone(),
        )
    };
    (border_right, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-right"), ($value).to_string().clone())
    };
    (border_right_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-right-color"),
            ($value).to_string().clone(),
        )
    };
    (border_right_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-right-style"),
            ($value).to_string().clone(),
        )
    };
    (border_right_width, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-right-width"),
            ($value).to_string().clone(),
        )
    };
    (border_top, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-top"), ($value).to_string().clone())
    };
    (border_top_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-top-color"),
            ($value).to_string().clone(),
        )
    };
    (border_top_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-top-style"),
            ($value).to_string().clone(),
        )
    };
    (border_top_width, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-top-width"),
            ($value).to_string().clone(),
        )
    };
    (border_collapse, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-collapse"),
            ($value).to_string().clone(),
        )
    };
    (border_image, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-image"), ($value).to_string().clone())
    };
    (border_image_outset, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-image-outset"),
            ($value).to_string().clone(),
        )
    };
    (border_image_repeat, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-image-repeat"),
            ($value).to_string().clone(),
        )
    };
    (border_image_slice, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-image-slice"),
            ($value).to_string().clone(),
        )
    };
    (border_image_source, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-image-source"),
            ($value).to_string().clone(),
        )
    };
    (border_image_width, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-image-width"),
            ($value).to_string().clone(),
        )
    };
    (border_radius, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-radius"), ($value).to_string().clone())
    };
    (border_bottom_left_radius, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-bottom-left-radius"),
            ($value).to_string().clone(),
        )
    };
    (border_bottom_right_radius, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-bottom-right-radius"),
            ($value).to_string().clone(),
        )
    };
    (border_top_left_radius, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-top-left-radius"),
            ($value).to_string().clone(),
        )
    };
    (border_top_right_radius, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("border-top-right-radius"),
            ($value).to_string().clone(),
        )
    };
    (border_spacing, $dict:expr, $value:expr) => {
        $dict.insert(String::from("border-spacing"), ($value).to_string().clone())
    };
    (bottom, $dict:expr, $value:expr) => {
        $dict.insert(String::from("bottom"), ($value).to_string().clone())
    };
    (box_decoration_break, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("box-decoration-break"),
            ($value).to_string().clone(),
        )
    };
    (box_shadow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("box-shadow"), ($value).to_string().clone())
    };
    (box_sizing, $dict:expr, $value:expr) => {
        $dict.insert(String::from("box-sizing"), ($value).to_string().clone())
    };
    (box_snap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("box-snap"), ($value).to_string().clone())
    };
    (break_after, $dict:expr, $value:expr) => {
        $dict.insert(String::from("break-after"), ($value).to_string().clone())
    };
    (break_before, $dict:expr, $value:expr) => {
        $dict.insert(String::from("break-before"), ($value).to_string().clone())
    };
    (break_inside, $dict:expr, $value:expr) => {
        $dict.insert(String::from("break-inside"), ($value).to_string().clone())
    };
    (buffered_rendering, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("buffered-rendering"),
            ($value).to_string().clone(),
        )
    };
    (caption_side, $dict:expr, $value:expr) => {
        $dict.insert(String::from("caption-side"), ($value).to_string().clone())
    };
    (clear, $dict:expr, $value:expr) => {
        $dict.insert(String::from("clear"), ($value).to_string().clone())
    };
    (clear_side, $dict:expr, $value:expr) => {
        $dict.insert(String::from("clear-side"), ($value).to_string().clone())
    };
    (clip, $dict:expr, $value:expr) => {
        $dict.insert(String::from("clip"), ($value).to_string().clone())
    };
    (clip_path, $dict:expr, $value:expr) => {
        $dict.insert(String::from("clip-path"), ($value).to_string().clone())
    };
    (clip_rule, $dict:expr, $value:expr) => {
        $dict.insert(String::from("clip-rule"), ($value).to_string().clone())
    };
    (color, $dict:expr, $value:expr) => {
        $dict.insert(String::from("color"), ($value).to_string().clone())
    };
    (color_adjust, $dict:expr, $value:expr) => {
        $dict.insert(String::from("color-adjust"), ($value).to_string().clone())
    };
    (color_correction, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("color-correction"),
            ($value).to_string().clone(),
        )
    };
    (color_interpolation, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("color-interpolation"),
            ($value).to_string().clone(),
        )
    };
    (color_interpolation_filters, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("color-interpolation-filters"),
            ($value).to_string().clone(),
        )
    };
    (color_profile, $dict:expr, $value:expr) => {
        $dict.insert(String::from("color-profile"), ($value).to_string().clone())
    };
    (color_rendering, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("color-rendering"),
            ($value).to_string().clone(),
        )
    };
    (column_fill, $dict:expr, $value:expr) => {
        $dict.insert(String::from("column-fill"), ($value).to_string().clone())
    };
    (column_gap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("column-gap"), ($value).to_string().clone())
    };
    (column_rule, $dict:expr, $value:expr) => {
        $dict.insert(String::from("column-rule"), ($value).to_string().clone())
    };
    (column_rule_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("column-rule-color"),
            ($value).to_string().clone(),
        )
    };
    (column_rule_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("column-rule-style"),
            ($value).to_string().clone(),
        )
    };
    (column_rule_width, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("column-rule-width"),
            ($value).to_string().clone(),
        )
    };
    (column_span, $dict:expr, $value:expr) => {
        $dict.insert(String::from("column-span"), ($value).to_string().clone())
    };
    (columns, $dict:expr, $value:expr) => {
        $dict.insert(String::from("columns"), ($value).to_string().clone())
    };
    (column_count, $dict:expr, $value:expr) => {
        $dict.insert(String::from("column-count"), ($value).to_string().clone())
    };
    (column_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("column-width"), ($value).to_string().clone())
    };
    (contain, $dict:expr, $value:expr) => {
        $dict.insert(String::from("contain"), ($value).to_string().clone())
    };
    (content, $dict:expr, $value:expr) => {
        $dict.insert(String::from("content"), ($value).to_string().clone())
    };
    (counter_increment, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("counter-increment"),
            ($value).to_string().clone(),
        )
    };
    (counter_reset, $dict:expr, $value:expr) => {
        $dict.insert(String::from("counter-reset"), ($value).to_string().clone())
    };
    (counter_set, $dict:expr, $value:expr) => {
        $dict.insert(String::from("counter-set"), ($value).to_string().clone())
    };
    (cue, $dict:expr, $value:expr) => {
        $dict.insert(String::from("cue"), ($value).to_string().clone())
    };
    (cue_after, $dict:expr, $value:expr) => {
        $dict.insert(String::from("cue-after"), ($value).to_string().clone())
    };
    (cue_before, $dict:expr, $value:expr) => {
        $dict.insert(String::from("cue-before"), ($value).to_string().clone())
    };
    (cursor, $dict:expr, $value:expr) => {
        $dict.insert(String::from("cursor"), ($value).to_string().clone())
    };
    (direction, $dict:expr, $value:expr) => {
        $dict.insert(String::from("direction"), ($value).to_string().clone())
    };
    (display, $dict:expr, $value:expr) => {
        $dict.insert(String::from("display"), ($value).to_string().clone())
    };
    (display_inside, $dict:expr, $value:expr) => {
        $dict.insert(String::from("display-inside"), ($value).to_string().clone())
    };
    (display_outside, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("display-outside"),
            ($value).to_string().clone(),
        )
    };
    (display_extras, $dict:expr, $value:expr) => {
        $dict.insert(String::from("display-extras"), ($value).to_string().clone())
    };
    (display_box, $dict:expr, $value:expr) => {
        $dict.insert(String::from("display-box"), ($value).to_string().clone())
    };
    (dominant_baseline, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("dominant-baseline"),
            ($value).to_string().clone(),
        )
    };
    (elevation, $dict:expr, $value:expr) => {
        $dict.insert(String::from("elevation"), ($value).to_string().clone())
    };
    (empty_cells, $dict:expr, $value:expr) => {
        $dict.insert(String::from("empty-cells"), ($value).to_string().clone())
    };
    (enable_background, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("enable-background"),
            ($value).to_string().clone(),
        )
    };
    (fill, $dict:expr, $value:expr) => {
        $dict.insert(String::from("fill"), ($value).to_string().clone())
    };
    (fill_opacity, $dict:expr, $value:expr) => {
        $dict.insert(String::from("fill-opacity"), ($value).to_string().clone())
    };
    (fill_rule, $dict:expr, $value:expr) => {
        $dict.insert(String::from("fill-rule"), ($value).to_string().clone())
    };
    (filter, $dict:expr, $value:expr) => {
        $dict.insert(String::from("filter"), ($value).to_string().clone())
    };
    (float, $dict:expr, $value:expr) => {
        $dict.insert(String::from("float"), ($value).to_string().clone())
    };
    (float_defer_column, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("float-defer-column"),
            ($value).to_string().clone(),
        )
    };
    (float_defer_page, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("float-defer-page"),
            ($value).to_string().clone(),
        )
    };
    (float_offset, $dict:expr, $value:expr) => {
        $dict.insert(String::from("float-offset"), ($value).to_string().clone())
    };
    (float_wrap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("float-wrap"), ($value).to_string().clone())
    };
    (flow_into, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flow-into"), ($value).to_string().clone())
    };
    (flow_from, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flow-from"), ($value).to_string().clone())
    };
    (flex, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flex"), ($value).to_string().clone())
    };
    (flex_basis, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flex-basis"), ($value).to_string().clone())
    };
    (flex_grow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flex-grow"), ($value).to_string().clone())
    };
    (flex_shrink, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flex-shrink"), ($value).to_string().clone())
    };
    (flex_flow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flex-flow"), ($value).to_string().clone())
    };
    (flex_direction, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flex-direction"), ($value).to_string().clone())
    };
    (flex_wrap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flex-wrap"), ($value).to_string().clone())
    };
    (flood_color, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flood-color"), ($value).to_string().clone())
    };
    (flood_opacity, $dict:expr, $value:expr) => {
        $dict.insert(String::from("flood-opacity"), ($value).to_string().clone())
    };
    (font, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font"), ($value).to_string().clone())
    };
    (font_family, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-family"), ($value).to_string().clone())
    };
    (font_size, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-size"), ($value).to_string().clone())
    };
    (font_stretch, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-stretch"), ($value).to_string().clone())
    };
    (font_style, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-style"), ($value).to_string().clone())
    };
    (font_weight, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-weight"), ($value).to_string().clone())
    };
    (font_feature_settings, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-feature-settings"),
            ($value).to_string().clone(),
        )
    };
    (font_kerning, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-kerning"), ($value).to_string().clone())
    };
    (font_language_override, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-language-override"),
            ($value).to_string().clone(),
        )
    };
    (font_size_adjust, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-size-adjust"),
            ($value).to_string().clone(),
        )
    };
    (font_synthesis, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-synthesis"), ($value).to_string().clone())
    };
    (font_variant, $dict:expr, $value:expr) => {
        $dict.insert(String::from("font-variant"), ($value).to_string().clone())
    };
    (font_variant_alternates, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-variant-alternates"),
            ($value).to_string().clone(),
        )
    };
    (font_variant_caps, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-variant-caps"),
            ($value).to_string().clone(),
        )
    };
    (font_variant_east_asian, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-variant-east-asian"),
            ($value).to_string().clone(),
        )
    };
    (font_variant_ligatures, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-variant-ligatures"),
            ($value).to_string().clone(),
        )
    };
    (font_variant_numeric, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-variant-numeric"),
            ($value).to_string().clone(),
        )
    };
    (font_variant_position, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("font-variant-position"),
            ($value).to_string().clone(),
        )
    };
    (footnote_policy, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("footnote-policy"),
            ($value).to_string().clone(),
        )
    };
    (glyph_orientation_horizontal, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("glyph-orientation-horizontal"),
            ($value).to_string().clone(),
        )
    };
    (glyph_orientation_vertical, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("glyph-orientation-vertical"),
            ($value).to_string().clone(),
        )
    };
    (grid, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid"), ($value).to_string().clone())
    };
    (grid_auto_flow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-auto-flow"), ($value).to_string().clone())
    };
    (grid_auto_columns, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("grid-auto-columns"),
            ($value).to_string().clone(),
        )
    };
    (grid_auto_rows, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-auto-rows"), ($value).to_string().clone())
    };
    (grid_template, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-template"), ($value).to_string().clone())
    };
    (grid_template_areas, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("grid-template-areas"),
            ($value).to_string().clone(),
        )
    };
    (grid_template_columns, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("grid-template-columns"),
            ($value).to_string().clone(),
        )
    };
    (grid_template_rows, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("grid-template-rows"),
            ($value).to_string().clone(),
        )
    };
    (grid_area, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-area"), ($value).to_string().clone())
    };
    (grid_column, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-column"), ($value).to_string().clone())
    };
    (grid_column_start, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("grid-column-start"),
            ($value).to_string().clone(),
        )
    };
    (grid_column_end, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("grid-column-end"),
            ($value).to_string().clone(),
        )
    };
    (grid_row, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-row"), ($value).to_string().clone())
    };
    (grid_row_start, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-row-start"), ($value).to_string().clone())
    };
    (grid_row_end, $dict:expr, $value:expr) => {
        $dict.insert(String::from("grid-row-end"), ($value).to_string().clone())
    };
    (hanging_punctuation, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("hanging-punctuation"),
            ($value).to_string().clone(),
        )
    };
    (height, $dict:expr, $value:expr) => {
        $dict.insert(String::from("height"), ($value).to_string().clone())
    };
    (hyphenate_character, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("hyphenate-character"),
            ($value).to_string().clone(),
        )
    };
    (hyphenate_limit_chars, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("hyphenate-limit-chars"),
            ($value).to_string().clone(),
        )
    };
    (hyphenate_limit_last, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("hyphenate-limit-last"),
            ($value).to_string().clone(),
        )
    };
    (hyphenate_limit_lines, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("hyphenate-limit-lines"),
            ($value).to_string().clone(),
        )
    };
    (hyphenate_limit_zone, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("hyphenate-limit-zone"),
            ($value).to_string().clone(),
        )
    };
    (hyphens, $dict:expr, $value:expr) => {
        $dict.insert(String::from("hyphens"), ($value).to_string().clone())
    };
    (icon, $dict:expr, $value:expr) => {
        $dict.insert(String::from("icon"), ($value).to_string().clone())
    };
    (image_orientation, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("image-orientation"),
            ($value).to_string().clone(),
        )
    };
    (image_resolution, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("image-resolution"),
            ($value).to_string().clone(),
        )
    };
    (image_rendering, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("image-rendering"),
            ($value).to_string().clone(),
        )
    };
    (ime, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ime"), ($value).to_string().clone())
    };
    (ime_align, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ime-align"), ($value).to_string().clone())
    };
    (ime_mode, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ime-mode"), ($value).to_string().clone())
    };
    (ime_offset, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ime-offset"), ($value).to_string().clone())
    };
    (ime_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ime-width"), ($value).to_string().clone())
    };
    (initial_letters, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("initial-letters"),
            ($value).to_string().clone(),
        )
    };
    (inline_box_align, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("inline-box-align"),
            ($value).to_string().clone(),
        )
    };
    (isolation, $dict:expr, $value:expr) => {
        $dict.insert(String::from("isolation"), ($value).to_string().clone())
    };
    (justify_content, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("justify-content"),
            ($value).to_string().clone(),
        )
    };
    (justify_items, $dict:expr, $value:expr) => {
        $dict.insert(String::from("justify-items"), ($value).to_string().clone())
    };
    (justify_self, $dict:expr, $value:expr) => {
        $dict.insert(String::from("justify-self"), ($value).to_string().clone())
    };
    (kerning, $dict:expr, $value:expr) => {
        $dict.insert(String::from("kerning"), ($value).to_string().clone())
    };
    (left, $dict:expr, $value:expr) => {
        $dict.insert(String::from("left"), ($value).to_string().clone())
    };
    (letter_spacing, $dict:expr, $value:expr) => {
        $dict.insert(String::from("letter-spacing"), ($value).to_string().clone())
    };
    (lighting_color, $dict:expr, $value:expr) => {
        $dict.insert(String::from("lighting-color"), ($value).to_string().clone())
    };
    (line_box_contain, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("line-box-contain"),
            ($value).to_string().clone(),
        )
    };
    (line_break, $dict:expr, $value:expr) => {
        $dict.insert(String::from("line-break"), ($value).to_string().clone())
    };
    (line_grid, $dict:expr, $value:expr) => {
        $dict.insert(String::from("line-grid"), ($value).to_string().clone())
    };
    (line_height, $dict:expr, $value:expr) => {
        $dict.insert(String::from("line-height"), ($value).to_string().clone())
    };
    (line_slack, $dict:expr, $value:expr) => {
        $dict.insert(String::from("line-slack"), ($value).to_string().clone())
    };
    (line_snap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("line-snap"), ($value).to_string().clone())
    };
    (list_style, $dict:expr, $value:expr) => {
        $dict.insert(String::from("list-style"), ($value).to_string().clone())
    };
    (list_style_image, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("list-style-image"),
            ($value).to_string().clone(),
        )
    };
    (list_style_position, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("list-style-position"),
            ($value).to_string().clone(),
        )
    };
    (list_style_type, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("list-style-type"),
            ($value).to_string().clone(),
        )
    };
    (margin, $dict:expr, $value:expr) => {
        $dict.insert(String::from("margin"), ($value).to_string().clone())
    };
    (margin_bottom, $dict:expr, $value:expr) => {
        $dict.insert(String::from("margin-bottom"), ($value).to_string().clone())
    };
    (margin_left, $dict:expr, $value:expr) => {
        $dict.insert(String::from("margin-left"), ($value).to_string().clone())
    };
    (margin_right, $dict:expr, $value:expr) => {
        $dict.insert(String::from("margin-right"), ($value).to_string().clone())
    };
    (margin_top, $dict:expr, $value:expr) => {
        $dict.insert(String::from("margin-top"), ($value).to_string().clone())
    };
    (marker, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marker"), ($value).to_string().clone())
    };
    (marker_end, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marker-end"), ($value).to_string().clone())
    };
    (marker_mid, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marker-mid"), ($value).to_string().clone())
    };
    (marker_pattern, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marker-pattern"), ($value).to_string().clone())
    };
    (marker_segment, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marker-segment"), ($value).to_string().clone())
    };
    (marker_start, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marker-start"), ($value).to_string().clone())
    };
    (marker_knockout_left, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("marker-knockout-left"),
            ($value).to_string().clone(),
        )
    };
    (marker_knockout_right, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("marker-knockout-right"),
            ($value).to_string().clone(),
        )
    };
    (marker_side, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marker-side"), ($value).to_string().clone())
    };
    (marks, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marks"), ($value).to_string().clone())
    };
    (marquee_direction, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("marquee-direction"),
            ($value).to_string().clone(),
        )
    };
    (marquee_play_count, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("marquee-play-count"),
            ($value).to_string().clone(),
        )
    };
    (marquee_speed, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marquee-speed"), ($value).to_string().clone())
    };
    (marquee_style, $dict:expr, $value:expr) => {
        $dict.insert(String::from("marquee-style"), ($value).to_string().clone())
    };
    (mask, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask"), ($value).to_string().clone())
    };
    (mask_image, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-image"), ($value).to_string().clone())
    };
    (mask_repeat, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-repeat"), ($value).to_string().clone())
    };
    (mask_position, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-position"), ($value).to_string().clone())
    };
    (mask_clip, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-clip"), ($value).to_string().clone())
    };
    (mask_origin, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-origin"), ($value).to_string().clone())
    };
    (mask_size, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-size"), ($value).to_string().clone())
    };
    (mask_box, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-box"), ($value).to_string().clone())
    };
    (mask_box_outset, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("mask-box-outset"),
            ($value).to_string().clone(),
        )
    };
    (mask_box_repeat, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("mask-box-repeat"),
            ($value).to_string().clone(),
        )
    };
    (mask_box_slice, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-box-slice"), ($value).to_string().clone())
    };
    (mask_box_source, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("mask-box-source"),
            ($value).to_string().clone(),
        )
    };
    (mask_box_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-box-width"), ($value).to_string().clone())
    };
    (mask_type, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mask-type"), ($value).to_string().clone())
    };
    (max_height, $dict:expr, $value:expr) => {
        $dict.insert(String::from("max-height"), ($value).to_string().clone())
    };
    (max_lines, $dict:expr, $value:expr) => {
        $dict.insert(String::from("max-lines"), ($value).to_string().clone())
    };
    (max_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("max-width"), ($value).to_string().clone())
    };
    (min_height, $dict:expr, $value:expr) => {
        $dict.insert(String::from("min-height"), ($value).to_string().clone())
    };
    (min_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("min-width"), ($value).to_string().clone())
    };
    (mix_blend_mode, $dict:expr, $value:expr) => {
        $dict.insert(String::from("mix-blend-mode"), ($value).to_string().clone())
    };
    (nav_down, $dict:expr, $value:expr) => {
        $dict.insert(String::from("nav-down"), ($value).to_string().clone())
    };
    (nav_index, $dict:expr, $value:expr) => {
        $dict.insert(String::from("nav-index"), ($value).to_string().clone())
    };
    (nav_left, $dict:expr, $value:expr) => {
        $dict.insert(String::from("nav-left"), ($value).to_string().clone())
    };
    (nav_right, $dict:expr, $value:expr) => {
        $dict.insert(String::from("nav-right"), ($value).to_string().clone())
    };
    (nav_up, $dict:expr, $value:expr) => {
        $dict.insert(String::from("nav-up"), ($value).to_string().clone())
    };
    (object_fit, $dict:expr, $value:expr) => {
        $dict.insert(String::from("object-fit"), ($value).to_string().clone())
    };
    (object_position, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("object-position"),
            ($value).to_string().clone(),
        )
    };
    (offset_after, $dict:expr, $value:expr) => {
        $dict.insert(String::from("offset-after"), ($value).to_string().clone())
    };
    (offset_before, $dict:expr, $value:expr) => {
        $dict.insert(String::from("offset-before"), ($value).to_string().clone())
    };
    (offset_end, $dict:expr, $value:expr) => {
        $dict.insert(String::from("offset-end"), ($value).to_string().clone())
    };
    (offset_start, $dict:expr, $value:expr) => {
        $dict.insert(String::from("offset-start"), ($value).to_string().clone())
    };
    (opacity, $dict:expr, $value:expr) => {
        $dict.insert(String::from("opacity"), ($value).to_string().clone())
    };
    (order, $dict:expr, $value:expr) => {
        $dict.insert(String::from("order"), ($value).to_string().clone())
    };
    (orphans, $dict:expr, $value:expr) => {
        $dict.insert(String::from("orphans"), ($value).to_string().clone())
    };
    (outline, $dict:expr, $value:expr) => {
        $dict.insert(String::from("outline"), ($value).to_string().clone())
    };
    (outline_color, $dict:expr, $value:expr) => {
        $dict.insert(String::from("outline-color"), ($value).to_string().clone())
    };
    (outline_style, $dict:expr, $value:expr) => {
        $dict.insert(String::from("outline-style"), ($value).to_string().clone())
    };
    (outline_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("outline-width"), ($value).to_string().clone())
    };
    (outline_offset, $dict:expr, $value:expr) => {
        $dict.insert(String::from("outline-offset"), ($value).to_string().clone())
    };
    (overflow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("overflow"), ($value).to_string().clone())
    };
    (overflow_x, $dict:expr, $value:expr) => {
        $dict.insert(String::from("overflow-x"), ($value).to_string().clone())
    };
    (overflow_y, $dict:expr, $value:expr) => {
        $dict.insert(String::from("overflow-y"), ($value).to_string().clone())
    };
    (overflow_style, $dict:expr, $value:expr) => {
        $dict.insert(String::from("overflow-style"), ($value).to_string().clone())
    };
    (overflow_wrap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("overflow-wrap"), ($value).to_string().clone())
    };
    (padding, $dict:expr, $value:expr) => {
        $dict.insert(String::from("padding"), ($value).to_string().clone())
    };
    (padding_bottom, $dict:expr, $value:expr) => {
        $dict.insert(String::from("padding-bottom"), ($value).to_string().clone())
    };
    (padding_left, $dict:expr, $value:expr) => {
        $dict.insert(String::from("padding-left"), ($value).to_string().clone())
    };
    (padding_right, $dict:expr, $value:expr) => {
        $dict.insert(String::from("padding-right"), ($value).to_string().clone())
    };
    (padding_top, $dict:expr, $value:expr) => {
        $dict.insert(String::from("padding-top"), ($value).to_string().clone())
    };
    (page, $dict:expr, $value:expr) => {
        $dict.insert(String::from("page"), ($value).to_string().clone())
    };
    (page_break_after, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("page-break-after"),
            ($value).to_string().clone(),
        )
    };
    (page_break_before, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("page-break-before"),
            ($value).to_string().clone(),
        )
    };
    (page_break_inside, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("page-break-inside"),
            ($value).to_string().clone(),
        )
    };
    (paint_order, $dict:expr, $value:expr) => {
        $dict.insert(String::from("paint-order"), ($value).to_string().clone())
    };
    (pause, $dict:expr, $value:expr) => {
        $dict.insert(String::from("pause"), ($value).to_string().clone())
    };
    (pause_after, $dict:expr, $value:expr) => {
        $dict.insert(String::from("pause-after"), ($value).to_string().clone())
    };
    (pause_before, $dict:expr, $value:expr) => {
        $dict.insert(String::from("pause-before"), ($value).to_string().clone())
    };
    (perspective, $dict:expr, $value:expr) => {
        $dict.insert(String::from("perspective"), ($value).to_string().clone())
    };
    (perspective_origin, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("perspective-origin"),
            ($value).to_string().clone(),
        )
    };
    (pitch, $dict:expr, $value:expr) => {
        $dict.insert(String::from("pitch"), ($value).to_string().clone())
    };
    (pitch_range, $dict:expr, $value:expr) => {
        $dict.insert(String::from("pitch-range"), ($value).to_string().clone())
    };
    (play_during, $dict:expr, $value:expr) => {
        $dict.insert(String::from("play-during"), ($value).to_string().clone())
    };
    (pointer_events, $dict:expr, $value:expr) => {
        $dict.insert(String::from("pointer-events"), ($value).to_string().clone())
    };
    (position, $dict:expr, $value:expr) => {
        $dict.insert(String::from("position"), ($value).to_string().clone())
    };
    (quotes, $dict:expr, $value:expr) => {
        $dict.insert(String::from("quotes"), ($value).to_string().clone())
    };
    (region_fragment, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("region-fragment"),
            ($value).to_string().clone(),
        )
    };
    (resize, $dict:expr, $value:expr) => {
        $dict.insert(String::from("resize"), ($value).to_string().clone())
    };
    (rest, $dict:expr, $value:expr) => {
        $dict.insert(String::from("rest"), ($value).to_string().clone())
    };
    (rest_after, $dict:expr, $value:expr) => {
        $dict.insert(String::from("rest-after"), ($value).to_string().clone())
    };
    (rest_before, $dict:expr, $value:expr) => {
        $dict.insert(String::from("rest-before"), ($value).to_string().clone())
    };
    (richness, $dict:expr, $value:expr) => {
        $dict.insert(String::from("richness"), ($value).to_string().clone())
    };
    (right, $dict:expr, $value:expr) => {
        $dict.insert(String::from("right"), ($value).to_string().clone())
    };
    (ruby_align, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ruby-align"), ($value).to_string().clone())
    };
    (ruby_merge, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ruby-merge"), ($value).to_string().clone())
    };
    (ruby_position, $dict:expr, $value:expr) => {
        $dict.insert(String::from("ruby-position"), ($value).to_string().clone())
    };
    (scroll_behavior, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("scroll-behavior"),
            ($value).to_string().clone(),
        )
    };
    (scroll_snap_coordinate, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("scroll-snap-coordinate"),
            ($value).to_string().clone(),
        )
    };
    (scroll_snap_destination, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("scroll-snap-destination"),
            ($value).to_string().clone(),
        )
    };
    (scroll_snap_points_x, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("scroll-snap-points-x"),
            ($value).to_string().clone(),
        )
    };
    (scroll_snap_points_y, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("scroll-snap-points-y"),
            ($value).to_string().clone(),
        )
    };
    (scroll_snap_type, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("scroll-snap-type"),
            ($value).to_string().clone(),
        )
    };
    (shape_image_threshold, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("shape-image-threshold"),
            ($value).to_string().clone(),
        )
    };
    (shape_inside, $dict:expr, $value:expr) => {
        $dict.insert(String::from("shape-inside"), ($value).to_string().clone())
    };
    (shape_margin, $dict:expr, $value:expr) => {
        $dict.insert(String::from("shape-margin"), ($value).to_string().clone())
    };
    (shape_outside, $dict:expr, $value:expr) => {
        $dict.insert(String::from("shape-outside"), ($value).to_string().clone())
    };
    (shape_padding, $dict:expr, $value:expr) => {
        $dict.insert(String::from("shape-padding"), ($value).to_string().clone())
    };
    (shape_rendering, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("shape-rendering"),
            ($value).to_string().clone(),
        )
    };
    (size, $dict:expr, $value:expr) => {
        $dict.insert(String::from("size"), ($value).to_string().clone())
    };
    (speak, $dict:expr, $value:expr) => {
        $dict.insert(String::from("speak"), ($value).to_string().clone())
    };
    (speak_as, $dict:expr, $value:expr) => {
        $dict.insert(String::from("speak-as"), ($value).to_string().clone())
    };
    (speak_header, $dict:expr, $value:expr) => {
        $dict.insert(String::from("speak-header"), ($value).to_string().clone())
    };
    (speak_numeral, $dict:expr, $value:expr) => {
        $dict.insert(String::from("speak-numeral"), ($value).to_string().clone())
    };
    (speak_punctuation, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("speak-punctuation"),
            ($value).to_string().clone(),
        )
    };
    (speech_rate, $dict:expr, $value:expr) => {
        $dict.insert(String::from("speech-rate"), ($value).to_string().clone())
    };
    (stop_color, $dict:expr, $value:expr) => {
        $dict.insert(String::from("stop-color"), ($value).to_string().clone())
    };
    (stop_opacity, $dict:expr, $value:expr) => {
        $dict.insert(String::from("stop-opacity"), ($value).to_string().clone())
    };
    (stress, $dict:expr, $value:expr) => {
        $dict.insert(String::from("stress"), ($value).to_string().clone())
    };
    (string_set, $dict:expr, $value:expr) => {
        $dict.insert(String::from("string-set"), ($value).to_string().clone())
    };
    (stroke, $dict:expr, $value:expr) => {
        $dict.insert(String::from("stroke"), ($value).to_string().clone())
    };
    (stroke_dasharray, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("stroke-dasharray"),
            ($value).to_string().clone(),
        )
    };
    (stroke_dashoffset, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("stroke-dashoffset"),
            ($value).to_string().clone(),
        )
    };
    (stroke_linecap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("stroke-linecap"), ($value).to_string().clone())
    };
    (stroke_linejoin, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("stroke-linejoin"),
            ($value).to_string().clone(),
        )
    };
    (stroke_miterlimit, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("stroke-miterlimit"),
            ($value).to_string().clone(),
        )
    };
    (stroke_opacity, $dict:expr, $value:expr) => {
        $dict.insert(String::from("stroke-opacity"), ($value).to_string().clone())
    };
    (stroke_width, $dict:expr, $value:expr) => {
        $dict.insert(String::from("stroke-width"), ($value).to_string().clone())
    };
    (tab_size, $dict:expr, $value:expr) => {
        $dict.insert(String::from("tab-size"), ($value).to_string().clone())
    };
    (table_layout, $dict:expr, $value:expr) => {
        $dict.insert(String::from("table-layout"), ($value).to_string().clone())
    };
    (text_align, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-align"), ($value).to_string().clone())
    };
    (text_align_all, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-align-all"), ($value).to_string().clone())
    };
    (text_align_last, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-align-last"),
            ($value).to_string().clone(),
        )
    };
    (text_anchor, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-anchor"), ($value).to_string().clone())
    };
    (text_combine_upright, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-combine-upright"),
            ($value).to_string().clone(),
        )
    };
    (text_decoration, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-decoration"),
            ($value).to_string().clone(),
        )
    };
    (text_decoration_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-decoration-color"),
            ($value).to_string().clone(),
        )
    };
    (text_decoration_line, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-decoration-line"),
            ($value).to_string().clone(),
        )
    };
    (text_decoration_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-decoration-style"),
            ($value).to_string().clone(),
        )
    };
    (text_decoration_skip, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-decoration-skip"),
            ($value).to_string().clone(),
        )
    };
    (text_emphasis, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-emphasis"), ($value).to_string().clone())
    };
    (text_emphasis_color, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-emphasis-color"),
            ($value).to_string().clone(),
        )
    };
    (text_emphasis_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-emphasis-style"),
            ($value).to_string().clone(),
        )
    };
    (text_emphasis_position, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-emphasis-position"),
            ($value).to_string().clone(),
        )
    };
    (text_emphasis_skip, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-emphasis-skip"),
            ($value).to_string().clone(),
        )
    };
    (text_height, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-height"), ($value).to_string().clone())
    };
    (text_indent, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-indent"), ($value).to_string().clone())
    };
    (text_justify, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-justify"), ($value).to_string().clone())
    };
    (text_orientation, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-orientation"),
            ($value).to_string().clone(),
        )
    };
    (text_overflow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-overflow"), ($value).to_string().clone())
    };
    (text_rendering, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-rendering"), ($value).to_string().clone())
    };
    (text_shadow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-shadow"), ($value).to_string().clone())
    };
    (text_size_adjust, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-size-adjust"),
            ($value).to_string().clone(),
        )
    };
    (text_space_collapse, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-space-collapse"),
            ($value).to_string().clone(),
        )
    };
    (text_spacing, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-spacing"), ($value).to_string().clone())
    };
    (text_transform, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-transform"), ($value).to_string().clone())
    };
    (text_underline_position, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("text-underline-position"),
            ($value).to_string().clone(),
        )
    };
    (text_wrap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("text-wrap"), ($value).to_string().clone())
    };
    (top, $dict:expr, $value:expr) => {
        $dict.insert(String::from("top"), ($value).to_string().clone())
    };
    (touch_action, $dict:expr, $value:expr) => {
        $dict.insert(String::from("touch-action"), ($value).to_string().clone())
    };
    (transform, $dict:expr, $value:expr) => {
        $dict.insert(String::from("transform"), ($value).to_string().clone())
    };
    (transform_box, $dict:expr, $value:expr) => {
        $dict.insert(String::from("transform-box"), ($value).to_string().clone())
    };
    (transform_origin, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("transform-origin"),
            ($value).to_string().clone(),
        )
    };
    (transform_style, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("transform-style"),
            ($value).to_string().clone(),
        )
    };
    (transition, $dict:expr, $value:expr) => {
        $dict.insert(String::from("transition"), ($value).to_string().clone())
    };
    (transition_delay, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("transition-delay"),
            ($value).to_string().clone(),
        )
    };
    (transition_duration, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("transition-duration"),
            ($value).to_string().clone(),
        )
    };
    (transition_property, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("transition-property"),
            ($value).to_string().clone(),
        )
    };
    (transition_timing_function, $dict:expr, $value:expr) => {
        $dict.insert(
            String::from("transition-timing-function"),
            ($value).to_string().clone(),
        )
    };
    (unicode_bidi, $dict:expr, $value:expr) => {
        $dict.insert(String::from("unicode-bidi"), ($value).to_string().clone())
    };
    (vector_effect, $dict:expr, $value:expr) => {
        $dict.insert(String::from("vector-effect"), ($value).to_string().clone())
    };
    (vertical_align, $dict:expr, $value:expr) => {
        $dict.insert(String::from("vertical-align"), ($value).to_string().clone())
    };
    (visibility, $dict:expr, $value:expr) => {
        $dict.insert(String::from("visibility"), ($value).to_string().clone())
    };
    (voice_balance, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-balance"), ($value).to_string().clone())
    };
    (voice_duration, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-duration"), ($value).to_string().clone())
    };
    (voice_family, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-family"), ($value).to_string().clone())
    };
    (voice_pitch, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-pitch"), ($value).to_string().clone())
    };
    (voice_range, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-range"), ($value).to_string().clone())
    };
    (voice_rate, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-rate"), ($value).to_string().clone())
    };
    (voice_stress, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-stress"), ($value).to_string().clone())
    };
    (voice_volumn, $dict:expr, $value:expr) => {
        $dict.insert(String::from("voice-volumn"), ($value).to_string().clone())
    };
    (volume, $dict:expr, $value:expr) => {
        $dict.insert(String::from("volume"), ($value).to_string().clone())
    };
    (white_space, $dict:expr, $value:expr) => {
        $dict.insert(String::from("white-space"), ($value).to_string().clone())
    };
    (widows, $dict:expr, $value:expr) => {
        $dict.insert(String::from("widows"), ($value).to_string().clone())
    };
    (width, $dict:expr, $value:expr) => {
        assert_valid_dimensions!($value, {});
        $dict.insert(String::from("width"), ($value).to_string().clone())
    };
    (will_change, $dict:expr, $value:expr) => {
        $dict.insert(String::from("will-change"), ($value).to_string().clone())
    };
    (word_break, $dict:expr, $value:expr) => {
        $dict.insert(String::from("word-break"), ($value).to_string().clone())
    };
    (word_spacing, $dict:expr, $value:expr) => {
        $dict.insert(String::from("word-spacing"), ($value).to_string().clone())
    };
    (word_wrap, $dict:expr, $value:expr) => {
        $dict.insert(String::from("word-wrap"), ($value).to_string().clone())
    };
    (wrap_flow, $dict:expr, $value:expr) => {
        $dict.insert(String::from("wrap-flow"), ($value).to_string().clone())
    };
    (wrap_through, $dict:expr, $value:expr) => {
        $dict.insert(String::from("wrap-through"), ($value).to_string().clone())
    };
    (writing_mode, $dict:expr, $value:expr) => {
        $dict.insert(String::from("writing-mode"), ($value).to_string().clone())
    };
    (z_index, $dict:expr, $value:expr) => {
        $dict.insert(String::from("z-index"), ($value).to_string().clone())
    };
}

macro_rules! css_dict {

    ($($key:ident : $value:expr),* $(,)?) => {
        {
        let mut temp_map: HashMap<String,String> = HashMap::new();
        $(
            css_key!($key, temp_map, $value);
        )*
        temp_map
        }
    };

}

fn main() {
    // let x: i32 = 15;
    let styles: HashMap<String, String> = css_dict! {
        color: "black",
        width: "10em 10px 10% 10",
        align_content: "initial",
    };

    // Display the generated HashMap
    for (key, value) in styles.iter() {
        println!("{}: {}", key, value);
    }
}

// procedural macro
// can be used to check the keys are valid
// can be used to check the result is valid
// provide warning if not valid
// output final hashmap
// convert hashmap to css string

// return object with list of css class names
