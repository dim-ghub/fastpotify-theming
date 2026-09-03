//! Shared palette, typography, icons, and base widgets.
//!
//! Inter provides real font weights, and Lucide provides a consistent icon set.
//! All colors use [`Palette`] so light, dark, and album-art-tinted themes stay
//! consistent.

use egui::{Color32, CornerRadius, Response, Sense, Stroke, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub dark: bool,
    pub window: Color32,
    pub panel: Color32,
    pub surface: Color32,
    pub surface_hover: Color32,
    pub surface_active: Color32,
    pub outline: Color32,
    pub text: Color32,
    pub secondary: Color32,
    pub dim: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub on_accent: Color32,
    pub danger: Color32,
    pub warning: Color32,
    pub overlay: Color32,
    pub shadow: Color32,

    // Material 3 / Astra design tokens
    pub surface_container_lowest: Color32,
    pub surface_container_low: Color32,
    pub surface_container: Color32,
    pub surface_container_high: Color32,
    pub surface_container_highest: Color32,
    pub primary_container: Color32,
    pub on_primary_container: Color32,
    pub outline_variant: Color32,
}

impl Palette {
    pub const fn default_dark() -> Self {
        Self {
            dark: true,
            window: Color32::from_rgb(0x0a, 0x0f, 0x0f),
            panel: Color32::from_rgb(0x0e, 0x15, 0x14),
            surface: Color32::from_rgb(0x13, 0x1b, 0x1a),
            surface_hover: Color32::from_rgb(0x19, 0x21, 0x20),
            surface_active: Color32::from_rgb(0x1d, 0x28, 0x27),
            outline: Color32::from_rgb(0x6d, 0x78, 0x76),
            text: Color32::from_rgb(0xdc, 0xe8, 0xe6),
            secondary: Color32::from_rgb(0xa2, 0xad, 0xac),
            dim: Color32::from_rgb(0x6d, 0x78, 0x76),
            accent: Color32::from_rgb(0x9b, 0xd0, 0xcc),
            accent_hover: Color32::from_rgb(0xb8, 0xed, 0xe9),
            on_accent: Color32::from_rgb(0x0d, 0x48, 0x45),
            danger: Color32::from_rgb(0xfa, 0x74, 0x6f),
            warning: Color32::from_rgb(0xf2, 0xb8, 0x5c),
            overlay: Color32::from_rgb(0x13, 0x1b, 0x1a),
            shadow: Color32::from_black_alpha(140),

            surface_container_lowest: Color32::from_rgb(0x00, 0x00, 0x00),
            surface_container_low: Color32::from_rgb(0x0e, 0x15, 0x14),
            surface_container: Color32::from_rgb(0x13, 0x1b, 0x1a),
            surface_container_high: Color32::from_rgb(0x19, 0x21, 0x20),
            surface_container_highest: Color32::from_rgb(0x1d, 0x28, 0x27),
            primary_container: Color32::from_rgb(0x25, 0x5b, 0x58),
            on_primary_container: Color32::from_rgb(0xb8, 0xed, 0xe9),
            outline_variant: Color32::from_rgb(0x3f, 0x4a, 0x49),
        }
    }

    pub fn dark() -> Self {
        load_caelestia_palette().unwrap_or_else(Self::default_dark)
    }

    pub fn light() -> Self {
        Self {
            dark: false,
            window: Color32::from_rgb(0xf4, 0xfa, 0xf8),
            panel: Color32::from_rgb(0xee, 0xf5, 0xf3),
            surface: Color32::from_rgb(0xe8, 0xef, 0xed),
            surface_hover: Color32::from_rgb(0xe2, 0xe9, 0xe7),
            surface_active: Color32::from_rgb(0xdc, 0xe4, 0xe1),
            outline: Color32::from_rgb(0x6f, 0x79, 0x78),
            text: Color32::from_rgb(0x16, 0x1d, 0x1c),
            secondary: Color32::from_rgb(0x3f, 0x49, 0x48),
            dim: Color32::from_rgb(0x6f, 0x79, 0x78),
            accent: Color32::from_rgb(0x33, 0x67, 0x64),
            accent_hover: Color32::from_rgb(0x25, 0x5b, 0x58),
            on_accent: Color32::WHITE,
            danger: Color32::from_rgb(0xba, 0x1a, 0x1a),
            warning: Color32::from_rgb(0xb8, 0x7a, 0x14),
            overlay: Color32::from_rgb(0xe8, 0xef, 0xed),
            shadow: Color32::from_black_alpha(40),

            surface_container_lowest: Color32::WHITE,
            surface_container_low: Color32::from_rgb(0xee, 0xf5, 0xf3),
            surface_container: Color32::from_rgb(0xe8, 0xef, 0xed),
            surface_container_high: Color32::from_rgb(0xe2, 0xe9, 0xe7),
            surface_container_highest: Color32::from_rgb(0xdc, 0xe4, 0xe1),
            primary_container: Color32::from_rgb(0xb8, 0xed, 0xe9),
            on_primary_container: Color32::from_rgb(0x00, 0x20, 0x1f),
            outline_variant: Color32::from_rgb(0xbe, 0xc9, 0xc7),
        }
    }

