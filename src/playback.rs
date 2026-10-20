//! A looping clip's playback: waiting to be seen, playing, refused by the
//! browser and offering a play button, or showing the browser's controls.
//! Pure, and tested natively; `view::media` drives it with the browser's
//! answers, which `effects` gets.

/// Why a video did not start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayError {
    /// The browser refused: it saves power or data, or waits for a gesture.
    Refused,
}

/// What happens to a looping clip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// The motion the visitor prefers is known: less of it when `reduced`.
    Motion {
        /// Whether less motion was asked for.
        reduced: bool,
    },
    /// It came into sight and was started on its own, with this result.
    Played(Result<(), PlayError>),
    /// Its play button was clicked, with this result.
    Clicked(Result<(), PlayError>),
}

/// What a looping clip shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Playback {
    /// Its poster, until it comes into sight: as prerender draws it.
    #[default]
    Waiting,
    /// Playing, muted and looped, while in sight.
    Playing,
    /// The browser would not start it on its own: its poster and a play
    /// button, so that it reads as a video and a click starts it.
    Refused,
    /// Its poster and the browser's controls: with less motion asked for,
    /// or when even a click did not start it.
    Controls,
}

impl Playback {
    /// Returns what the clip shows after `event`.
    ///
    /// Post: a clip that starts plays; one the browser refuses offers its
    /// play button, unless it shows its controls already, which stay.
    pub const fn next(self, event: Event) -> Self {
        match (self, event) {
            (_, Event::Motion { reduced: true }) => Playback::Controls,
            (playback, Event::Motion { reduced: false }) => playback,
            (_, Event::Played(Ok(())) | Event::Clicked(Ok(()))) => Playback::Playing,
            (Playback::Controls, Event::Played(Err(_))) => Playback::Controls,
            (_, Event::Played(Err(_))) => Playback::Refused,
            (_, Event::Clicked(Err(_))) => Playback::Controls,
        }
    }

    /// Returns whether the browser's controls show.
    pub const fn controls(self) -> bool {
        matches!(self, Playback::Controls)
    }

    /// Returns whether the play button shows.
    pub const fn offers_play(self) -> bool {
        matches!(self, Playback::Refused)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Playback; 4] = [
        Playback::Waiting,
        Playback::Playing,
        Playback::Refused,
        Playback::Controls,
    ];

    const REFUSED: Result<(), PlayError> = Err(PlayError::Refused);

    #[test]
    fn a_clip_waits_on_its_poster_until_it_is_seen() {
        let waiting = Playback::default().next(Event::Motion { reduced: false });
        assert_eq!(waiting, Playback::Waiting);
        assert!(!waiting.controls());
        assert!(!waiting.offers_play());
    }

    #[test]
    fn with_less_motion_a_clip_shows_its_controls() {
        for playback in ALL {
            let still = playback.next(Event::Motion { reduced: true });
            assert_eq!(still, Playback::Controls, "{playback:?}");
            assert!(still.controls());
        }
    }

    #[test]
    fn a_clip_the_browser_refuses_offers_a_play_button() {
        for playback in [Playback::Waiting, Playback::Playing, Playback::Refused] {
            let refused = playback.next(Event::Played(REFUSED));
            assert_eq!(refused, Playback::Refused, "{playback:?}");
            assert!(refused.offers_play());
            assert!(!refused.controls());
        }
    }

    #[test]
    fn a_clip_that_starts_plays_with_nothing_over_it() {
        for playback in ALL {
            for event in [Event::Played(Ok(())), Event::Clicked(Ok(()))] {
                let playing = playback.next(event);
                assert_eq!(playing, Playback::Playing, "{playback:?} {event:?}");
                assert!(!playing.controls());
                assert!(!playing.offers_play());
            }
        }
    }

    #[test]
    fn a_click_refused_too_leaves_the_browser_s_controls() {
        let after = Playback::Refused.next(Event::Clicked(REFUSED));
        assert_eq!(after, Playback::Controls);
        assert!(after.controls());
    }

    #[test]
    fn controls_shown_stay_when_the_browser_refuses_again() {
        assert_eq!(
            Playback::Controls.next(Event::Played(REFUSED)),
            Playback::Controls
        );
    }

    #[test]
    fn a_clip_never_shows_both_a_play_button_and_controls() {
        for playback in ALL {
            assert!(
                !(playback.controls() && playback.offers_play()),
                "{playback:?}"
            );
        }
    }
}
