//! penrose :: minimal configuration
//!
//! This file will give you a functional if incredibly minimal window manager that
//! has multiple workspaces and simple client / workspace movement.
use penrose::{
    builtin::{
        actions::{
            exit,
            floating::{sink_focused, MouseDragHandler, MouseResizeHandler},
            modify_with, send_layout_message, spawn,
        },
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReserveTop},
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{
            parse_keybindings_with_xmodmap, KeyEventHandler, MouseEventHandler,
            MouseState,
        },
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::hooks::add_ewmh_hooks,
    map, stack,
    x11rb::RustConn,
    Result,
};
use penrose_ui::{bar::Position, core::TextStyle, status_bar};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

const FONT: &str = "Noto Sans CJK TC";
const BLACK: u32 = 0x282828ff;
const WHITE: u32 = 0xebdbb2ff;
const GREY: u32 = 0x3c3836ff;
const BLUE: u32 = 0x458588ff;

const MAX_MAIN: u32 = 1;
const RATIO: f32 = 0.6;
const RATIO_STEP: f32 = 0.1;
const OUTER_PX: u32 = 0;
const INNER_PX: u32 = 5;
const BAR_HEIGHT_PX: u32 = 30;

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "XF86MonBrightnessUp" => spawn("xbacklight +5"),
        "XF86MonBrightnessDown" => spawn("xbacklight -5"),
        "XF86AudioRaiseVolume" => spawn("pulsemixer --change-volume +5"),
        "XF86AudioLowerVolume" => spawn("pulsemixer --change-volume -5"),
        "XF86AudioMute" => spawn("pulsemixer --toggle-mute"),
        "XF86AudioMicMute" => spawn("mute-mic"),
        "Print" => spawn("scrot -s"),
        "M-u" => modify_with(|cs| cs.focus_down()),
        "M-o" => modify_with(|cs| cs.focus_up()),
        "M-l" => modify_with(|cs| cs.swap_down()),
        "M-j" => modify_with(|cs| cs.swap_up()),
        "M-S-c" => modify_with(|cs| cs.kill_focused()),
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-w" => modify_with(|cs| cs.focus_screen(0)),
        "M-e" => modify_with(|cs| cs.focus_screen(1)),
        "M-S-e" => modify_with(|cs| cs.move_focused_to_screen(1)),
        "M-S-w" => modify_with(|cs| cs.move_focused_to_screen(0)),
        "M-space" => modify_with(|cs| cs.next_layout()),
        "M-S-space" => modify_with(|cs| cs.previous_layout()),
        "M-S-i" => send_layout_message(|| IncMain(1)),
        "M-S-k" => send_layout_message(|| IncMain(-1)),
        "M-S-l" => send_layout_message(|| ExpandMain),
        "M-S-j" => send_layout_message(|| ShrinkMain),
        "M-p" => spawn("dmenu_run -c -l 14 -g 4"),
        "M-S-Return" => spawn("st"),
        "M-d" => spawn("vesktop"),
        "M-F7" => spawn("iwd-dmenu"),
        "M-S-q" => spawn("pkill -fi penrose"),
        "M-f" => spawn("firefox"),
        "M-S-f" => spawn("firefox --private-window"),
        "M-q" => exit(),
        "M-t" => sink_focused(),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn mouse_bindings() -> HashMap<MouseState, Box<dyn MouseEventHandler<RustConn>>> {
    use penrose::core::bindings::{
        ModifierKey::{Meta},
        MouseButton::{Left, Right},
    };

    map! {
        map_keys: |(button, modifiers)| MouseState { button, modifiers };

        (Left, vec![Meta]) => MouseDragHandler::boxed_default(),
        (Right, vec![Meta]) => MouseResizeHandler::boxed_default(),
        // (Middle, vec![Shift, Meta]) => click_handler(sink_focused()),
    }
}

fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::side_mirrored(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX))
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        ..Config::default()
    });
    let style = TextStyle {
        fg: WHITE.into(),
        bg: None,
        padding: (2, 2),
    };

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let bar = status_bar(BAR_HEIGHT_PX, FONT, 8, style, BLUE, GREY, Position::Top).unwrap();

    let wm = bar.add_to(WindowManager::new(
            config, 
            key_bindings, 
            mouse_bindings(), 
            conn
    )?);

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
