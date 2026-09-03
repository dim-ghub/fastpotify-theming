//! The now-playing bar along the bottom of the window.

use egui::{Align, CornerRadius, Frame, Layout, Margin, Rect, Sense, UiBuilder, Vec2, pos2, vec2};

use crate::app::{App, NowPlaying};
use crate::model::{Action, Page};
use crate::player::RepeatMode;
use crate::theme::{self, Icon};
use crate::util;

use super::widgets::{SliderEvent, thin_slider, wavy_slider};

pub fn show_in_frame_bottom(app: &mut App, ui: &mut egui::Ui, container_rect: Rect) {
    let palette = app.palette;
    let bg_fill = palette.window;
    ui.painter().rect_filled(
        container_rect,
        CornerRadius::ZERO,
        bg_fill,
    );
    let rect = container_rect.shrink2(vec2(16.0, 0.0));
    let now = app.now_playing();
    let width = rect.width();
    let side = (width * 0.32).clamp(240.0, 440.0);
    let cy = rect.center().y;
    let left = Rect::from_min_max(rect.min, pos2(rect.left() + side, rect.bottom()));
    let center = Rect::from_min_max(
        pos2(rect.left() + side, rect.top()),
        pos2(rect.right() - side, rect.bottom()),
    );

    now_playing_block(app, ui, left, now.as_ref());
    transport(app, ui, now.as_ref(), center);

    let right_band = Rect::from_min_size(pos2(rect.right() - side, cy - 18.0), vec2(side, 36.0));
    let mut right_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(right_band)
            .layout(Layout::right_to_left(Align::Center)),
    );
    extras(app, &mut right_ui, now.as_ref());
}

pub fn show_in_rect(app: &mut App, ui: &mut egui::Ui, container_rect: Rect) {
    show_in_frame_bottom(app, ui, container_rect);
}

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let bg_fill = palette.window;
    egui::Panel::bottom("player-bar")
        .exact_size(theme::PLAYER_BAR_HEIGHT)
        .resizable(false)
        .show_separator_line(false)
        .frame(
            Frame::new()
                .fill(bg_fill)
                .inner_margin(Margin::ZERO),
        )
        .show(ui, |ui| {
            show_in_rect(app, ui, ui.max_rect());
        });
}

