//! The desktop's state and its transitions: windows opened, raised, moved,
//! rolled up and zoomed, the icon selected and the menu open.
//!
//! Every transition is `Desktop -> Event -> Desktop`, total and pure; the
//! view dispatches events and draws the state, and nothing else changes it.

use super::command::{Action, Command, IconId, Link, MenuId};
use super::geometry::{Layout, Point, Rect, Viewport};
use super::window::{WindowId, WindowState};

/// The open windows, back to front: the last is in front and active.
///
/// Invariant: no window appears twice.
/// Preservation: [`Windows::raised_with`] removes a window before pushing it,
/// and every other operation only removes or maps in place.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Windows(Vec<(WindowId, WindowState)>);

impl Windows {
    /// Returns the state of `id`, if it is open.
    pub fn get(&self, id: WindowId) -> Option<WindowState> {
        self.0
            .iter()
            .find_map(|&(open, state)| (open == id).then_some(state))
    }

    /// Returns the window in front.
    pub fn front(&self) -> Option<WindowId> {
        self.0.last().map(|&(id, _)| id)
    }

    /// Returns whether `id` is the window in front.
    pub fn is_front(&self, id: WindowId) -> bool {
        self.front() == Some(id)
    }

    /// Returns the open windows with their states, back to front.
    pub fn iter(&self) -> impl Iterator<Item = (WindowId, WindowState)> + '_ {
        self.0.iter().copied()
    }

    /// Returns these windows with `id` in front in `state`.
    fn raised_with(self, id: WindowId, state: WindowState) -> Self {
        let mut windows = self.without(id).0;
        windows.push((id, state));
        Self(windows)
    }

    /// Returns these windows with `id` in front, unchanged, if it is open.
    fn raised(self, id: WindowId) -> Self {
        match self.get(id) {
            Some(state) => self.raised_with(id, state),
            None => self,
        }
    }

    /// Returns these windows with `id` closed.
    fn without(self, id: WindowId) -> Self {
        Self(self.0.into_iter().filter(|&(open, _)| open != id).collect())
    }

    /// Returns these windows with `id`'s state changed by `change`, in place.
    fn changed(self, id: WindowId, change: impl Fn(WindowState) -> WindowState) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|(open, state)| (open, if open == id { change(state) } else { state }))
                .collect(),
        )
    }

    /// Returns these windows with every state changed by `change`.
    fn all_changed(self, change: impl Fn(WindowId, WindowState) -> WindowState) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|(id, state)| (id, change(id, state)))
                .collect(),
        )
    }
}

/// A window being dragged by its title bar: a dotted outline follows the
/// pointer, and the window moves there once, when the pointer is let go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Drag {
    window: WindowId,
    grab: Point,
    start: Rect,
    outline: Rect,
}

impl Drag {
    /// Returns the window dragged.
    pub const fn window(self) -> WindowId {
        self.window
    }

    /// Returns where the window would move if let go now.
    pub const fn outline(self) -> Rect {
        self.outline
    }

    /// Returns whether the outline has left the window's frame.
    pub fn has_moved(self) -> bool {
        self.outline != self.start
    }
}

/// Something that happens to the desktop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// The viewport took a new size.
    Resized(Viewport),
    /// Opens a window, or brings it to the front, its content showing.
    Open(WindowId),
    /// Closes a window.
    Close(WindowId),
    /// Brings a window to the front.
    Raise(WindowId),
    /// Rolls a window up, or down.
    ToggleShade(WindowId),
    /// Zooms a window, or gives it its standard frame back.
    ToggleZoom(WindowId),
    /// The pointer pressed a window's title bar at a point.
    BeginDrag {
        /// The window pressed.
        window: WindowId,
        /// Where.
        at: Point,
    },
    /// The pointer moved, a title bar held.
    DragTo(Point),
    /// The pointer let go of a title bar.
    EndDrag,
    /// Selects an icon.
    Select(IconId),
    /// The desk itself was pressed: no icon is selected and no menu open.
    PressDesk,
    /// Opens a menu, or closes it if it is open.
    ToggleMenu(MenuId),
    /// The pointer came over a menu's title; it opens if another menu is open.
    HoverMenu(MenuId),
    /// Closes the menu open.
    CloseMenu,
    /// Escape: closes the menu open, or gives a drag up.
    Escape,
}

/// What a menu command comes to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// An event for the desktop.
    Event(Event),
    /// A page to visit elsewhere.
    Visit(Link),
}