    /// A colour derived from album art, softened so it can sit behind text.
    pub fn tint_from_art(&self, rgb: [u8; 3]) -> Color32 {
        let [r, g, b] = rgb.map(|c| c as f32 / 255.0);
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let lightness = (max + min) / 2.0;
        let target = if self.dark { 0.30 } else { 0.72 };
        let (r, g, b) = if lightness < 0.01 {
            (target, target, target)
        } else {
            let scale = target / lightness;
            (
                (r * scale).min(1.0),
                (g * scale).min(1.0),
                (b * scale).min(1.0),
            )
        };
        Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

/// A serializable colour scheme that can be saved to and loaded from a JSON
/// file.  Hex strings like `"#1ed760"` are accepted for each colour; missing
/// fields fall back to the built-in dark palette so partial overrides work.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorScheme {
    /// Human-readable name (stored in the file, not derived from the filename).
    pub name: String,
    pub dark: bool,
    pub window: String,
    pub panel: String,
    pub surface: String,
    pub surface_hover: String,
    pub surface_active: String,
    pub outline: String,
    pub text: String,
    pub secondary: String,
    pub dim: String,
    pub accent: String,
    pub accent_hover: String,
    pub on_accent: String,
    pub danger: String,
    pub warning: String,
    pub overlay: String,
    pub shadow: String,

    #[serde(default)]
    pub surface_container_lowest: String,
    #[serde(default)]
    pub surface_container_low: String,
    #[serde(default)]
    pub surface_container: String,
    #[serde(default)]
    pub surface_container_high: String,
    #[serde(default)]
    pub surface_container_highest: String,
    #[serde(default)]
    pub primary_container: String,
    #[serde(default)]
    pub on_primary_container: String,
    #[serde(default)]
    pub outline_variant: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        let p = Palette::dark();
        Self {
            name: String::new(),
            dark: true,
            window: hex(p.window),
            panel: hex(p.panel),
            surface: hex(p.surface),
            surface_hover: hex(p.surface_hover),
            surface_active: hex(p.surface_active),
            outline: hex(p.outline),
            text: hex(p.text),
            secondary: hex(p.secondary),
            dim: hex(p.dim),
            accent: hex(p.accent),
            accent_hover: hex(p.accent_hover),
            on_accent: hex(p.on_accent),
            danger: hex(p.danger),
            warning: hex(p.warning),
            overlay: hex(p.overlay),
            shadow: hex_shadow(p.shadow),
            surface_container_lowest: hex(p.surface_container_lowest),
            surface_container_low: hex(p.surface_container_low),
            surface_container: hex(p.surface_container),
            surface_container_high: hex(p.surface_container_high),
            surface_container_highest: hex(p.surface_container_highest),
            primary_container: hex(p.primary_container),
            on_primary_container: hex(p.on_primary_container),
            outline_variant: hex(p.outline_variant),
        }
    }
}

impl ColorScheme {
    /// Build a [`Palette`] from this scheme.
    pub fn to_palette(&self) -> Palette {
        let def = if self.dark { Palette::dark() } else { Palette::light() };
        Palette {
            dark: self.dark,
            window: parse_color_or(&self.window, def.window),
            panel: parse_color_or(&self.panel, def.panel),
            surface: parse_color_or(&self.surface, def.surface),
            surface_hover: parse_color_or(&self.surface_hover, def.surface_hover),
            surface_active: parse_color_or(&self.surface_active, def.surface_active),
            outline: parse_color_or(&self.outline, def.outline),
            text: parse_color_or(&self.text, def.text),
            secondary: parse_color_or(&self.secondary, def.secondary),
            dim: parse_color_or(&self.dim, def.dim),
            accent: parse_color_or(&self.accent, def.accent),
            accent_hover: parse_color_or(&self.accent_hover, def.accent_hover),
            on_accent: parse_color_or(&self.on_accent, def.on_accent),
            danger: parse_color_or(&self.danger, def.danger),
            warning: parse_color_or(&self.warning, def.warning),
            overlay: parse_color_or(&self.overlay, def.overlay),
            shadow: parse_color_alpha_or(&self.shadow, def.shadow),
            surface_container_lowest: parse_color_or(&self.surface_container_lowest, def.surface_container_lowest),
            surface_container_low: parse_color_or(&self.surface_container_low, def.surface_container_low),
            surface_container: parse_color_or(&self.surface_container, def.surface_container),
            surface_container_high: parse_color_or(&self.surface_container_high, def.surface_container_high),
            surface_container_highest: parse_color_or(&self.surface_container_highest, def.surface_container_highest),
            primary_container: parse_color_or(&self.primary_container, def.primary_container),
            on_primary_container: parse_color_or(&self.on_primary_container, def.on_primary_container),
            outline_variant: parse_color_or(&self.outline_variant, def.outline_variant),
        }
    }

    /// Create a scheme from an existing palette and a name.
    pub fn from_palette(name: impl Into<String>, p: &Palette) -> Self {
        Self {
            name: name.into(),
            dark: p.dark,
            window: hex(p.window),
            panel: hex(p.panel),
            surface: hex(p.surface),
            surface_hover: hex(p.surface_hover),
            surface_active: hex(p.surface_active),
            outline: hex(p.outline),
            text: hex(p.text),
            secondary: hex(p.secondary),
            dim: hex(p.dim),
            accent: hex(p.accent),
            accent_hover: hex(p.accent_hover),
            on_accent: hex(p.on_accent),
            danger: hex(p.danger),
            warning: hex(p.warning),
            overlay: hex(p.overlay),
            shadow: hex_shadow(p.shadow),
            surface_container_lowest: hex(p.surface_container_lowest),
            surface_container_low: hex(p.surface_container_low),
            surface_container: hex(p.surface_container),
            surface_container_high: hex(p.surface_container_high),
            surface_container_highest: hex(p.surface_container_highest),
            primary_container: hex(p.primary_container),
            on_primary_container: hex(p.on_primary_container),
            outline_variant: hex(p.outline_variant),
        }
    }
}

/// Parse Caelestia scheme JSON (`~/.local/state/caelestia/scheme.json`)
pub fn parse_caelestia_scheme(json: &str) -> Option<Palette> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let mode = v.get("mode").and_then(|m| m.as_str()).unwrap_or("dark");
    let dark = mode != "light";
    let fallback = if dark { Palette::default_dark() } else { Palette::light() };
    let colours = v.get("colours")?.as_object()?;

    let get_c = |key: &str| -> Option<Color32> {
        colours.get(key).and_then(|val| val.as_str()).map(parse_color)
    };

    let bg = get_c("background").or_else(|| get_c("surface")).unwrap_or(fallback.window);
    let s_lowest = get_c("surfaceContainerLowest").unwrap_or(fallback.surface_container_lowest);
    let s_low = get_c("surfaceContainerLow").unwrap_or(fallback.surface_container_low);
    let s_base = get_c("surfaceContainer").unwrap_or(fallback.surface_container);
    let s_high = get_c("surfaceContainerHigh").unwrap_or(fallback.surface_container_high);
    let s_highest = get_c("surfaceContainerHighest").unwrap_or(fallback.surface_container_highest);
    let on_surf = get_c("onSurface").unwrap_or(fallback.text);
    let on_surf_var = get_c("onSurfaceVariant").unwrap_or(fallback.secondary);
    let outline = get_c("outline").unwrap_or(fallback.dim);
    let outline_var = get_c("outlineVariant").unwrap_or(fallback.outline_variant);
    let primary = get_c("primary").unwrap_or(fallback.accent);
    let prim_fixed = get_c("primaryFixed").or_else(|| get_c("primaryFixedDim")).unwrap_or(fallback.accent_hover);
    let on_primary = get_c("onPrimary").unwrap_or(fallback.on_accent);
    let prim_cont = get_c("primaryContainer").unwrap_or(fallback.primary_container);
    let on_prim_cont = get_c("onPrimaryContainer").unwrap_or(fallback.on_primary_container);
    let error = get_c("error").unwrap_or(fallback.danger);
    let warning = get_c("peach").or_else(|| get_c("yellow")).unwrap_or(fallback.warning);
    let shadow = get_c("shadow").map(|c| Color32::from_rgba_premultiplied(c.r(), c.g(), c.b(), if dark { 140 } else { 40 })).unwrap_or(fallback.shadow);

    Some(Palette {
        dark,
        window: bg,
        panel: s_low,
        surface: s_base,
        surface_hover: s_high,
        surface_active: s_highest,
        outline,
        text: on_surf,
        secondary: on_surf_var,
        dim: outline,
        accent: primary,
        accent_hover: prim_fixed,
        on_accent: on_primary,
        danger: error,
        warning,
        overlay: s_base,
        shadow,
        surface_container_lowest: s_lowest,
        surface_container_low: s_low,
        surface_container: s_base,
        surface_container_high: s_high,
        surface_container_highest: s_highest,
        primary_container: prim_cont,
        on_primary_container: on_prim_cont,
        outline_variant: outline_var,
    })
}

