//! Platinum's title bar boxes as pixel maps, ported from LispM Desktop's
//! `lispm-desktop-platinum-draw-box`, so they are drawn as Mac OS 9 drew them.

/// Side in pixels of a title bar box.
pub const BOX_SIZE: u8 = 13;

/// A box of a window's title bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoxKind {
    /// The close box, at the left end.
    Close,
    /// The zoom box, a square sharing the frame's top left corner.
    Zoom,
    /// The collapse box, two lines across it.
    Collapse,
}

/// A rectangle of pixels in one of Platinum's greys, inside a box.
///
/// Invariant: `x + width <= BOX_SIZE` and `y + height <= BOX_SIZE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Run {
    /// Leftmost column.
    pub x: u8,
    /// Top row.
    pub y: u8,
    /// Columns covered.
    pub width: u8,
    /// Rows covered.
    pub height: u8,
    /// The grey, from 0x00 black to 0xFF white.
    pub grey: u8,
}

/// Returns the run along row `y` from column `from` to `to`, both in.
const fn row(from: u8, to: u8, y: u8, grey: u8) -> Run {
    Run {
        x: from,
        y,
        width: to.saturating_sub(from).saturating_add(1),
        height: 1,
        grey,
    }
}

/// Returns the run down column `x` from row `from` to `to`, both in.
const fn column(x: u8, from: u8, to: u8, grey: u8) -> Run {
    Run {
        x,
        y: from,
        width: 1,
        height: to.saturating_sub(from).saturating_add(1),
        grey,
    }
}

/// Returns the single pixel at `x`, `y`.
const fn dot(x: u8, y: u8, grey: u8) -> Run {
    row(x, x, y, grey)
}

/// The near black boxes are framed and marked in.
const INK: u8 = 0x22;

/// The square's greys along its diagonal, top left to bottom right.
const RAMP: [u8; 7] = [0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

/// The same square held down.
const PRESSED_RAMP: [u8; 7] = [0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB];

/// Returns the runs of a box of `kind`, painted in order, later runs over earlier.
///
/// The box is a recessed well round a dark frame, inside it a bevel round a
/// square shaded diagonally from grey to white; `pressed` darkens the square
/// and turns the bevel over.
pub fn title_box(kind: BoxKind, pressed: bool) -> Vec<Run> {
    let (lit, shaded, corner) = if pressed {
        (0x88, 0xCC, 0x77)
    } else {
        (0xCC, 0x88, 0xFF)
    };
    let ramp = if pressed { PRESSED_RAMP } else { RAMP };
    let mut runs = vec![
        // The well.
        row(0, 11, 0, 0x88),
        column(0, 0, 11, 0x88),
        row(1, 12, 12, 0xFF),
        column(12, 1, 12, 0xFF),
        // The frame.
        row(1, 11, 1, INK),
        row(1, 11, 11, INK),
        column(1, 1, 11, INK),
        column(11, 1, 11, INK),
        // The bevel, lit from the top left, or from below when pressed.
        row(2, 10, 2, lit),
        column(2, 3, 10, lit),
        dot(2, 2, corner),
        column(10, 3, 10, shaded),
        row(3, 10, 10, shaded),
    ];
    runs.extend(square(ramp));
    runs.extend(glyph(kind));
    runs
}

/// Returns the 7 by 7 square at 3, 3, each pixel the grey of `ramp` its diagonal stands at.
fn square(ramp: [u8; 7]) -> impl Iterator<Item = Run> {
    (0u8..7).flat_map(move |j| {
        (0u8..7).filter_map(move |i| {
            let step = usize::from((i + j) / 2);
            ramp.get(step).map(|&grey| dot(3 + i, 3 + j, grey))
        })
    })
}

/// Returns the runs marking a box of `kind` over its square.
fn glyph(kind: BoxKind) -> Vec<Run> {
    match kind {
        BoxKind::Close => Vec::new(),
        BoxKind::Zoom => vec![column(7, 1, 7, INK), row(1, 7, 7, INK)],
        BoxKind::Collapse => vec![row(1, 11, 5, INK), row(1, 11, 7, INK)],
    }
}

/// Returns `grey` as a CSS colour.
pub fn grey_hex(grey: u8) -> String {
    format!("#{grey:02X}{grey:02X}{grey:02X}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const KINDS: [BoxKind; 3] = [BoxKind::Close, BoxKind::Zoom, BoxKind::Collapse];

    /// Returns the grey painted last at `x`, `y`, if any run covers it.
    fn pixel(runs: &[Run], x: u8, y: u8) -> Option<u8> {
        runs.iter()
            .rev()
            .find(|r| (r.x..r.x + r.width).contains(&x) && (r.y..r.y + r.height).contains(&y))
            .map(|r| r.grey)
    }

    #[test]
    fn every_run_lies_inside_the_box() {
        for kind in KINDS {
            for pressed in [false, true] {
                for run in title_box(kind, pressed) {
                    assert!(run.width > 0 && run.height > 0, "{run:?}");
                    assert!(run.x + run.width <= BOX_SIZE, "{kind:?} {run:?}");
                    assert!(run.y + run.height <= BOX_SIZE, "{kind:?} {run:?}");
                }
            }
        }
    }

    #[test]
    fn the_square_shades_from_grey_to_white() {
        let runs = title_box(BoxKind::Close, false);
        assert_eq!(pixel(&runs, 3, 3), Some(0x99));
        assert_eq!(pixel(&runs, 9, 9), Some(0xFF));
    }

    #[test]
    fn pressing_darkens_the_square() {
        let up = title_box(BoxKind::Close, false);
        let down = title_box(BoxKind::Close, true);
        assert!(pixel(&down, 6, 6) < pixel(&up, 6, 6));
    }

    #[test]
    fn glyphs_mark_the_square_in_ink() {
        let zoom = title_box(BoxKind::Zoom, false);
        let collapse = title_box(BoxKind::Collapse, false);
        let close = title_box(BoxKind::Close, false);
        assert_eq!(pixel(&zoom, 7, 4), Some(INK));
        assert_eq!(pixel(&collapse, 6, 5), Some(INK));
        assert_ne!(pixel(&close, 7, 4), Some(INK));
        assert_ne!(pixel(&close, 6, 5), Some(INK));
    }

    #[test]
    fn the_well_leaves_two_corners_to_the_title_bar() {
        let runs = title_box(BoxKind::Close, false);
        assert_eq!(pixel(&runs, 12, 0), None);
        assert_eq!(pixel(&runs, 0, 12), None);
    }

    #[test]
    fn grey_hex_repeats_the_level() {
        assert_eq!(grey_hex(0x22), "#222222");
        assert_eq!(grey_hex(0xFF), "#FFFFFF");
    }
}