impl From<Action> for Outcome {
    fn from(action: Action) -> Self {
        match action {
            Action::Open(window) => Outcome::Event(Event::Open(window)),
            Action::Visit(link) => Outcome::Visit(link),
        }
    }
}

/// The whole desktop.
///
/// Clone law: a clone is a snapshot of a value with no identity; the
/// reducer clones only when an older snapshot is still being drawn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Desktop {
    viewport: Viewport,
    windows: Windows,
    selection: Option<IconId>,
    menu: Option<MenuId>,
    drag: Option<Drag>,
}

impl Desktop {
    /// Returns the desktop at the start on `viewport`: About LispM open and in front.
    pub fn new(viewport: Viewport) -> Self {
        Self {
            viewport,
            windows: Windows::default().raised_with(
                WindowId::About,
                WindowState::opened(WindowId::About, viewport),
            ),
            selection: None,
            menu: None,
            drag: None,
        }
    }

    /// Returns the layout the desktop is drawn in.
    pub const fn layout(&self) -> Layout {
        self.viewport.layout()
    }

    /// Returns the open windows.
    pub const fn windows(&self) -> &Windows {
        &self.windows
    }

    /// Returns the state `id` is drawn in: as open on the desk, or, in the
    /// stacked layout, standing in the page whether or not it was opened.
    pub fn shown(&self, id: WindowId) -> Option<WindowState> {
        match self.layout() {
            Layout::Desktop => self.windows.get(id),
            Layout::Stacked => id.stacks().then(|| {
                self.windows
                    .get(id)
                    .unwrap_or_else(|| WindowState::opened(id, self.viewport))
            }),
        }
    }

    /// Returns whether `id` is drawn active: in front on the desk, or stacked.
    pub fn is_active(&self, id: WindowId) -> bool {
        match self.layout() {
            Layout::Desktop => self.windows.is_front(id),
            Layout::Stacked => true,
        }
    }

    /// Returns the icon selected.
    pub const fn selection(&self) -> Option<IconId> {
        self.selection
    }

    /// Returns the menu open.
    pub const fn menu(&self) -> Option<MenuId> {
        self.menu
    }

    /// Returns the drag under way.
    pub const fn drag(&self) -> Option<Drag> {
        self.drag
    }

    /// Returns what `command` comes to now, or nothing if it is disabled.
    pub fn resolve(&self, command: Command) -> Option<Outcome> {
        match command {
            Command::Do(action) => Some(action.into()),
            Command::OpenSelection => self.selection.map(|icon| icon.action().into()),
            Command::CloseFront => self
                .windows
                .front()
                .map(|id| Outcome::Event(Event::Close(id))),
        }
    }

    /// Returns the desktop after `event`.
    pub fn apply(self, event: Event) -> Self {
        match event {
            Event::Resized(viewport) => self.resized(viewport),
            Event::Open(id) => self.opened(id),
            Event::Close(id) => self.closed(id),
            Event::Raise(id) => Self {
                windows: self.windows.raised(id),
                menu: None,
                ..self
            },
            Event::ToggleShade(id) => self.with_window(id, WindowState::shade_toggled),
            Event::ToggleZoom(id) => {
                let viewport = self.viewport;
                self.with_window(id, |state| state.zoom_toggled(viewport))
            }
            Event::BeginDrag { window, at } => self.drag_begun(window, at),
            Event::DragTo(at) => self.dragged_to(at),
            Event::EndDrag => self.drag_ended(),
            Event::Select(icon) => Self {
                selection: Some(icon),
                menu: None,
                ..self
            },
            Event::PressDesk => Self {
                selection: None,
                menu: None,
                ..self
            },
            Event::ToggleMenu(menu) => self.menu_toggled(menu),
            Event::HoverMenu(menu) => self.menu_hovered(menu),
            Event::CloseMenu => Self { menu: None, ..self },
            Event::Escape => self.escaped(),
        }
    }

    /// Frames mean nothing while windows are stacked, so when the desktop
    /// layout begins every window is laid out afresh; within it, windows only
    /// move to keep their title bars on the desk.
    fn resized(self, viewport: Viewport) -> Self {
        let relay = self.layout() != viewport.layout() && viewport.layout().permits_drag();
        let windows = self.windows.all_changed(|id, state| {
            if relay {
                state.relaid(id, viewport)
            } else {
                state.kept_on(viewport)
            }
        });
        Self {
            viewport,
            windows,
            drag: None,
            ..self
        }
    }