/// Returns the path to the active Caelestia scheme file if it exists.
pub fn caelestia_scheme_path() -> Option<std::path::PathBuf> {
    if let Ok(state) = std::env::var("XDG_STATE_HOME") {
        let p = std::path::PathBuf::from(state).join("caelestia/scheme.json");
        if p.exists() {
            return Some(p);
        }
    }
    let home = std::env::var("HOME").ok()?;
    let p = std::path::PathBuf::from(home).join(".local/state/caelestia/scheme.json");
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

/// Loads the palette from Caelestia's active scheme file.
pub fn load_caelestia_palette() -> Option<Palette> {
    let path = caelestia_scheme_path()?;
    let content = std::fs::read_to_string(&path).ok()?;
    parse_caelestia_scheme(&content)
}

/// Returns the last modified timestamp of the Caelestia scheme file.
pub fn caelestia_scheme_mtime() -> Option<std::time::SystemTime> {
    let path = caelestia_scheme_path()?;
    std::fs::metadata(&path).and_then(|m| m.modified()).ok()
}

fn parse_color_or(s: &str, fallback: Color32) -> Color32 {
    if s.is_empty() {
        fallback
    } else {
        parse_color(s)
    }
}

fn parse_color_alpha_or(s: &str, fallback: Color32) -> Color32 {
    if s.is_empty() {
        fallback
    } else {
        parse_color_alpha(s)
    }
}

fn hex(c: Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}

fn hex_shadow(c: Color32) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", c.r(), c.g(), c.b(), c.a())
}

fn parse_color(s: &str) -> Color32 {
    let s = s.trim_start_matches('#');
    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            Color32::from_rgb(r, g, b)
        }
        _ => parse_color_alpha(s),
    }
}

fn parse_color_alpha(s: &str) -> Color32 {
    let s = s.trim_start_matches('#');
    match s.len() {
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
            Color32::from_rgba_premultiplied(r, g, b, a)
        }
        6 => parse_color(s),
        _ => Color32::BLACK,
    }
}

pub const RADIUS: u8 = 12;
pub const RADIUS_SMALL: u8 = 6;
pub const RADIUS_LARGE: u8 = 16;
pub const RADIUS_PILL: u8 = 99;
pub const ROW_HEIGHT: f32 = 56.0;
pub const COMPACT_ROW_HEIGHT: f32 = 48.0;
/// The compact track list: one line, no cover.
pub const THIN_ROW_HEIGHT: f32 = 36.0;
pub const PLAYER_BAR_HEIGHT: f32 = 88.0;
/// The narrowest either right-hand panel goes. The queue and the lyrics
/// take the same edge and swap places there, so a width that suits one
/// has to suit the other, or the window would jump on the swap.
pub const SIDE_PANEL_MIN_WIDTH: f32 = 280.0;
pub const TOP_BAR_HEIGHT: f32 = 56.0;

/// macOS hides the titlebar and draws the window content all the way to the
/// top edge, so whatever sits at the top of the window has to leave room for
/// the traffic lights. Zero everywhere else, and in fullscreen, where the
/// buttons are gone.
pub fn titlebar_inset(ctx: &egui::Context) -> f32 {
    if cfg!(target_os = "macos") && !ctx.input(|input| input.viewport().fullscreen.unwrap_or(false))
    {
        28.0
    } else {
        0.0
    }
}

const INTER_MEDIUM: &str = "inter-medium";
const INTER_SEMIBOLD: &str = "inter-semibold";
const INTER_BOLD: &str = "inter-bold";
pub const MATERIAL_SYMBOLS: &str = "material_symbols";

pub fn material_font(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name(MATERIAL_SYMBOLS.into()))
}

pub fn regular(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Proportional)
}

pub fn medium(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name(INTER_MEDIUM.into()))
}

pub fn semibold(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name(INTER_SEMIBOLD.into()))
}

pub fn bold(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name(INTER_BOLD.into()))
}

/// Install fonts, icons, and the base style once.
pub fn install(ctx: &egui::Context) {
    install_fonts(ctx);
    register_icons(ctx);
    egui_extras::install_image_loaders(ctx);
}

