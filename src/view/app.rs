//! The whole screen: the menu bar, the desk with its icons and windows, and
//! the outline of a window being dragged.

use std::rc::Rc;

use yew::prelude::*;

use crate::effects;
use crate::model::command::IconId;
use crate::model::desktop::{Desktop, Event, Outcome};
use crate::model::geometry::{Layout, Rect, TITLE_BAR_HEIGHT};
use crate::model::window::{WindowId, WindowState};

use super::content::content;
use super::icon::DeskIcon;
use super::menu_bar::{MenuBar, menu_lines};
use super::window::PlatinumWindow;

impl Reducible for Desktop {
    type Action = Event;

    fn reduce(self: Rc<Self>, event: Event) -> Rc<Self> {
        // A snapshot still being drawn is copied; otherwise the state moves.
        Rc::new(Rc::unwrap_or_clone(self).apply(event))
    }
}

/// What every component on the desk needs: where to send events, and the layout.
///
/// Clone law: a clone shares the same callback; it names the same desk.
#[derive(Clone, PartialEq)]
pub struct Desk {
    /// Sends an event to the desktop.
    pub on_event: Callback<Event>,
    /// The layout the desktop is drawn in.
    pub layout: Layout,
}

impl Desk {
    /// Carries out `outcome`: sends its event, scrolling to a window opened in
    /// the stacked layout, or visits its page.
    pub fn perform(&self, outcome: Outcome) {
        match outcome {
            Outcome::Event(event) => {
                self.on_event.emit(event);
                if let (Layout::Stacked, Event::Open(window)) = (self.layout, event) {
                    effects::reveal(window);
                }
            }
            Outcome::Visit(link) => effects::visit(link),
        }
    }

    /// Returns a callback sending `event`, whatever it is called with.
    pub fn emits<T>(&self, event: Event) -> Callback<T> {
        let on_event = self.on_event.clone();
        Callback::from(move |_| on_event.emit(event))
    }
}

/// Keeps the desktop told of the viewport's size and of Escape.
#[hook]
fn use_page_listeners(on_event: Callback<Event>) {
    use_effect_with((), move |_| {
        let resized = on_event.clone();
        let resize = effects::on_resize(move |viewport| resized.emit(Event::Resized(viewport)));
        let escape = effects::on_escape(move || on_event.emit(Event::Escape));
        move || drop((resize, escape))
    });
}

/// Returns the class naming `layout` on the screen.
const fn layout_class(layout: Layout) -> &'static str {
    match layout {
        Layout::Desktop => "desktop-layout",
        Layout::Stacked => "stacked-layout",
    }
}

/// Returns the CSS placing an element on `rect`, its height left out if `fit`.
pub fn place(rect: Rect, fit: bool) -> String {
    let size = rect.size;
    let height = if fit {
        String::new()
    } else {
        format!("height:{}px;", size.height())
    };
    format!(
        "left:{}px;top:{}px;width:{}px;{height}",
        rect.left(),
        rect.top(),
        size.width()
    )
}

/// The site.
#[function_component(App)]
pub fn app() -> Html {
    let desktop = use_reducer_eq(|| Desktop::new(effects::viewport()));
    let on_event = {
        let dispatcher = desktop.dispatcher();
        Callback::from(move |event| dispatcher.dispatch(event))
    };
    use_page_listeners(on_event.clone());
    let layout = desktop.layout();
    let desk = Desk { on_event, layout };
    let open_menu = desktop.menu();
    let lines = open_menu
        .map(|menu| menu_lines(&desktop, menu))
        .unwrap_or_default();
    // Windows and icons keep their presses to themselves, so a press that
    // reaches the desk is a press on the desk itself.
    let press_desk = desk.emits::<PointerEvent>(Event::PressDesk);
    html! {
        <ContextProvider<Desk> context={desk}>
            <div class={classes!("screen", layout_class(layout))}>
                <MenuBar open={open_menu} {lines} />
                <main class="desk" onpointerdown={press_desk}>
                    { icons(&desktop) }
                    { windows(&desktop) }
                    { outline(&desktop) }
                </main>
            </div>
        </ContextProvider<Desk>>
    }
}

/// Returns the icons, down the desk's right edge, the Trash in its corner.
fn icons(desktop: &Desktop) -> Html {
    let icon = |icon: IconId| {
        html! { <DeskIcon key={icon.label()} {icon} selected={desktop.selection() == Some(icon)} /> }
    };
    html! {
        <nav class="icons" aria-label="Desktop">
            { for IconId::ALL.into_iter().filter(|icon| !icon.is_cornered()).map(icon) }
            <div class="corner">
                { for IconId::ALL.into_iter().filter(|icon| icon.is_cornered()).map(icon) }
            </div>
        </nav>
    }
}

/// Returns the windows: back to front on the desk, or in their order stacked.
fn windows(desktop: &Desktop) -> Html {
    let order: Vec<WindowId> = match desktop.layout() {
        Layout::Desktop => desktop.windows().iter().map(|(id, _)| id).collect(),
        Layout::Stacked => WindowId::ALL.to_vec(),
    };
    let window = |id: WindowId| {
        desktop.shown(id).map(|state| {
            html! {
                <PlatinumWindow key={id.element_id()} {id} {state} active={desktop.is_active(id)}>
                    { content(id) }
                </PlatinumWindow>
            }
        })
    };
    html! { <>{ for order.into_iter().filter_map(window) }</> }
}

/// Returns the dotted outline of the window being dragged, if one is: as tall
/// as its title bar alone if it is rolled up.
fn outline(desktop: &Desktop) -> Html {
    match desktop.drag().filter(|drag| drag.has_moved()) {
        Some(drag) => {
            let rolled = desktop
                .windows()
                .get(drag.window())
                .is_some_and(WindowState::is_rolled_up);
            let rect = drag.outline();
            let rect = if rolled {
                Rect::new(rect.left(), rect.top(), rect.size.width(), ROLLED_HEIGHT)
            } else {
                rect
            };
            outline_at(rect)
        }
        None => Html::default(),
    }
}

/// Height in pixels of a rolled up window: its title bar, outline and shadow.
const ROLLED_HEIGHT: i32 = TITLE_BAR_HEIGHT + 2;

/// Returns a dotted outline on `rect`, its edges drawn inverted over what is below.
fn outline_at(rect: Rect) -> Html {
    html! {
        <div class="drag-outline" style={place(rect, false)} aria-hidden="true">
            <span class="edge top" /><span class="edge right" />
            <span class="edge bottom" /><span class="edge left" />
        </div>
    }
}
