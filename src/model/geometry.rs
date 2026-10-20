//! Desktop geometry in CSS pixels: points, sizes, rectangles and the viewport.
//!
//! The origin is the viewport's top left corner, where the menu bar starts;
//! the desk, where windows and icons lie, is the viewport below the menu bar.

/// Height in pixels of the menu bar, as Mac OS 9's.
pub const MENU_BAR_HEIGHT: i32 = 20;

/// Height in pixels of a window's title bar with the outline above it.
pub const TITLE_BAR_HEIGHT: i32 = 22;

/// Width in pixels a window's title bar keeps on the desk, so it can be grabbed again.
pub const GRAB_WIDTH: i32 = 64;

/// Width in pixels of the column of icons down the desk's right edge.
pub const ICON_COLUMN_WIDTH: i32 = 104;

/// The smallest viewport the desktop layout is drawn in; smaller ones stack.
const DESKTOP_MINIMUM: Size = Size::new(760, 540);

/// Returns `value` within `low..=high`, or `low` when the range is empty.
///
/// Post: `low <= high` implies `low <= result <= high`; otherwise `result == low`.
const fn bounded(value: i32, low: i32, high: i32) -> i32 {
    let capped = if value > high { high } else { value };
    if capped < low { low } else { capped }
}

/// A point on the viewport.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    /// Pixels from the viewport's left edge.
    pub x: i32,
    /// Pixels from the viewport's top edge.
    pub y: i32,
}

impl Point {
    /// Returns the point `x` pixels right and `y` pixels down of the origin.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the displacement that carries `origin` to this point.
    pub const fn offset_from(self, origin: Point) -> Offset {
        Offset {
            dx: self.x.saturating_sub(origin.x),
            dy: self.y.saturating_sub(origin.y),
        }
    }
}

/// A displacement between two points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Offset {
    /// Pixels to the right; negative to the left.
    pub dx: i32,
    /// Pixels down; negative up.
    pub dy: i32,
}

/// A width and a height, neither negative.
///
/// Invariant: `width >= 0 && height >= 0`; [`Size::new`] floors both at zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    width: i32,
    height: i32,
}

impl Size {
    /// Returns the size `width` by `height`, a negative extent taken as zero.
    pub const fn new(width: i32, height: i32) -> Self {
        Self {
            width: if width < 0 { 0 } else { width },
            height: if height < 0 { 0 } else { height },
        }
    }

    /// Returns the width.
    pub const fn width(self) -> i32 {
        self.width
    }

    /// Returns the height.
    pub const fn height(self) -> i32 {
        self.height
    }

    /// Returns whether this size is at least `other` in both extents.
    pub const fn covers(self, other: Size) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

/// A rectangle: its top left corner and its size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    /// The top left corner.
    pub origin: Point,
    /// The width and height.
    pub size: Size,
}

impl Rect {
    /// Returns the rectangle at `x`, `y`, `width` by `height`.
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    /// Returns the left edge.
    pub const fn left(self) -> i32 {
        self.origin.x
    }

    /// Returns the top edge.
    pub const fn top(self) -> i32 {
        self.origin.y
    }

    /// Returns the right edge, just past the last column.
    pub const fn right(self) -> i32 {
        self.origin.x.saturating_add(self.size.width())
    }

    /// Returns the bottom edge, just past the last row.
    pub const fn bottom(self) -> i32 {
        self.origin.y.saturating_add(self.size.height())
    }

    /// Returns this rectangle moved by `offset`.
    pub const fn translated(self, offset: Offset) -> Self {
        Self {
            origin: Point::new(
                self.origin.x.saturating_add(offset.dx),
                self.origin.y.saturating_add(offset.dy),
            ),
            size: self.size,
        }
    }

    /// Returns this rectangle with `left`, `top`, `right` and `bottom` pixels
    /// taken off its sides, its size floored at zero.
    pub const fn inset(self, left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect::new(
            self.origin.x.saturating_add(left),
            self.origin.y.saturating_add(top),
            self.size.width().saturating_sub(left).saturating_sub(right),
            self.size
                .height()
                .saturating_sub(top)
                .saturating_sub(bottom),
        )
    }

    /// Returns this rectangle moved so that its title bar stays reachable on `desk`.
    ///
    /// Post: the top lies on `desk` with a title bar's height below it, and at
    /// least `min(GRAB_WIDTH, width)` columns of the title bar lie on `desk`,
    /// whenever `desk` is large enough to allow it.
    pub const fn kept_on(self, desk: Rect) -> Self {
        let width = self.size.width();
        let grab = if width < GRAB_WIDTH {
            width
        } else {
            GRAB_WIDTH
        };
        let x = bounded(
            self.origin.x,
            desk.left().saturating_sub(width.saturating_sub(grab)),
            desk.right().saturating_sub(grab),
        );
        let y = bounded(
            self.origin.y,
            desk.top(),
            desk.bottom().saturating_sub(TITLE_BAR_HEIGHT),
        );
        Self {
            origin: Point::new(x, y),
            size: self.size,
        }
    }
}