    fn opened(self, id: WindowId) -> Self {
        let state = self.windows.get(id).map_or_else(
            || WindowState::opened(id, self.viewport),
            WindowState::unrolled,
        );
        Self {
            windows: self.windows.raised_with(id, state),
            menu: None,
            ..self
        }
    }

    fn closed(self, id: WindowId) -> Self {
        let drag = self.drag.filter(|drag| drag.window != id);
        Self {
            windows: self.windows.without(id),
            drag,
            ..self
        }
    }

    fn with_window(self, id: WindowId, change: impl Fn(WindowState) -> WindowState) -> Self {
        Self {
            windows: self.windows.changed(id, change).raised(id),
            ..self
        }
    }

    fn drag_begun(self, window: WindowId, at: Point) -> Self {
        let start = self.windows.get(window).map(WindowState::frame);
        match (self.layout().permits_drag(), start) {
            (true, Some(start)) => Self {
                windows: self.windows.raised(window),
                menu: None,
                drag: Some(Drag {
                    window,
                    grab: at,
                    start,
                    outline: start,
                }),
                ..self
            },
            _ => self,
        }
    }

    fn dragged_to(self, at: Point) -> Self {
        let desk = self.viewport.desk();
        let drag = self.drag.map(|drag| Drag {
            outline: drag
                .start
                .translated(at.offset_from(drag.grab))
                .kept_on(desk),
            ..drag
        });
        Self { drag, ..self }
    }

    fn drag_ended(self) -> Self {
        match self.drag {
            Some(drag) if drag.has_moved() => Self {
                windows: self
                    .windows
                    .changed(drag.window, |state| state.moved_to(drag.outline)),
                drag: None,
                ..self
            },
            _ => Self { drag: None, ..self },
        }
    }

    fn menu_toggled(self, menu: MenuId) -> Self {
        let open = if self.menu == Some(menu) {
            None
        } else {
            Some(menu)
        };
        Self { menu: open, ..self }
    }

    fn menu_hovered(self, menu: MenuId) -> Self {
        let open = self.menu.map(|_| menu);
        Self { menu: open, ..self }
    }