fn now_playing_block(app: &mut App, ui: &mut egui::Ui, region: Rect, now: Option<&NowPlaying>) {
    let palette = app.palette;
    let cy = region.center().y;
    let cover_rect = Rect::from_min_size(pos2(region.left() + 4.0, cy - 28.0), Vec2::splat(56.0));

    let Some(now) = now else {
        super::widgets::paint_cover(ui, &palette, None, cover_rect, 12.0, Icon::Music);
        let text_left = cover_rect.right() + 12.0;
        let text_rect = Rect::from_min_size(
            pos2(text_left, cy - 17.0),
            vec2((region.right() - text_left - 8.0).max(40.0), 34.0),
        );
        let mut text_ui = ui.new_child(
            UiBuilder::new()
                .max_rect(text_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        text_ui.spacing_mut().item_spacing.y = 2.0;
        theme::text(
            &mut text_ui,
            "Nothing playing",
            theme::medium(14.0),
            palette.secondary,
        );
        theme::text(
            &mut text_ui,
            "Pick a song, album, or playlist",
            theme::regular(12.0),
            palette.dim,
        );
        return;
    };

    super::widgets::paint_cover(
        ui,
        &palette,
        now.art_small.as_deref().or(now.art_url.as_deref()),
        cover_rect,
        12.0,
        Icon::Music,
    );
    let cover_response = ui
        .interact(
            cover_rect,
            egui::Id::new("now-playing-cover"),
            Sense::click(),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    // Hovering the cover offers to dock the art large at the sidebar's
    // bottom, the way Spotify expands it. (#92)
    let art_available = now.art_url.is_some() || now.art_small.is_some();
    let expand_rect = Rect::from_center_size(
        pos2(cover_rect.right() - 10.0, cover_rect.top() + 10.0),
        Vec2::splat(18.0),
    );
    let offer_expand = art_available && !app.settings.art_expanded && app.settings.sidebar_visible;
    let over_expand = offer_expand && ui.rect_contains_pointer(expand_rect);
    if cover_response.clicked() && !over_expand {
        if let Some(id) = &now.album_id {
            app.actions.push(Action::Open(Page::Album(id.clone())));
        } else if let Some(id) = &now.show_id {
            app.actions.push(Action::Open(Page::Show(id.clone())));
        }
    }
    if offer_expand && (cover_response.hovered() || over_expand) {
        let expand = ui
            .interact(
                expand_rect,
                egui::Id::new("now-playing-art-expand"),
                Sense::click(),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        ui.painter()
            .circle_filled(expand_rect.center(), 9.0, palette.panel.gamma_multiply(0.9));
        Icon::ChevronUp.image(palette.text, 12.0).paint_at(
            ui,
            Rect::from_center_size(expand_rect.center(), Vec2::splat(12.0)),
        );
        if expand.clicked() {
            app.settings.art_expanded = true;
            app.actions.push(Action::SettingsChanged);
        }
    }
    let heart_width = if now.is_episode { 0.0 } else { 42.0 };
    let text_left = cover_rect.right() + 12.0;
    let text_width = (region.right() - text_left - heart_width).max(40.0);
    let text_rect = Rect::from_min_size(pos2(text_left, cy - 18.0), vec2(text_width, 36.0));
    let info_response = ui.interact(text_rect, egui::Id::new("now-playing-info"), Sense::click());
    let mut text_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(text_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    text_ui.set_clip_rect(text_rect.intersect(ui.clip_rect()));
    text_ui.spacing_mut().item_spacing.y = 2.0;
    let title_response = theme::link(&mut text_ui, &now.title, theme::medium(14.0), palette.text);
    if title_response.clicked() {
        if let Some(id) = &now.album_id {
            app.actions.push(Action::Open(Page::Album(id.clone())));
        } else if let Some(id) = &now.show_id {
            app.actions.push(Action::Open(Page::Show(id.clone())));
        }
    }
    text_ui.horizontal_top(|ui| {
        if now.artists.is_empty() {
            if theme::link(ui, &now.subtitle, theme::regular(12.0), palette.secondary).clicked()
                && let Some(id) = &now.show_id
            {
                app.actions.push(Action::Open(Page::Show(id.clone())));
            }
        } else {
            super::widgets::artist_links(
                ui,
                app,
                &now.artists,
                theme::regular(12.0),
                palette.secondary,
            );
        }
    });
    // The playing thing answers the same right-click menu as a table row,
    // from the cover, the empty space around the words, or the words.
    if let Some(item) = app.now_playing_item() {
        for response in [&cover_response, &info_response, &title_response] {
            egui::Popup::context_menu(response)
                .frame(super::widgets::menu_frame(&palette))
                .show(|ui| super::widgets::item_menu(ui, app, &item, None, None));
        }
    }

    if !now.is_episode {
        let saved = app.is_saved(&now.uri).unwrap_or(false);
        let (icon, color, tooltip) = if saved {
            (Icon::HeartFilled, palette.accent, "Remove from Liked Songs")
        } else {
            (Icon::Heart, palette.secondary, "Save to Liked Songs")
        };
        // Sit the heart just past the actual text, not at the region's far
        // edge, so it stays visually attached to the title.
        let natural = {
            let title =
                ui.painter()
                    .layout_no_wrap(now.title.clone(), theme::medium(14.0), palette.text);
            let subtitle = ui.painter().layout_no_wrap(
                now.subtitle.clone(),
                theme::regular(12.0),
                palette.secondary,
            );
            title.size().x.max(subtitle.size().x).min(text_width)
        };
        let heart_x = (text_left + natural + 21.0).min(region.right() - 21.0);
        let heart_rect = Rect::from_center_size(pos2(heart_x, cy), Vec2::splat(30.0));
        let mut heart_ui = ui.new_child(
            UiBuilder::new()
                .max_rect(heart_rect)
                .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
        );
        if theme::icon_button(&mut heart_ui, icon, 17.0, color, palette.text, tooltip).clicked() {
            app.actions.push(Action::ToggleSaved(now.uri.clone()));
        }
    }
}

fn transport(app: &mut App, ui: &mut egui::Ui, now: Option<&NowPlaying>, region: Rect) {
    let palette = app.palette;
    // Everything here is placed with explicit rects: egui's implicit rows
    // centre each widget in the row height known when it is added, which
    // left earlier icons riding high next to the play disc.
    //
    // The buttons row (36) and the progress row (~15, after a 6px gap) form
    // one cluster, centred as a group in the 88px bar: the buttons sit 8px
    // above the bar's midline and the progress row 23px below it. Measured
    // on screen this puts equal breathing room above and beneath the
    // cluster.
    let playing = now.is_some_and(|now| now.playing);
    let loading = now.is_some_and(|now| now.loading);
    let shuffle = now.is_some_and(|now| now.shuffle);
    let repeat = now.map(|now| now.repeat).unwrap_or_default();

    // 1. Progress row (moved upwards to prevent any overlap with control buttons)
    let progress_cy = region.center().y - 23.0;
    let slider_width = (region.width() - 120.0).clamp(160.0, 560.0);
    let (position, duration) = now
        .map(|now| (now.position_ms, now.duration_ms))
        .unwrap_or((0, 0));

    // Smooth handle movement: interpolate smoothly at 60 FPS between Spotify state updates
    let smooth_id = egui::Id::new("player_smooth_position");
    let current_time = ui.input(|i| i.time);
    let (last_pos, last_anchor_time): (u32, f64) = ui
        .data(|d| d.get_temp(smooth_id))
        .unwrap_or((position, current_time));

    let (anchor_pos, anchor_time) = if position != last_pos {
        ui.data_mut(|d| d.insert_temp(smooth_id, (position, current_time)));
        (position, current_time)
    } else {
        (last_pos, last_anchor_time)
    };

    let shown_position = match app.seek_preview {
        Some(fraction) => (fraction * duration as f32) as u32,
        None => {
            if playing && duration > 0 {
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
                let elapsed_ms = ((current_time - anchor_time).max(0.0) * 1000.0) as u32;
                (anchor_pos + elapsed_ms).min(duration)
            } else {
                position
            }
        }
    };
    let time_color = if now.is_some() {
        palette.secondary
    } else {
        palette.dim
    };
    let slider_left = region.center().x - slider_width / 2.0;
    ui.painter().text(
        pos2(slider_left - 10.0, progress_cy),
        egui::Align2::RIGHT_CENTER,
        util::format_duration_ms(shown_position),
        theme::medium(11.5),
        time_color,
    );
    let slider_rect =
        Rect::from_center_size(pos2(region.center().x, progress_cy), vec2(slider_width, 24.0));
    let mut slider_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(slider_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let fraction = if duration > 0 {
        shown_position as f32 / duration as f32
    } else {
        0.0
    };
    let is_playing = app.believed_playing();
    match wavy_slider(
        &mut slider_ui,
        &palette,
        egui::Id::new("seek-slider"),
        fraction,
        slider_width,
        palette.accent,
        is_playing,
    ) {
        SliderEvent::Dragging(value) => app.seek_preview = Some(value),
        SliderEvent::Committed(value) => {
            app.seek_preview = None;
            if duration > 0 {
                app.actions
                    .push(Action::Seek((value * duration as f32) as u32));
            }
        }
        SliderEvent::None => {}
    }
    ui.painter().text(
        pos2(slider_left + slider_width + 10.0, progress_cy),
        egui::Align2::LEFT_CENTER,
        util::format_duration_ms(duration),
        theme::medium(11.5),
        time_color,
    );

    // 2. ButtonRow (BELOW progress bar, pushes neighboring buttons when expanding)
    let buttons_cy = region.center().y + 16.0;
    let base_sizes = [
        vec2(30.0, 40.0), // Shuffle (vertical pill)
        vec2(36.0, 36.0), // Previous (circle)
        vec2(44.0, 44.0), // Play/Pause (circle when paused, rounded square when playing)
        vec2(36.0, 36.0), // Next (circle)
        vec2(30.0, 40.0), // Repeat (vertical pill)
    ];
    let btn_ids = [
        egui::Id::new("ctl_shuffle"),
        egui::Id::new("ctl_prev"),
        egui::Id::new("ctl_play"),
        egui::Id::new("ctl_next"),
        egui::Id::new("ctl_repeat"),
    ];

    // Compute dynamic width for each button so growth pushes the other buttons outwards
    let mut widths = [base_sizes[0].x, base_sizes[1].x, base_sizes[2].x, base_sizes[3].x, base_sizes[4].x];
    for i in 0..5 {
        widths[i] += button_expansion(ui, btn_ids[i]);
    }

    let gap = 8.0;
    let total_w: f32 = widths.iter().sum::<f32>() + gap * 4.0;
    let mut btn_x = region.center().x - total_w / 2.0;

    let mut slots = Vec::with_capacity(5);
    for i in 0..5 {
        let w = widths[i];
        let h = base_sizes[i].y;
        let r = Rect::from_center_size(pos2(btn_x + w / 2.0, buttons_cy), vec2(w, h));
        btn_x += w + gap;
        slots.push(r);
    }

    let centered = |ui: &mut egui::Ui, r: Rect| {
        ui.new_child(
            UiBuilder::new()
                .max_rect(r)
                .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
        )
    };

    // Shuffle: Vertical pill with shape morph
    let mut cell = centered(ui, slots[0]);
    if caelestia_icon_button(
        &mut cell,
        &palette,
        btn_ids[0],
        Icon::Shuffle,
        slots[0],
        shuffle,
        false,
        false,
        "Shuffle",
    )
    .clicked()
    {
        app.actions.push(Action::ToggleShuffle);
    }

    // Previous: Round circle with shape morph
    let mut cell = centered(ui, slots[1]);
    if caelestia_icon_button(
        &mut cell,
        &palette,
        btn_ids[1],
        Icon::SkipBackFilled,
        slots[1],
        false,
        false,
        false,
        "Previous",
    )
    .clicked()
    {
        app.actions.push(Action::Previous);
    }

    // Play / Pause: Circle when paused, rounded square when playing, with shape morph
    let disc = slots[2];
    let mut cell = centered(ui, disc);
    if loading || app.any_play_pending() {
        let play_shape_t = ui.ctx().animate_bool(btn_ids[2].with("play_shape"), true);
        let circle_r = disc.width().min(disc.height()) / 2.0;
        let radius = egui::lerp(circle_r..=12.0, play_shape_t);
        ui.painter().rect_filled(
            disc,
            CornerRadius::same(radius as u8),
            palette.accent,
        );
        theme::spinner(&mut cell, 20.0, palette.on_accent);
    } else {
        let icon = if playing {
            Icon::PauseFilled
        } else {
            Icon::PlayFilled
        };
        if caelestia_icon_button(
            &mut cell,
            &palette,
            btn_ids[2],
            icon,
            disc,
            false,
            true,
            playing,
            if playing { "Pause" } else { "Play" },
        )
        .clicked()
        {
            app.actions.push(Action::TogglePlay);
        }
    }

    // Next: Round circle with shape morph
    let mut cell = centered(ui, slots[3]);
    if caelestia_icon_button(
        &mut cell,
        &palette,
        btn_ids[3],
        Icon::SkipForwardFilled,
        slots[3],
        false,
        false,
        false,
        "Next",
    )
    .clicked()
    {
        app.actions.push(Action::Next);
    }

    // Repeat: Vertical pill with shape morph
    let (repeat_icon, repeat_active, tooltip) = match repeat {
        RepeatMode::Off => (Icon::Repeat, false, "Repeat"),
        RepeatMode::Context => (Icon::Repeat, true, "Repeat one"),
        RepeatMode::Track => (Icon::Repeat1, true, "Repeat off"),
    };
    let mut cell = centered(ui, slots[4]);
    if caelestia_icon_button(
        &mut cell,
        &palette,
        btn_ids[4],
        repeat_icon,
        slots[4],
        repeat_active,
        false,
        false,
        tooltip,
    )
    .clicked()
    {
        app.actions.push(Action::CycleRepeat);
    }
}

fn extras(app: &mut App, ui: &mut egui::Ui, now: Option<&NowPlaying>) {
    let palette = app.palette;
    ui.spacing_mut().item_spacing.x = 6.0;
    let volume = now
        .map(|now| now.volume_percent)
        .unwrap_or_else(|| crate::app::volume_to_percent(app.local.volume));
    let shown = match app.volume_preview {
        Some(fraction) => (fraction * 100.0).round() as u8,
        None => volume,
    };
    match thin_slider(
        ui,
        &palette,
        egui::Id::new("volume-slider"),
        shown as f32 / 100.0,
        120.0,
        palette.accent,
        Some(0.05),
    ) {
        SliderEvent::Dragging(value) => {
            app.volume_preview = Some(value);
            // Local volume is cheap to apply continuously; remote goes on release.
            if now.is_none_or(|now| now.local) {
                app.actions
                    .push(Action::PreviewVolume((value * 100.0).round() as u8));
            }
        }
        SliderEvent::Committed(value) => {
            app.volume_preview = None;
            app.actions
                .push(Action::SetVolume((value * 100.0).round() as u8));
        }
        SliderEvent::None => {}
    }
    let volume_icon = match shown {
        0 => Icon::VolumeX,
        1..=33 => Icon::Volume,
        34..=66 => Icon::Volume1,
        _ => Icon::Volume2,
    };
    if theme::icon_button(
        ui,
        volume_icon,
        22.0,
        palette.secondary,
        palette.text,
        if shown == 0 { "Unmute" } else { "Mute" },
    )
    .clicked()
    {
        app.actions.push(Action::ToggleMute);
    }
    ui.add_space(4.0);
    let remote = now.is_some_and(|now| !now.local);
    let devices = theme::icon_button(
        ui,
        Icon::Speaker,
        22.0,
        if remote {
            palette.accent
        } else {
            palette.secondary
        },
        palette.text,
        "Connect to a device",
    );
    ui.ctx().data_mut(|data| {
        data.insert_temp(egui::Id::new(super::devices::BUTTON_RECT_ID), devices.rect)
    });
    if devices.clicked() {
        app.actions.push(Action::ToggleDevicesPopup);
    }
    let queue_open = app.show_queue_panel || matches!(app.page(), Page::Queue);
    if theme::icon_button(
        ui,
        Icon::ListVideo,
        22.0,
        if queue_open {
            palette.accent
        } else {
            palette.secondary
        },
        palette.text,
        "Queue",
    )
    .clicked()
    {
        app.actions.push(Action::ToggleQueuePanel);
    }
    if theme::icon_button(
        ui,
        Icon::Mic,
        22.0,
        if app.show_lyrics_panel {
            palette.accent
        } else {
            palette.secondary
        },
        palette.text,
        "Lyrics",
    )
    .clicked()
    {
        app.actions.push(Action::ToggleLyricsPanel);
    }
}

fn button_expansion(ui: &egui::Ui, id: egui::Id) -> f32 {
    let current_time = ui.input(|i| i.time);
    let click_id = id.with("click_time");
    let last_click = ui.data(|d| d.get_temp::<f64>(click_id)).unwrap_or(0.0);
    let click_elapsed = (current_time - last_click).max(0.0);
    let click_bounce = if click_elapsed < 0.32 {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
        let t = (click_elapsed / 0.32) as f32;
        (t * std::f32::consts::PI).sin()
    } else {
        0.0
    };
    let press_t = ui.ctx().animate_bool(id.with("press"), false);
    press_t * 10.0 + click_bounce * 14.0
}

fn caelestia_icon_button(
    ui: &mut egui::Ui,
    palette: &theme::Palette,
    id: egui::Id,
    icon: Icon,
    rect: Rect,
    active: bool,
    is_primary: bool,
    is_playing_shape: bool,
    tooltip: &str,
) -> egui::Response {
    let mut response = ui.allocate_rect(rect, Sense::click());
    if ui.is_rect_visible(rect) {
        let current_time = ui.input(|i| i.time);
        let click_id = id.with("click_time");

        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();

        if response.clicked() {
            ui.data_mut(|d| d.insert_temp(click_id, current_time));
        }

        // Animate press state so button_expansion can read it on next frame
        ui.ctx().animate_bool(id.with("press"), pressed);
        let hover_t = ui.ctx().animate_bool(id.with("hover"), hovered);

        let scale = 1.0 + hover_t * 0.04;
        let draw_rect = Rect::from_center_size(rect.center(), rect.size() * scale);

        // Shape morphing:
        // Play button: circle when paused, rounded square when playing!
        // Toggle buttons (shuffle/repeat): pill when inactive, rounded squircle when active
        let circle_r = draw_rect.width().min(draw_rect.height()) / 2.0;
        let radius = if is_primary {
            let play_t = ui.ctx().animate_bool(id.with("play_shape"), is_playing_shape);
            let square_r = 12.0;
            egui::lerp(circle_r..=square_r, play_t)
        } else if active {
            let active_t = ui.ctx().animate_bool(id.with("active_shape"), true);
            egui::lerp(circle_r..=10.0, active_t)
        } else {
            circle_r
        };

        let fill = if is_primary {
            if hovered {
                palette.accent_hover
            } else {
                palette.accent
            }
        } else if active {
            if hovered {
                palette.accent_hover
            } else {
                palette.primary_container
            }
        } else if pressed {
            palette.surface_container_highest
        } else if hovered {
            palette.surface_container_highest
        } else {
            palette.surface_container_high
        };

        let icon_color = if is_primary {
            palette.on_accent
        } else if active {
            palette.on_primary_container
        } else {
            palette.text
        };

        ui.painter()
            .rect_filled(draw_rect, CornerRadius::same(radius as u8), fill);
        let icon_size = if is_primary { 22.0 } else { 17.0 };
        theme::paint_icon(ui, icon, draw_rect, icon_size, icon_color);
    }
    response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    if tooltip.is_empty() {
        response
    } else {
        response.on_hover_text(tooltip)
    }
}