/// Applies the palette to egui's own widgets so dialogs, menus, and text
/// fields agree with the custom views.
pub fn apply(ctx: &egui::Context, palette: &Palette) {
    let mut style = (*ctx.global_style()).clone();
    let visuals = &mut style.visuals;
    *visuals = if palette.dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    visuals.dark_mode = palette.dark;
    visuals.panel_fill = palette.panel;
    visuals.window_fill = palette.surface;
    visuals.extreme_bg_color = palette.surface_container_lowest;
    visuals.faint_bg_color = palette.surface_container;
    visuals.code_bg_color = palette.surface_container;
    visuals.override_text_color = Some(palette.text);
    visuals.weak_text_color = Some(palette.secondary);
    visuals.hyperlink_color = palette.accent;
    visuals.selection.bg_fill = palette.primary_container;
    visuals.selection.stroke = Stroke::NONE;
    visuals.window_stroke = Stroke::NONE;
    visuals.window_corner_radius = CornerRadius::same(RADIUS_LARGE);
    visuals.menu_corner_radius = CornerRadius::same(RADIUS);
    visuals.window_shadow = egui::epaint::Shadow {
        offset: [0, 8],
        blur: 32,
        spread: 0,
        color: palette.shadow,
    };
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: [0, 4],
        blur: 20,
        spread: 0,
        color: palette.shadow,
    };
    let corner = CornerRadius::same(RADIUS_SMALL + 2);
    for widget in [
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        widget.corner_radius = corner;
        widget.bg_stroke = Stroke::NONE;
        widget.fg_stroke = Stroke::new(1.0, palette.text);
        widget.expansion = 0.0;
    }
    visuals.widgets.noninteractive.corner_radius = corner;
    visuals.widgets.noninteractive.bg_fill = palette.panel;
    visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, palette.text);
    visuals.widgets.inactive.bg_fill = palette.surface_container;
    visuals.widgets.inactive.weak_bg_fill = palette.surface_container;
    visuals.widgets.hovered.bg_fill = palette.surface_container_high;
    visuals.widgets.hovered.weak_bg_fill = palette.surface_container_high;
    visuals.widgets.active.bg_fill = palette.surface_container_highest;
    visuals.widgets.active.weak_bg_fill = palette.surface_container_highest;
    visuals.widgets.open.bg_fill = palette.surface_container_high;
    visuals.widgets.open.weak_bg_fill = palette.surface_container_high;
    visuals.text_cursor.stroke = Stroke::new(2.0, palette.accent);
    visuals.striped = false;
    visuals.slider_trailing_fill = true;
    visuals.handle_shape = egui::style::HandleShape::Circle;

    use egui::FontFamily::{Monospace, Proportional};
    use egui::{FontId, TextStyle};
    style.text_styles = [
        (TextStyle::Small, FontId::new(11.5, Proportional)),
        (TextStyle::Body, FontId::new(14.0, Proportional)),
        (TextStyle::Button, FontId::new(14.0, Proportional)),
        (TextStyle::Heading, FontId::new(22.0, Proportional)),
        (TextStyle::Monospace, FontId::new(13.0, Monospace)),
    ]
    .into();
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    style.spacing.interact_size = Vec2::new(40.0, 28.0);
    style.spacing.menu_margin = egui::Margin::same(6);
    style.spacing.window_margin = egui::Margin::same(16);
    style.spacing.scroll = egui::style::ScrollStyle {
        bar_width: 8.0,
        floating_width: 6.0,
        floating_allocated_width: 0.0,
        handle_min_length: 28.0,
        bar_inner_margin: 3.0,
        bar_outer_margin: 2.0,
        dormant_background_opacity: 0.0,
        dormant_handle_opacity: 0.0,
        active_background_opacity: 0.0,
        active_handle_opacity: 0.55,
        interact_handle_opacity: 0.85,
        foreground_color: true,
        ..egui::style::ScrollStyle::floating()
    };
    style.interaction.selectable_labels = false;
    style.interaction.tooltip_delay = 0.4;
    style.animation_time = 0.12;
    style.url_in_tooltip = false;
    ctx.set_global_style(style);
}