    fn escaped(self) -> Self {
        match self.menu {
            Some(_) => Self { menu: None, ..self },
            None => Self { drag: None, ..self },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::{MENU_BAR_HEIGHT, Offset};

    const LAPTOP: Viewport = Viewport::new(1440, 900);
    const PHONE: Viewport = Viewport::new(375, 812);

    fn run(events: &[Event]) -> Desktop {
        events
            .iter()
            .fold(Desktop::new(LAPTOP), |desktop, &event| desktop.apply(event))
    }

    fn order(desktop: &Desktop) -> Vec<WindowId> {
        desktop.windows().iter().map(|(id, _)| id).collect()
    }

    #[test]
    fn the_desktop_opens_with_about_in_front() {
        let desktop = Desktop::new(LAPTOP);
        assert_eq!(order(&desktop), vec![WindowId::About]);
        assert!(desktop.is_active(WindowId::About));
    }

    #[test]
    fn opening_an_open_window_raises_it_without_a_copy() {
        let desktop = run(&[
            Event::Open(WindowId::Agent),
            Event::Open(WindowId::Browser),
            Event::Open(WindowId::Agent),
        ]);
        assert_eq!(
            order(&desktop),
            vec![WindowId::About, WindowId::Browser, WindowId::Agent]
        );
    }

    #[test]
    fn opening_a_rolled_up_window_rolls_it_down() {
        let desktop = run(&[
            Event::ToggleShade(WindowId::About),
            Event::Open(WindowId::About),
        ]);
        assert_eq!(
            desktop
                .windows()
                .get(WindowId::About)
                .map(WindowState::is_rolled_up),
            Some(false)
        );
    }

    #[test]
    fn closing_the_front_window_activates_the_one_behind() {
        let desktop = run(&[Event::Open(WindowId::Agent), Event::Close(WindowId::Agent)]);
        assert_eq!(desktop.windows().front(), Some(WindowId::About));
    }

    #[test]
    fn a_drag_moves_the_window_only_when_let_go() {
        let start = WindowId::About.standard_frame(LAPTOP);
        let grab = Point::new(start.left() + 100, start.top() + 10);
        let to = Point::new(grab.x + 40, grab.y + 30);
        let dragging = run(&[
            Event::BeginDrag {
                window: WindowId::About,
                at: grab,
            },
            Event::DragTo(to),
        ]);
        assert_eq!(
            dragging
                .windows()
                .get(WindowId::About)
                .map(WindowState::frame),
            Some(start)
        );
        let expected = start.translated(Offset { dx: 40, dy: 30 });
        assert_eq!(dragging.drag().map(Drag::outline), Some(expected));
        let dropped = dragging.apply(Event::EndDrag);
        assert_eq!(
            dropped
                .windows()
                .get(WindowId::About)
                .map(WindowState::frame),
            Some(expected)
        );
        assert_eq!(dropped.drag(), None);
    }

    #[test]
    fn escape_gives_a_drag_up_and_leaves_the_window() {
        let start = WindowId::About.standard_frame(LAPTOP);
        let desktop = run(&[
            Event::BeginDrag {
                window: WindowId::About,
                at: start.origin,
            },
            Event::DragTo(Point::new(5, 600)),
            Event::Escape,
            Event::EndDrag,
        ]);
        assert_eq!(
            desktop
                .windows()
                .get(WindowId::About)
                .map(WindowState::frame),
            Some(start)
        );
    }

    #[test]
    fn a_drag_keeps_the_title_bar_under_the_menu_bar() {
        let start = WindowId::About.standard_frame(LAPTOP);
        let desktop = run(&[
            Event::BeginDrag {
                window: WindowId::About,
                at: start.origin,
            },
            Event::DragTo(Point::new(start.left(), -400)),
            Event::EndDrag,
        ]);
        let frame = desktop
            .windows()
            .get(WindowId::About)
            .map(WindowState::frame);
        assert_eq!(frame.map(Rect::top), Some(MENU_BAR_HEIGHT));
    }

    #[test]
    fn windows_do_not_drag_in_the_stacked_layout() {
        let desktop = Desktop::new(PHONE).apply(Event::BeginDrag {
            window: WindowId::About,
            at: Point::new(30, 30),
        });
        assert_eq!(desktop.drag(), None);
    }

    #[test]
    fn the_stacked_layout_shows_every_window_but_the_trash() {
        let desktop = Desktop::new(PHONE);
        for id in WindowId::ALL {
            assert_eq!(desktop.shown(id).is_some(), id.stacks(), "{id:?}");
        }
    }

    #[test]
    fn hovering_switches_menus_only_while_one_is_open() {
        let closed = run(&[Event::HoverMenu(MenuId::Go)]);
        assert_eq!(closed.menu(), None);
        let switched = run(&[
            Event::ToggleMenu(MenuId::File),
            Event::HoverMenu(MenuId::Go),
        ]);
        assert_eq!(switched.menu(), Some(MenuId::Go));
        let toggled = run(&[Event::ToggleMenu(MenuId::Go), Event::ToggleMenu(MenuId::Go)]);
        assert_eq!(toggled.menu(), None);
    }

    #[test]
    fn open_selection_is_disabled_until_an_icon_is_selected() {
        let desktop = Desktop::new(LAPTOP);
        assert_eq!(desktop.resolve(Command::OpenSelection), None);
        let selected = desktop.apply(Event::Select(IconId::GitHub));
        assert_eq!(
            selected.resolve(Command::OpenSelection),
            Some(Outcome::Visit(Link::Organization))
        );
    }

    #[test]
    fn close_front_is_disabled_with_no_window_open() {
        let desktop = run(&[Event::Close(WindowId::About)]);
        assert_eq!(desktop.resolve(Command::CloseFront), None);
    }

    #[test]
    fn widening_from_stacked_lays_windows_out_afresh() {
        let desktop = Desktop::new(Viewport::new(0, 0))
            .apply(Event::ToggleShade(WindowId::About))
            .apply(Event::Resized(LAPTOP));
        let about = desktop.windows().get(WindowId::About);
        assert_eq!(
            about.map(WindowState::frame),
            Some(WindowId::About.standard_frame(LAPTOP))
        );
        assert_eq!(about.map(WindowState::is_rolled_up), Some(true));
    }

    #[test]
    fn resizing_keeps_every_title_bar_reachable() {
        let desktop = run(&[
            Event::Open(WindowId::Trash),
            Event::Resized(Viewport::new(800, 560)),
        ]);
        let desk = Viewport::new(800, 560).desk();
        for (id, state) in desktop.windows().iter() {
            assert_eq!(state.frame(), state.frame().kept_on(desk), "{id:?}");
        }
    }
}
