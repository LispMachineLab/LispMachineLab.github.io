//! The windows of the desktop: which there are, where each opens, and how
//! one is rolled up and zoomed.

use super::geometry::{Rect, Viewport};

/// Pixels between the desk's edges and a zoomed window.
const ZOOM_MARGIN: i32 = 8;

/// Pixels each stacked project window opens right of and below the one before.
const STAGGER: (i32, i32) = (28, 26);

/// A window of the site; each exists at most once.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindowId {
    /// What LispM is, open at the start.
    About,
    /// LispM Agent's window.
    Agent,
    /// LispM Desktop's window.
    Desktop,
    /// LispM Browser's window.
    Browser,
    /// How to install the three projects.
    ReadMe,
    /// The Trash, empty.
    Trash,
}

impl WindowId {
    /// Every window, in the order the stacked layout shows them.
    pub const ALL: [WindowId; 6] = [
        WindowId::About,
        WindowId::Agent,
        WindowId::Desktop,
        WindowId::Browser,
        WindowId::ReadMe,
        WindowId::Trash,
    ];

    /// Returns the title on the window's title bar.
    pub const fn title(self) -> &'static str {
        match self {
            WindowId::About => "About LispM",
            WindowId::Agent => "LispM Agent",
            WindowId::Desktop => "LispM Desktop",
            WindowId::Browser => "LispM Browser",
            WindowId::ReadMe => "Read Me",
            WindowId::Trash => "Trash",
        }
    }

    /// Returns the window's element id in the page, unique among windows.
    pub const fn element_id(self) -> &'static str {
        match self {
            WindowId::About => "about",
            WindowId::Agent => "agent",
            WindowId::Desktop => "desktop",
            WindowId::Browser => "browser",
            WindowId::ReadMe => "read-me",
            WindowId::Trash => "trash",
        }
    }

    /// Returns whether the stacked layout shows this window: every one but the Trash.
    pub const fn stacks(self) -> bool {
        !matches!(self, WindowId::Trash)
    }

    /// Returns the frame the window opens in on `viewport`, kept on its desk.
    pub const fn standard_frame(self, viewport: Viewport) -> Rect {
        let area = viewport.window_area();
        let width = area.size.width();
        let height = area.size.height();
        let frame = match self {
            WindowId::About => {
                let w = least(620, width - 48);
                let h = least(660, height - 56);
                Rect::new(area.left() + (width - w) / 2, area.top() + 28, w, h)
            }
            WindowId::Agent => staggered(area, 0, 680),
            WindowId::Desktop => staggered(area, 1, 680),
            WindowId::Browser => staggered(area, 2, 680),
            WindowId::ReadMe => staggered(area, 3, 600),
            WindowId::Trash => Rect::new(area.right() - 324, area.bottom() - 228, 300, 180),
        };
        frame.kept_on(viewport.desk())
    }
}

/// Returns the smaller of `a` and `b`.
const fn least(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

/// Returns the frame of the `step`th staggered window of `area`, at most `width` wide.
const fn staggered(area: Rect, step: i32, width: i32) -> Rect {
    let x = area.left() + 32 + STAGGER.0 * step;
    let y = area.top() + 24 + STAGGER.1 * step;
    Rect::new(
        x,
        y,
        least(width, area.right() - x - 24),
        least(580, area.bottom() - y - 24),
    )
}

/// Whether a window shows its content or only its title bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shade {
    /// The content shows.
    Down,
    /// Rolled up to the title bar, as Mac OS 9's collapse box left it.
    Up,
}

/// Whether a window has its standard frame or fills the desk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Zoom {
    /// The frame it was opened or moved to.
    Standard,
    /// Filling the window area; `standard` is the frame the zoom box gives back.
    Zoomed {
        /// The frame before zooming.
        standard: Rect,
    },
}

/// An open window's frame, shade and zoom.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowState {
    frame: Rect,
    shade: Shade,
    zoom: Zoom,
}

impl WindowState {
    /// Returns the state of `id` opened on `viewport`: its standard frame, content showing.
    pub const fn opened(id: WindowId, viewport: Viewport) -> Self {
        Self {
            frame: id.standard_frame(viewport),
            shade: Shade::Down,
            zoom: Zoom::Standard,
        }
    }

    /// Returns the window's frame.
    pub const fn frame(self) -> Rect {
        self.frame
    }

    /// Returns whether the window is rolled up to its title bar.
    pub const fn is_rolled_up(self) -> bool {
        matches!(self.shade, Shade::Up)
    }

    /// Returns the window rolled up if it was down, and down if it was up.
    pub const fn shade_toggled(self) -> Self {
        let shade = match self.shade {
            Shade::Down => Shade::Up,
            Shade::Up => Shade::Down,
        };
        Self { shade, ..self }
    }