fn install_fonts(ctx: &egui::Context) {
    use egui::epaint::text::VariationCoords;
    use egui::{FontData, FontDefinitions, FontFamily};
    use std::sync::Arc;

    let mut fonts = FontDefinitions::default();
    let gsans = include_bytes!("../assets/fonts/GoogleSansFlex-VariableFont_GRAD,ROND,opsz,slnt,wdth,wght.ttf");
    let inter = include_bytes!("../assets/fonts/InterVariable.ttf");
    let weighted_gsans = |weight: f32| {
        let mut data = FontData::from_static(gsans);
        data.tweak.coords = VariationCoords::new([(b"wght", weight)]);
        Arc::new(data)
    };
    let weighted_inter = |weight: f32| {
        let mut data = FontData::from_static(inter);
        data.tweak.coords = VariationCoords::new([(b"wght", weight)]);
        Arc::new(data)
    };

    fonts.font_data.insert("google_sans".to_owned(), weighted_gsans(400.0));
    fonts.font_data.insert("inter".to_owned(), weighted_inter(400.0));
    fonts
        .font_data
        .insert(INTER_MEDIUM.to_owned(), weighted_gsans(500.0));
    fonts
        .font_data
        .insert(INTER_SEMIBOLD.to_owned(), weighted_gsans(600.0));
    fonts
        .font_data
        .insert(INTER_BOLD.to_owned(), weighted_gsans(700.0));

    let mat_symbols = include_bytes!("../assets/fonts/MaterialSymbolsRounded.ttf");
    let mut mat_data = FontData::from_static(mat_symbols);
    mat_data.tweak.coords = VariationCoords::new([(b"wght", 400.0), (b"FILL", 1.0)]);
    fonts.font_data.insert(MATERIAL_SYMBOLS.to_owned(), Arc::new(mat_data));
    fonts.families.insert(
        FontFamily::Name(MATERIAL_SYMBOLS.into()),
        vec![MATERIAL_SYMBOLS.to_owned()],
    );

    let noto_emoji = include_bytes!("../assets/fonts/NotoEmoji.ttf");
    fonts.font_data.insert(
        "noto_emoji".to_owned(),
        Arc::new(FontData::from_static(noto_emoji)),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "google_sans".to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(1, "inter".to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(2, MATERIAL_SYMBOLS.to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(3, "noto_emoji".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(1, "noto_emoji".to_owned());
    let fallbacks: Vec<String> = fonts.families[&FontFamily::Proportional]
        .iter()
        .skip(1)
        .cloned()
        .collect();
    for name in [INTER_MEDIUM, INTER_SEMIBOLD, INTER_BOLD] {
        let mut family = vec![name.to_owned()];
        family.extend(fallbacks.iter().cloned());
        fonts.families.insert(FontFamily::Name(name.into()), family);
    }

    // Add installed fallbacks for scripts Inter does not cover. Keep them after
    // Inter and the emoji font to preserve Latin shapes and color emoji.
    for font in crate::system_fonts::fallbacks() {
        // Reuse cached font bytes to avoid copying large collections whenever
        // epaint rebuilds the glyph atlas.
        let mut data = FontData::from_static(&font.bytes);
        data.index = font.index;
        fonts.font_data.insert(font.name.clone(), Arc::new(data));
        for family in fonts.families.values_mut() {
            family.push(font.name.clone());
        }
    }

    ctx.set_fonts(fonts);
}

macro_rules! icons {
    ($($variant:ident => $file:literal),* $(,)?) => {
        &[$((
            Icon::$variant,
            concat!("bytes://fastpotify-icon-", $file, ".svg"),
            include_bytes!(concat!("../assets/icons/", $file, ".svg")).as_slice(),
        )),*]
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Icon {
    ArrowLeft,
    ArrowRight,
    AudioLines,
    BadgeCheck,
    Bookmark,
    BookmarkFilled,
    Car,
    Cast,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    CircleAlert,
    CircleCheck,
    CirclePlay,
    CirclePlus,
    CircleX,
    Clock,
    Compass,
    Copy,
    Disc,
    Ellipsis,
    ExternalLink,
    Gamepad,
    Globe,
    GripVertical,
    Headphones,
    Heart,
    HeartFilled,
    House,
    Info,
    Laptop,
    Library,
    ListEnd,
    ListMusic,
    ListPlus,
    ListVideo,
    Loader,
    Lock,
    LogOut,
    Mic,
    Minus,
    Monitor,
    Moon,
    Music,
    Pause,
    PauseFilled,
    PanelLeft,
    Pin,
    PinOff,
    Pencil,
    Play,
    PlayFilled,
    Plus,
    Radio,
    Refresh,
    Repeat,
    Repeat1,
    Search,
    Settings,
    Shrink,
    Shuffle,
    SkipBack,
    SkipBackFilled,
    SkipForward,
    SkipForwardFilled,
    Smartphone,
    Sparkles,
    Speaker,
    Square,
    SquarePen,
    Sun,
    Tablet,
    Trash,
    TrendingUp,
    Tv,
    User,
    Users,
    Volume,
    Volume1,
    Volume2,
    VolumeX,
    Watch,
    X,
    Zap,
}

const ICONS: &[(Icon, &str, &[u8])] = icons! {
    ArrowLeft => "arrow-left",
    ArrowRight => "arrow-right",
    AudioLines => "audio-lines",
    BadgeCheck => "badge-check",
    Bookmark => "bookmark",
    BookmarkFilled => "bookmark-filled",
    Car => "car",
    Cast => "cast",
    Check => "check",
    ChevronDown => "chevron-down",
    ChevronLeft => "chevron-left",
    ChevronRight => "chevron-right",
    ChevronUp => "chevron-up",
    CircleAlert => "circle-alert",
    CircleCheck => "circle-check",
    CirclePlay => "circle-play",
    CirclePlus => "circle-plus",
    CircleX => "circle-x",
    Clock => "clock",
    Compass => "compass",
    Copy => "copy",
    Disc => "disc-3",
    Ellipsis => "ellipsis",
    ExternalLink => "external-link",
    Gamepad => "gamepad-2",
    Globe => "globe",
    GripVertical => "grip-vertical",
    Headphones => "headphones",
    Heart => "heart",
    HeartFilled => "heart-filled",
    House => "house",
    Info => "info",
    Laptop => "laptop",
    Library => "library",
    ListEnd => "list-end",
    ListMusic => "list-music",
    ListPlus => "list-plus",
    ListVideo => "list-video",
    Loader => "loader-circle",
    Lock => "lock",
    LogOut => "log-out",
    Mic => "mic",
    Minus => "minus",
    Monitor => "monitor",
    Moon => "moon",
    Music => "music",
    Pause => "pause",
    PauseFilled => "pause-filled",
    PanelLeft => "panel-left",
    Pin => "pin",
    PinOff => "pin-off",
    Pencil => "pencil",
    Play => "play",
    PlayFilled => "play-filled",
    Plus => "plus",
    Radio => "radio",
    Refresh => "refresh-cw",
    Repeat => "repeat",
    Repeat1 => "repeat-1",
    Search => "search",
    Settings => "settings",
    Shrink => "shrink",
    Shuffle => "shuffle",
    SkipBack => "skip-back",
    SkipBackFilled => "skip-back-filled",
    SkipForward => "skip-forward",
    SkipForwardFilled => "skip-forward-filled",
    Smartphone => "smartphone",
    Sparkles => "sparkles",
    Speaker => "speaker",
    Square => "square",
    SquarePen => "square-pen",
    Sun => "sun",
    Tablet => "tablet",
    Trash => "trash-2",
    TrendingUp => "trending-up",
    Tv => "tv",
    User => "user",
    Users => "users",
    Volume => "volume",
    Volume1 => "volume-1",
    Volume2 => "volume-2",
    VolumeX => "volume-x",
    Watch => "watch",
    X => "x",
    Zap => "zap",
};

impl Icon {
    pub fn uri(self) -> &'static str {
        ICONS
            .iter()
            .find(|(icon, _, _)| *icon == self)
            .map_or("", |(_, uri, _)| *uri)
    }

    pub fn image(self, color: Color32, size: f32) -> egui::Image<'static> {
        egui::Image::new(self.uri())
            .tint(color)
            .fit_to_exact_size(Vec2::splat(size))
    }

    pub fn symbol_glyph(self) -> &'static str {
        match self {
            Icon::ArrowLeft => "\u{e5c4}",
            Icon::ArrowRight => "\u{e5c8}",
            Icon::AudioLines => "\u{e01d}",
            Icon::BadgeCheck => "\u{e86c}",
            Icon::Bookmark => "\u{e866}",
            Icon::BookmarkFilled => "\u{e866}",
            Icon::Car => "\u{e531}",
            Icon::Cast => "\u{e307}",
            Icon::Check => "\u{e5ca}",
            Icon::ChevronDown => "\u{e5cf}",
            Icon::ChevronLeft => "\u{e5cb}",
            Icon::ChevronRight => "\u{e5cc}",
            Icon::ChevronUp => "\u{e5ce}",
            Icon::CircleAlert => "\u{e000}",
            Icon::CircleCheck => "\u{e86c}",
            Icon::CirclePlay => "\u{e038}",
            Icon::CirclePlus => "\u{e147}",
            Icon::CircleX => "\u{e5c9}",
            Icon::Clock => "\u{e8b5}",
            Icon::Compass => "\u{e87a}",
            Icon::Copy => "\u{e14d}",
            Icon::Disc => "\u{e019}",
            Icon::Ellipsis => "\u{e5d4}",
            Icon::ExternalLink => "\u{e895}",
            Icon::Gamepad => "\u{e30f}",
            Icon::Globe => "\u{e80b}",
            Icon::GripVertical => "\u{e5d4}",
            Icon::Headphones => "\u{e310}",
            Icon::Heart => "\u{e87e}",
            Icon::HeartFilled => "\u{e87d}",
            Icon::House => "\u{e88a}",
            Icon::Info => "\u{e88e}",
            Icon::Laptop => "\u{e30c}",
            Icon::Library => "\u{e021}",
            Icon::ListEnd => "\u{e896}",
            Icon::ListMusic => "\u{e896}",
            Icon::ListPlus => "\u{e03b}",
            Icon::ListVideo => "\u{e059}",
            Icon::Loader => "\u{e5d5}",
            Icon::Lock => "\u{e897}",
            Icon::LogOut => "\u{e9ba}",
            Icon::Mic => "\u{e029}",
            Icon::Minus => "\u{e15b}",
            Icon::Monitor => "\u{e30b}",
            Icon::Moon => "\u{e51c}",
            Icon::Music => "\u{e405}",
            Icon::Pause | Icon::PauseFilled => "\u{e034}",
            Icon::PanelLeft => "\u{e5d2}",
            Icon::Pin => "\u{e566}",
            Icon::PinOff => "\u{e566}",
            Icon::Pencil => "\u{e3c9}",
            Icon::Play | Icon::PlayFilled => "\u{e037}",
            Icon::Plus => "\u{e145}",
            Icon::Radio => "\u{e51e}",
            Icon::Refresh => "\u{e5d5}",
            Icon::Repeat => "\u{e040}",
            Icon::Repeat1 => "\u{e041}",
            Icon::Search => "\u{e8b6}",
            Icon::Settings => "\u{e8b8}",
            Icon::Shrink => "\u{e5d0}",
            Icon::Shuffle => "\u{e043}",
            Icon::SkipBack | Icon::SkipBackFilled => "\u{e045}",
            Icon::SkipForward | Icon::SkipForwardFilled => "\u{e044}",
            Icon::Smartphone => "\u{e32c}",
            Icon::Sparkles => "\u{e65f}",
            Icon::Speaker => "\u{e32d}",
            Icon::Square => "\u{e036}",
            Icon::SquarePen => "\u{e3c9}",
            Icon::Sun => "\u{e518}",
            Icon::Tablet => "\u{e330}",
            Icon::Trash => "\u{e872}",
            Icon::TrendingUp => "\u{e8e5}",
            Icon::Tv => "\u{e333}",
            Icon::User => "\u{e7fd}",
            Icon::Users => "\u{e7fb}",
            Icon::Volume => "\u{e050}",
            Icon::Volume1 => "\u{e04d}",
            Icon::Volume2 => "\u{e050}",
            Icon::VolumeX => "\u{e04f}",
            Icon::Watch => "\u{e334}",
            Icon::X => "\u{e5cd}",
            Icon::Zap => "\u{e8e8}",
        }
    }
}

fn register_icons(ctx: &egui::Context) {
    for (_, uri, bytes) in ICONS {
        ctx.include_bytes(*uri, *bytes);
    }
}

/// A static icon using Material Symbols.
pub fn icon(ui: &mut egui::Ui, icon: Icon, size: f32, color: Color32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    if ui.is_rect_visible(rect) {
        paint_icon(ui, icon, rect, size, color);
    }
    response
}

/// Paints an icon centred in `rect` without allocating space using Material Symbols Rounded.
pub fn paint_icon(ui: &egui::Ui, icon: Icon, rect: egui::Rect, size: f32, color: Color32) {
    let font_id = egui::FontId::new(size, egui::FontFamily::Name(MATERIAL_SYMBOLS.into()));
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon.symbol_glyph(),
        font_id,
        color,
    );
}

pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_premultiplied(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        (a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
    )
}

/// A frameless icon control whose colour lifts on hover with Caelestia-style smooth animation.
pub fn icon_button(
    ui: &mut egui::Ui,
    icon: Icon,
    size: f32,
    color: Color32,
    hover: Color32,
    tooltip: &str,
) -> Response {
    let edge = size + 12.0;
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(edge), Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered() || response.has_focus();
        let hover_t = ui.ctx().animate_bool(response.id.with("ib_hover"), hovered);
        let pressed = response.is_pointer_button_down_on();
        let press_t = ui.ctx().animate_bool(response.id.with("ib_press"), pressed);

        let scale = 1.0 + hover_t * 0.08 - press_t * 0.12;
        let tint = lerp_color(color, hover, hover_t);

        // Caelestia state layer hover circular glow
        if hover_t > 0.01 {
            let bg = Color32::from_white_alpha((hover_t * 18.0) as u8);
            ui.painter()
                .circle_filled(rect.center(), (edge / 2.0) * (0.85 + hover_t * 0.15), bg);
        }

        let draw_rect = egui::Rect::from_center_size(rect.center(), rect.size() * scale);
        paint_icon(ui, icon, draw_rect, size * scale, tint);
    }
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    if tooltip.is_empty() {
        response
    } else {
        response.on_hover_text(tooltip)
    }
}

/// Horizontal offset that optically centers play triangles.
pub fn play_glyph_offset(icon: Icon, icon_size: f32) -> Vec2 {
    if matches!(icon, Icon::PlayFilled | Icon::Play) {
        Vec2::new(icon_size * 0.05, 0.0)
    } else {
        Vec2::ZERO
    }
}

pub fn logo(ui: &egui::Ui, center: egui::Pos2, diameter: f32, disc: Color32, glyph: Color32) {
    ui.painter().circle_filled(center, diameter / 2.0, disc);
    let icon_size = diameter * 0.52;
    let rect = egui::Rect::from_center_size(center, Vec2::splat(diameter));
    paint_icon(ui, Icon::PlayFilled, rect, icon_size, glyph);
}

pub fn brand_mark(ui: &egui::Ui, rect: egui::Rect, glyph: Color32) {
    ui.painter().circle_filled(
        rect.center(),
        rect.width() / 2.0,
        ui.visuals().selection.stroke.color,
    );
    let icon_size = rect.width() * 0.55;
    paint_icon(ui, Icon::PlayFilled, rect, icon_size, glyph);
}

pub fn circle_button(
    ui: &mut egui::Ui,
    icon: Icon,
    diameter: f32,
    fill: Color32,
    fill_hover: Color32,
    icon_color: Color32,
    tooltip: &str,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(diameter), Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let grow = if hovered { 1.05 } else { 1.0 };
        let radius = diameter / 2.0 * grow;
        let fill = if hovered { fill_hover } else { fill };
        ui.painter().circle_filled(rect.center(), radius, fill);
        let icon_size = diameter * 0.52;
        paint_icon(ui, icon, rect, icon_size, icon_color);
    }
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    if tooltip.is_empty() {
        response
    } else {
        response.on_hover_text(tooltip)
    }
}