/// How the desktop is laid out in a viewport.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// Windows float over the desk and are dragged by their title bars.
    Desktop,
    /// Windows stand one under another down the page, for narrow screens.
    Stacked,
}

impl Layout {
    /// Returns whether windows in this layout are moved by dragging.
    pub const fn permits_drag(self) -> bool {
        matches!(self, Layout::Desktop)
    }
}

/// The largest extent in pixels a viewport is taken to have.
const MAXIMUM_EXTENT: i32 = 1 << 20;

/// The browser's viewport.
///
/// Invariant: both extents lie in `0..=MAXIMUM_EXTENT`, so the frames derived
/// from it stay far from `i32` overflow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Viewport {
    size: Size,
}

impl Viewport {
    /// Returns a viewport `width` by `height` pixels, each extent bounded.
    pub const fn new(width: i32, height: i32) -> Self {
        Self {
            size: Size::new(
                bounded(width, 0, MAXIMUM_EXTENT),
                bounded(height, 0, MAXIMUM_EXTENT),
            ),
        }
    }

    /// Returns the desk: the viewport below the menu bar.
    pub const fn desk(self) -> Rect {
        Rect::new(
            0,
            MENU_BAR_HEIGHT,
            self.size.width(),
            self.size.height().saturating_sub(MENU_BAR_HEIGHT),
        )
    }

    /// Returns the desk left of the column of icons, where windows open.
    pub const fn window_area(self) -> Rect {
        self.desk().inset(0, 0, ICON_COLUMN_WIDTH, 0)
    }

    /// Returns the layout this viewport is drawn in.
    pub const fn layout(self) -> Layout {
        if self.size.covers(DESKTOP_MINIMUM) {
            Layout::Desktop
        } else {
            Layout::Stacked
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_floors_negative_extents_at_zero() {
        assert_eq!(Size::new(-5, 7), Size::new(0, 7));
        assert_eq!(Size::new(3, -1).height(), 0);
    }

    #[test]
    fn bounded_returns_low_for_an_empty_range() {
        assert_eq!(bounded(50, 10, 20), 20);
        assert_eq!(bounded(5, 10, 20), 10);
        assert_eq!(bounded(15, 30, 20), 30);
    }

    #[test]
    fn offset_from_inverts_translation() {
        let start = Point::new(10, 40);
        let end = Point::new(-3, 95);
        let rect = Rect::new(10, 40, 100, 80);
        assert_eq!(rect.translated(end.offset_from(start)).origin, end);
    }

    #[test]
    fn kept_on_leaves_a_window_inside_the_desk_alone() {
        let desk = Viewport::new(1200, 800).desk();
        let rect = Rect::new(100, 60, 400, 300);
        assert_eq!(rect.kept_on(desk), rect);
    }

    #[test]
    fn kept_on_keeps_the_title_bar_below_the_menu_bar() {
        let desk = Viewport::new(1200, 800).desk();
        let kept = Rect::new(100, -200, 400, 300).kept_on(desk);
        assert_eq!(kept.top(), MENU_BAR_HEIGHT);
    }

    #[test]
    fn kept_on_keeps_a_grab_width_of_title_bar_on_the_desk() {
        let desk = Viewport::new(1200, 800).desk();
        let left = Rect::new(-5000, 100, 400, 300).kept_on(desk);
        let right = Rect::new(5000, 100, 400, 300).kept_on(desk);
        assert_eq!(left.right(), GRAB_WIDTH);
        assert_eq!(right.left(), desk.right() - GRAB_WIDTH);
    }

    #[test]
    fn kept_on_keeps_the_title_bar_above_the_bottom() {
        let desk = Viewport::new(1200, 800).desk();
        let kept = Rect::new(10, 5000, 400, 300).kept_on(desk);
        assert_eq!(kept.top(), desk.bottom() - TITLE_BAR_HEIGHT);
    }

    #[test]
    fn layout_stacks_below_the_desktop_minimum() {
        assert_eq!(Viewport::new(1280, 800).layout(), Layout::Desktop);
        assert_eq!(Viewport::new(375, 812).layout(), Layout::Stacked);
        assert_eq!(Viewport::new(1280, 500).layout(), Layout::Stacked);
    }

    #[test]
    fn desk_starts_under_the_menu_bar() {
        let desk = Viewport::new(1000, 700).desk();
        assert_eq!(
            desk,
            Rect::new(0, MENU_BAR_HEIGHT, 1000, 700 - MENU_BAR_HEIGHT)
        );
    }
}