    /// Returns the window with its content showing.
    pub const fn unrolled(self) -> Self {
        Self {
            shade: Shade::Down,
            ..self
        }
    }

    /// Returns the window zoomed to fill `viewport`'s window area, or given back
    /// its standard frame if it was zoomed.
    ///
    /// Law: `s.zoom_toggled(v).zoom_toggled(v) == s` while `s` is not zoomed.
    pub const fn zoom_toggled(self, viewport: Viewport) -> Self {
        match self.zoom {
            Zoom::Standard => Self {
                frame: viewport.window_area().inset(
                    ZOOM_MARGIN,
                    ZOOM_MARGIN,
                    ZOOM_MARGIN,
                    ZOOM_MARGIN,
                ),
                zoom: Zoom::Zoomed {
                    standard: self.frame,
                },
                ..self
            },
            Zoom::Zoomed { standard } => Self {
                frame: standard,
                zoom: Zoom::Standard,
                ..self
            },
        }
    }

    /// Returns the window moved to `frame`, which becomes its standard frame.
    pub const fn moved_to(self, frame: Rect) -> Self {
        Self {
            frame,
            zoom: Zoom::Standard,
            ..self
        }
    }

    /// Returns the window `id` laid out afresh on `viewport`: its standard
    /// frame, its shade kept.
    pub const fn relaid(self, id: WindowId, viewport: Viewport) -> Self {
        Self {
            frame: id.standard_frame(viewport),
            zoom: Zoom::Standard,
            ..self
        }
    }

    /// Returns the window moved so that its title bar stays reachable on `viewport`.
    pub const fn kept_on(self, viewport: Viewport) -> Self {
        Self {
            frame: self.frame.kept_on(viewport.desk()),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAPTOP: Viewport = Viewport::new(1440, 900);

    #[test]
    fn element_ids_are_distinct() {
        for (i, a) in WindowId::ALL.iter().enumerate() {
            for b in WindowId::ALL.iter().skip(i + 1) {
                assert_ne!(a.element_id(), b.element_id());
            }
        }
    }

    #[test]
    fn standard_frames_lie_in_the_window_area() {
        let area = LAPTOP.window_area();
        for id in WindowId::ALL {
            let frame = id.standard_frame(LAPTOP);
            assert!(frame.left() >= area.left(), "{id:?}");
            assert!(frame.top() >= area.top(), "{id:?}");
            assert!(frame.right() <= area.right(), "{id:?}");
            assert!(frame.bottom() <= area.bottom(), "{id:?}");
        }
    }

    #[test]
    fn project_windows_open_staggered() {
        let agent = WindowId::Agent.standard_frame(LAPTOP);
        let desktop = WindowId::Desktop.standard_frame(LAPTOP);
        assert_eq!(desktop.left() - agent.left(), STAGGER.0);
        assert_eq!(desktop.top() - agent.top(), STAGGER.1);
    }

    #[test]
    fn zoom_toggled_twice_gives_the_standard_frame_back() {
        let state = WindowState::opened(WindowId::Agent, LAPTOP);
        let zoomed = state.zoom_toggled(LAPTOP);
        let area = LAPTOP.window_area();
        let filled = area.inset(ZOOM_MARGIN, ZOOM_MARGIN, ZOOM_MARGIN, ZOOM_MARGIN);
        assert_eq!(zoomed.frame(), filled);
        assert_eq!(zoomed.zoom_toggled(LAPTOP), state);
    }

    #[test]
    fn moving_a_zoomed_window_makes_its_frame_standard() {
        let zoomed = WindowState::opened(WindowId::Agent, LAPTOP).zoom_toggled(LAPTOP);
        let target = Rect::new(50, 60, 300, 200);
        let moved = zoomed.moved_to(target);
        assert_eq!(moved.zoom, Zoom::Standard);
        assert_eq!(
            moved.zoom_toggled(LAPTOP).zoom_toggled(LAPTOP).frame(),
            target
        );
    }

    #[test]
    fn shade_toggled_twice_is_the_identity() {
        let state = WindowState::opened(WindowId::About, LAPTOP);
        assert!(state.shade_toggled().is_rolled_up());
        assert_eq!(state.shade_toggled().shade_toggled(), state);
        assert!(!state.shade_toggled().unrolled().is_rolled_up());
    }

    #[test]
    fn only_the_trash_is_left_out_of_the_stack() {
        let left_out: Vec<WindowId> = WindowId::ALL
            .into_iter()
            .filter(|id| !id.stacks())
            .collect();
        assert_eq!(left_out, vec![WindowId::Trash]);
    }
}