/// A disc the size of a [`circle_button`] whose icon is replaced by a
/// spinner: the pressed play button itself shows that Spotify is reacting.
pub fn circle_spinner(
    ui: &mut egui::Ui,
    diameter: f32,
    fill: Color32,
    spin: Color32,
    tooltip: &str,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(diameter), Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter()
            .circle_filled(rect.center(), diameter / 2.0, fill);
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
        ));
        spinner(&mut child, diameter * 0.55, spin);
    }
    if tooltip.is_empty() {
        response
    } else {
        response.on_hover_text(tooltip)
    }
}

/// A pill-shaped text button: filled for the primary action, outlined otherwise.
pub fn pill_button(ui: &mut egui::Ui, palette: &Palette, label: &str, primary: bool) -> Response {
    let font = semibold(13.0);
    let color = if primary {
        palette.on_accent
    } else {
        palette.text
    };
    let galley = ui.painter().layout_no_wrap(label.to_string(), font, color);
    let padding = Vec2::new(18.0, 8.0);
    let size = galley.size() + padding * 2.0;
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let hover_t = ui.ctx().animate_bool(response.id.with("pill_hover"), hovered);
        let pressed = response.is_pointer_button_down_on();
        let press_t = ui.ctx().animate_bool(response.id.with("pill_press"), pressed);

        let scale = 1.0 + hover_t * 0.03 - press_t * 0.05;
        let draw_rect = egui::Rect::from_center_size(rect.center(), rect.size() * scale);
        let radius = draw_rect.height() / 2.0;

        if primary {
            let fill = lerp_color(palette.accent, palette.accent_hover, hover_t);
            ui.painter().rect_filled(draw_rect, radius, fill);
        } else {
            let fill = lerp_color(palette.surface_container_highest, palette.surface_container_high, hover_t);
            ui.painter().rect_filled(
                draw_rect,
                CornerRadius::same(radius as u8),
                fill,
            );
        }
        let pos = draw_rect.center() - galley.size() / 2.0;
        ui.painter().galley(pos, galley, color);
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A muted button with an icon and label, for row and header actions.
pub fn soft_button(
    ui: &mut egui::Ui,
    palette: &Palette,
    icon: Option<Icon>,
    label: &str,
    active: bool,
) -> Response {
    let font = medium(13.0);
    let color = if active {
        palette.on_primary_container
    } else if ui.is_enabled() {
        palette.text
    } else {
        palette.secondary
    };
    let galley =
        ui.painter()
            .layout_no_wrap(crate::bidi::display_text(label).into_owned(), font, color);
    let icon_size = 15.0;
    let icon_width = if icon.is_some() { icon_size + 6.0 } else { 0.0 };
    let padding = Vec2::new(10.0, 6.0);
    let size = Vec2::new(galley.size().x + icon_width, galley.size().y) + padding * 2.0;
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let hover_t = ui.ctx().animate_bool(response.id.with("soft_hover"), hovered);
        let pressed = response.is_pointer_button_down_on();
        let press_t = ui.ctx().animate_bool(response.id.with("soft_press"), pressed);

        let scale = 1.0 + hover_t * 0.03 - press_t * 0.05;
        let draw_rect = egui::Rect::from_center_size(rect.center(), rect.size() * scale);

        let base_fill = if active {
            palette.primary_container
        } else {
            palette.surface_container
        };
        let hover_fill = if active {
            palette.accent_hover
        } else {
            palette.surface_container_high
        };
        let fill = lerp_color(base_fill, hover_fill, hover_t);

        ui.painter().rect_filled(
            draw_rect,
            CornerRadius::same((draw_rect.height() / 2.0) as u8),
            fill,
        );
        let mut x = draw_rect.left() + padding.x;
        if let Some(icon) = icon {
            let icon_rect = egui::Rect::from_center_size(
                egui::pos2(x + icon_size / 2.0, draw_rect.center().y),
                Vec2::splat(icon_size),
            );
            icon.image(color, icon_size).paint_at(ui, icon_rect);
            x += icon_width;
        }
        let pos = egui::pos2(x, draw_rect.center().y - galley.size().y / 2.0);
        ui.painter().galley(pos, galley, color);
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// An Astra-styled navigation row with a circular icon container, title, and optional badge/trailing element.
pub fn astra_nav_row(
    ui: &mut egui::Ui,
    palette: &Palette,
    icon: Icon,
    title: &str,
    subtitle: Option<&str>,
    active: bool,
) -> Response {
    let width = ui.available_width();
    let height = if subtitle.is_some() { 46.0 } else { 40.0 };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();

        let hover_t = ui.ctx().animate_bool(response.id.with("nav_hover"), hovered);
        let press_t = ui.ctx().animate_bool(response.id.with("nav_press"), pressed);

        let base_fill = if active {
            palette.primary_container
        } else {
            Color32::TRANSPARENT
        };
        let target_hover = if active {
            palette.primary_container
        } else {
            palette.surface_container_high
        };
        let mut row_fill = lerp_color(base_fill, target_hover, hover_t);
        if pressed {
            row_fill = lerp_color(row_fill, palette.surface_container_highest, press_t);
        }

        ui.painter()
            .rect_filled(rect, CornerRadius::same(RADIUS_SMALL as u8 + 4), row_fill);

        // Circular icon badge
        let badge_diameter = 28.0;
        let badge_center = egui::pos2(rect.left() + 6.0 + badge_diameter / 2.0, rect.center().y);
        let base_badge = if active {
            palette.accent
        } else {
            palette.surface_container_high
        };
        let hover_badge = if active {
            palette.accent
        } else {
            palette.surface_container_highest
        };
        let badge_fill = lerp_color(base_badge, hover_badge, hover_t);

        let base_icon = if active {
            palette.on_accent
        } else {
            palette.secondary
        };
        let target_icon = if active {
            palette.on_accent
        } else {
            palette.text
        };
        let icon_color = lerp_color(base_icon, target_icon, hover_t);

        ui.painter()
            .circle_filled(badge_center, badge_diameter / 2.0, badge_fill);
        let icon_size = 15.0;
        let icon_rect = egui::Rect::from_center_size(
            badge_center + play_glyph_offset(icon, icon_size),
            Vec2::splat(icon_size),
        );
        icon.image(icon_color, icon_size).paint_at(ui, icon_rect);

        // Text
        let text_left = rect.left() + 44.0;
        let text_color = if active {
            palette.text
        } else {
            lerp_color(palette.secondary, palette.text, hover_t)
        };

        if let Some(sub) = subtitle {
            let title_galley = ui.painter().layout_no_wrap(
                crate::bidi::display_text(title).into_owned(),
                medium(13.0),
                text_color,
            );
            let sub_galley = ui.painter().layout_no_wrap(
                crate::bidi::display_text(sub).into_owned(),
                regular(10.5),
                palette.dim,
            );
            let title_pos = egui::pos2(text_left, rect.center().y - title_galley.size().y + 1.0);
            let sub_pos = egui::pos2(text_left, rect.center().y + 2.0);
            ui.painter().galley(title_pos, title_galley, text_color);
            ui.painter().galley(sub_pos, sub_galley, palette.dim);
        } else {
            let title_galley = ui.painter().layout_no_wrap(
                crate::bidi::display_text(title).into_owned(),
                medium(13.0),
                text_color,
            );
            let title_pos = egui::pos2(text_left, rect.center().y - title_galley.size().y / 2.0);
            ui.painter().galley(title_pos, title_galley, text_color);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// An animated busy indicator paced independently of the graphics driver.
pub fn spinner(ui: &mut egui::Ui, size: f32, color: Color32) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(33));
        let radius = size / 2.0 - 2.0;
        let start = ui.input(|input| input.time) * std::f64::consts::TAU * 1.2;
        let sweep = 250_f64.to_radians();
        let points = (0..20)
            .map(|index| {
                let angle = start + sweep * f64::from(index) / 19.0;
                let (sin, cos) = angle.sin_cos();
                rect.center() + radius * egui::vec2(cos as f32, sin as f32)
            })
            .collect();
        ui.painter()
            .add(egui::Shape::line(points, Stroke::new(2.0, color)));
    }
    response
}

/// Truncated single-line text in a given font and colour.
pub fn text(
    ui: &mut egui::Ui,
    text: impl Into<String>,
    font: egui::FontId,
    color: Color32,
) -> Response {
    let text = text.into();
    if crate::bidi::is_rtl(&text) {
        // Laid out here so a cut lands at the reading end, on the left.
        let galley = crate::bidi::layout(
            ui.painter(),
            &text,
            font,
            color,
            ui.available_width(),
            1,
            Some(crate::bidi::ELLIPSIS),
        );
        return ui.add(egui::Label::new(galley).selectable(false));
    }
    ui.add(
        egui::Label::new(egui::RichText::new(text).font(font).color(color))
            .truncate()
            .selectable(false),
    )
}

/// Single-line text that acts like a link: underlines on hover, clickable.
pub fn link(
    ui: &mut egui::Ui,
    text: impl Into<String>,
    font: egui::FontId,
    color: Color32,
) -> Response {
    let text = text.into();
    let response = if crate::bidi::is_rtl(&text) {
        let galley = crate::bidi::layout(
            ui.painter(),
            &text,
            font,
            color,
            ui.available_width(),
            1,
            Some(crate::bidi::ELLIPSIS),
        );
        ui.add(
            egui::Label::new(galley)
                .selectable(false)
                .sense(Sense::click()),
        )
    } else {
        ui.add(
            egui::Label::new(egui::RichText::new(text).font(font).color(color))
                .truncate()
                .selectable(false)
                .sense(Sense::click()),
        )
    };
    if response.hovered() {
        let rect = response.rect;
        ui.painter()
            .hline(rect.x_range(), rect.bottom() - 1.0, Stroke::new(1.0, color));
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn section_title(ui: &mut egui::Ui, palette: &Palette, label: &str) -> Response {
    text(ui, label, bold(17.0), palette.text)
}

pub fn subtle(ui: &mut egui::Ui, palette: &Palette, label: &str) -> Response {
    text(ui, label, regular(13.0), palette.secondary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fonts_install_and_layout_emojis() {
        let ctx = egui::Context::default();
        install(&ctx);
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            let galley = ui.painter().layout_no_wrap(
                "Rosewood 🔥 Otomo 🎵 ❤️ 🚀".to_string(),
                regular(14.0),
                Color32::WHITE,
            );
            assert!(galley.rows[0].glyphs.len() >= 5);
        });
        output.textures_delta.clear();
    }

    #[test]
    fn color_scheme_default_matches_dark_palette() {
        let scheme = ColorScheme::default();
        let palette = scheme.to_palette();
        assert_eq!(palette, Palette::dark());
    }

    #[test]
    fn color_scheme_from_palette_round_trip() {
        let original = Palette::light();
        let scheme = ColorScheme::from_palette("test", &original);
        assert_eq!(scheme.name, "test");
        assert!(!scheme.dark);
        let restored = scheme.to_palette();
        assert_eq!(restored, original);
    }

    #[test]
    fn color_scheme_partial_json_uses_defaults() {
        let json = r##"{"name":"partial","accent":"#ff0000"}"##;
        let scheme: ColorScheme = serde_json::from_str(json).unwrap();
        assert_eq!(scheme.name, "partial");
        assert_eq!(scheme.accent, "#ff0000");
        // Other fields fall back to defaults
        assert_eq!(scheme.window, hex(Palette::dark().window));
    }

    #[test]
    fn color_scheme_parse_hex_6() {
        let c = parse_color("#1ed760");
        assert_eq!(c, Color32::from_rgb(0x1e, 0xd7, 0x60));
    }

    #[test]
    fn color_scheme_parse_hex_8() {
        let c = parse_color_alpha("#0000008c");
        assert_eq!(c, Color32::from_rgba_premultiplied(0, 0, 0, 0x8c));
    }

    #[test]
    fn color_scheme_serde_round_trip() {
        let scheme = ColorScheme::default();
        let json = serde_json::to_string(&scheme).unwrap();
        let restored: ColorScheme = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.to_palette(), scheme.to_palette());
    }
}
