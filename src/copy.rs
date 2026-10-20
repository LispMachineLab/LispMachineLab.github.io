//! A snippet's copy button: ready, then copied or failed, then ready again.

/// Why the clipboard did not take a snippet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyError {
    /// The page has no clipboard: it is not served securely, or the browser has none.
    Unavailable,
    /// The browser refused to write it.
    Refused,
}

/// What a copy button shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CopyState {
    /// Waiting to be pressed.
    #[default]
    Ready,
    /// The snippet is on the clipboard.
    Copied,
    /// The clipboard refused it; the visitor selects the code instead.
    Failed,
}

impl CopyState {
    /// Returns the state a copy settles in.
    pub const fn settled(result: Result<(), CopyError>) -> Self {
        match result {
            Ok(()) => CopyState::Copied,
            Err(_) => CopyState::Failed,
        }
    }

    /// Returns the button's text.
    pub const fn label(self) -> &'static str {
        match self {
            CopyState::Ready => "Copy",
            CopyState::Copied => "Copied",
            CopyState::Failed => "Select to copy",
        }
    }

    /// Returns whether the button shows a result, and goes back to ready after a while.
    pub const fn is_settled(self) -> bool {
        !matches!(self, CopyState::Ready)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_copy_settles_by_its_result() {
        assert_eq!(CopyState::settled(Ok(())), CopyState::Copied);
        assert_eq!(
            CopyState::settled(Err(CopyError::Refused)),
            CopyState::Failed
        );
        assert_eq!(
            CopyState::settled(Err(CopyError::Unavailable)),
            CopyState::Failed
        );
    }

    #[test]
    fn only_ready_is_unsettled() {
        assert!(!CopyState::Ready.is_settled());
        assert!(CopyState::Copied.is_settled());
        assert!(CopyState::Failed.is_settled());
    }

    #[test]
    fn each_state_reads_differently() {
        let labels = [CopyState::Ready, CopyState::Copied, CopyState::Failed].map(CopyState::label);
        assert_ne!(labels[0], labels[1]);
        assert_ne!(labels[1], labels[2]);
        assert_ne!(labels[0], labels[2]);
    }
}
