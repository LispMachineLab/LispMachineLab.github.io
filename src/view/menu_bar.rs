//! The menu bar: the rainbow λ's menu and File, Go and Help at its left end,
//! the clock and the application's name at its right.

use gloo_timers::callback::Interval;
use yew::prelude::*;

use crate::effects;
use crate::model::command::{MenuEntry, MenuId};
use crate::model::desktop::{Desktop, Event, Outcome};
use crate::model::geometry::Layout;

use super::app::Desk;

/// Milliseconds between readings of the clock.
const CLOCK_PERIOD_MS: u32 = 10_000;

/// A line of a menu as drawn now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Line {
    /// An item: its text, and what choosing it comes to, nothing if it is disabled.
    Item(&'static str, Option<Outcome>),
    /// An engraved line.
    Separator,
}

/// Returns the lines of `menu` as `desktop` enables them.
pub fn menu_lines(desktop: &Desktop, menu: MenuId) -> Vec<Line> {
    menu.entries()
        .iter()
        .map(|entry| match *entry {
            MenuEntry::Item { label, command } => Line::Item(label, desktop.resolve(command)),
            MenuEntry::Separator => Line::Separator,
        })
        .collect()
}

/// The properties of the [`MenuBar`].
#[derive(Properties, PartialEq)]
pub struct MenuBarProps {
    /// The menu open.
    pub open: Option<MenuId>,
    /// The open menu's lines.
    pub lines: Vec<Line>,
}

/// The menu bar along the top of the screen.
#[function_component(MenuBar)]
pub fn menu_bar(props: &MenuBarProps) -> Html {
    let Some(desk) = use_context::<Desk>() else {
        return Html::default();
    };
    let stacked = matches!(desk.layout, Layout::Stacked);
    html! {
        <header class="menu-bar">
            <nav class="menus" role="menubar" aria-label="Menu bar">
                { for MenuId::ALL.into_iter().map(|menu| menu_title(&desk, menu, props)) }
            </nav>
            <div class="status">
                <Clock />
                if !stacked {
                    <span class="application">
                        <img src="assets/icons/lispm.svg" alt="" width="16" height="16" />
                        { "LispM" }
                    </span>
                }
            </div>
        </header>
    }
}

/// Returns `menu`'s title, and its lines under it if it is open.
fn menu_title(desk: &Desk, menu: MenuId, props: &MenuBarProps) -> Html {
    let open = props.open == Some(menu);
    let toggle = {
        let on_event = desk.on_event.clone();
        Callback::from(move |event: PointerEvent| {
            event.stop_propagation();
            if effects::is_primary(&event) {
                on_event.emit(Event::ToggleMenu(menu));
            }
        })
    };
    let title = match menu {
        MenuId::Lambda => html! {
            <img src="assets/lispm-lambda.png" alt={menu.title()} width="16" height="16" />
        },
        _ => html! { menu.title() },
    };
    html! {
        <div class={classes!("menu", open.then_some("open"), matches!(menu, MenuId::Lambda).then_some("lambda"))}>
            <button type="button" class="menu-title" role="menuitem" aria-haspopup="menu"
                    aria-expanded={open.to_string()} onpointerdown={toggle}
                    onpointerenter={desk.emits::<PointerEvent>(Event::HoverMenu(menu))}
                    onkeydown={keyboard_toggle(desk, menu)}>
                { title }
            </button>
            if open {
                <ul class="menu-list" role="menu" aria-label={menu.title()}>
                    { for props.lines.iter().map(|&line| menu_line(desk, line)) }
                </ul>
            }
        </div>
    }
}

/// Returns the callback opening or closing `menu` from the keyboard.
fn keyboard_toggle(desk: &Desk, menu: MenuId) -> Callback<KeyboardEvent> {
    let on_event = desk.on_event.clone();
    Callback::from(move |event: KeyboardEvent| {
        if matches!(event.key().as_str(), "Enter" | " " | "ArrowDown") {
            event.prevent_default();
            on_event.emit(Event::ToggleMenu(menu));
        }
    })
}

/// Returns a line of an open menu.
fn menu_line(desk: &Desk, line: Line) -> Html {
    match line {
        Line::Separator => html! { <li class="separator" role="separator" /> },
        Line::Item(label, outcome) => {
            let choose = {
                let desk = desk.clone();
                Callback::from(move |event: MouseEvent| {
                    event.stop_propagation();
                    desk.on_event.emit(Event::CloseMenu);
                    if let Some(outcome) = outcome {
                        desk.perform(outcome);
                    }
                })
            };
            let stop = Callback::from(|event: PointerEvent| event.stop_propagation());
            html! {
                <li role="none">
                    <button type="button" role="menuitem" disabled={outcome.is_none()}
                            onpointerdown={stop} onclick={choose}>
                        { label }
                    </button>
                </li>
            }
        }
    }
}

/// The time, as the menu bar shows it, read every few seconds.
#[function_component(Clock)]
fn clock() -> Html {
    let time = use_state_eq(effects::now);
    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(CLOCK_PERIOD_MS, move || time.set(effects::now()));
            move || drop(interval)
        });
    }
    html! {
        <span class="clock">{ time.map(|time| time.label()).unwrap_or_default() }</span>
    }
}
