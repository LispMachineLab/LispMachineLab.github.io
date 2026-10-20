//! What the desktop's icons and menus do: open a window, or visit a page
//! elsewhere.

use super::window::WindowId;

/// A page on GitHub the site links to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    /// The LispMachineLab organization.
    Organization,
    /// The lispm-agent repository.
    AgentRepository,
    /// The lispm-desktop repository.
    DesktopRepository,
    /// The lispm-browser repository.
    BrowserRepository,
}

impl Link {
    /// Returns the page's address.
    pub const fn url(self) -> &'static str {
        match self {
            Link::Organization => "https://github.com/LispMachineLab",
            Link::AgentRepository => "https://github.com/LispMachineLab/lispm-agent",
            Link::DesktopRepository => "https://github.com/LispMachineLab/lispm-desktop",
            Link::BrowserRepository => "https://github.com/LispMachineLab/lispm-browser",
        }
    }
}

/// What opening an icon or choosing a menu item does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Opens a window, or brings it to the front.
    Open(WindowId),
    /// Visits a page elsewhere, in a new tab.
    Visit(Link),
}

/// An icon on the desk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconId {
    /// Opens LispM Agent's window.
    Agent,
    /// Opens LispM Desktop's window.
    Desktop,
    /// Opens LispM Browser's window.
    Browser,
    /// Opens the Read Me.
    ReadMe,
    /// Visits the organization on GitHub.
    GitHub,
    /// Opens the Trash.
    Trash,
}

impl IconId {
    /// Every icon, in the order they stand down the desk's right edge.
    pub const ALL: [IconId; 6] = [
        IconId::Agent,
        IconId::Desktop,
        IconId::Browser,
        IconId::ReadMe,
        IconId::GitHub,
        IconId::Trash,
    ];

    /// Returns the name under the icon.
    pub const fn label(self) -> &'static str {
        match self {
            IconId::Agent => "LispM Agent",
            IconId::Desktop => "LispM Desktop",
            IconId::Browser => "LispM Browser",
            IconId::ReadMe => "Read Me",
            IconId::GitHub => "LispMachineLab",
            IconId::Trash => "Trash",
        }
    }

    /// Returns the address of the icon's picture.
    pub const fn image(self) -> &'static str {
        match self {
            IconId::Agent => "assets/icons/robot.svg",
            IconId::Desktop => "assets/icons/computer.svg",
            IconId::Browser => "assets/icons/netscape.svg",
            IconId::ReadMe => "assets/icons/note.svg",
            IconId::GitHub => "assets/icons/folder-git-32.svg",
            IconId::Trash => "assets/icons/trash-32.svg",
        }
    }

    /// Returns whether the picture is drawn at 16 pixels and shown doubled.
    pub const fn is_small(self) -> bool {
        matches!(self, IconId::Agent | IconId::Desktop)
    }

    /// Returns whether the icon stands in the desk's bottom corner, as the Trash did.
    pub const fn is_cornered(self) -> bool {
        matches!(self, IconId::Trash)
    }

    /// Returns what opening the icon does.
    pub const fn action(self) -> Action {
        match self {
            IconId::Agent => Action::Open(WindowId::Agent),
            IconId::Desktop => Action::Open(WindowId::Desktop),
            IconId::Browser => Action::Open(WindowId::Browser),
            IconId::ReadMe => Action::Open(WindowId::ReadMe),
            IconId::GitHub => Action::Visit(Link::Organization),
            IconId::Trash => Action::Open(WindowId::Trash),
        }
    }
}

/// What a menu item asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    /// Does an action.
    Do(Action),
    /// Opens the icon selected, as File › Open did.
    OpenSelection,
    /// Closes the window in front.
    CloseFront,
}

/// An entry of a menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuEntry {
    /// An item to choose.
    Item {
        /// Its text.
        label: &'static str,
        /// What choosing it asks for.
        command: Command,
    },
    /// An engraved line between groups of items.
    Separator,
}

/// Returns the item reading `label` that asks for `command`.
const fn item(label: &'static str, command: Command) -> MenuEntry {
    MenuEntry::Item { label, command }
}

/// Returns the item reading `label` that opens `window`.
const fn opens(label: &'static str, window: WindowId) -> MenuEntry {
    item(label, Command::Do(Action::Open(window)))
}

/// Returns the item reading `label` that visits `link`.
const fn visits(label: &'static str, link: Link) -> MenuEntry {
    item(label, Command::Do(Action::Visit(link)))
}

const LAMBDA_MENU: [MenuEntry; 5] = [
    opens("About LispM", WindowId::About),
    MenuEntry::Separator,
    opens("LispM Agent", WindowId::Agent),
    opens("LispM Desktop", WindowId::Desktop),
    opens("LispM Browser", WindowId::Browser),
];

const FILE_MENU: [MenuEntry; 2] = [
    item("Open", Command::OpenSelection),
    item("Close Window", Command::CloseFront),
];

const GO_MENU: [MenuEntry; 5] = [
    visits("LispMachineLab on GitHub", Link::Organization),
    MenuEntry::Separator,
    visits("lispm-agent", Link::AgentRepository),
    visits("lispm-desktop", Link::DesktopRepository),
    visits("lispm-browser", Link::BrowserRepository),
];

const HELP_MENU: [MenuEntry; 1] = [opens("Read Me", WindowId::ReadMe)];

/// A menu of the menu bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuId {
    /// The rainbow λ's menu, as Mac OS 9's Apple menu.
    Lambda,
    /// File.
    File,
    /// Go.
    Go,
    /// Help.
    Help,
}

impl MenuId {
    /// Every menu, left to right.
    pub const ALL: [MenuId; 4] = [MenuId::Lambda, MenuId::File, MenuId::Go, MenuId::Help];

    /// Returns the menu's title; the λ menu's is the rainbow λ, a picture.
    pub const fn title(self) -> &'static str {
        match self {
            MenuId::Lambda => "LispM",
            MenuId::File => "File",
            MenuId::Go => "Go",
            MenuId::Help => "Help",
        }
    }

    /// Returns the menu's entries, top to bottom.
    pub const fn entries(self) -> &'static [MenuEntry] {
        match self {
            MenuId::Lambda => &LAMBDA_MENU,
            MenuId::File => &FILE_MENU,
            MenuId::Go => &GO_MENU,
            MenuId::Help => &HELP_MENU,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_project_window_has_an_icon() {
        for window in [
            WindowId::Agent,
            WindowId::Desktop,
            WindowId::Browser,
            WindowId::ReadMe,
        ] {
            assert!(
                IconId::ALL
                    .iter()
                    .any(|icon| icon.action() == Action::Open(window))
            );
        }
    }

    #[test]
    fn only_the_trash_stands_in_the_corner() {
        let cornered: Vec<IconId> = IconId::ALL
            .into_iter()
            .filter(|i| i.is_cornered())
            .collect();
        assert_eq!(cornered, vec![IconId::Trash]);
    }

    #[test]
    fn links_go_to_the_organization() {
        for link in [
            Link::Organization,
            Link::AgentRepository,
            Link::DesktopRepository,
            Link::BrowserRepository,
        ] {
            assert!(link.url().starts_with("https://github.com/LispMachineLab"));
        }
    }

    #[test]
    fn no_menu_starts_or_ends_with_a_separator() {
        for menu in MenuId::ALL {
            let entries = menu.entries();
            assert_ne!(entries.first(), Some(&MenuEntry::Separator), "{menu:?}");
            assert_ne!(entries.last(), Some(&MenuEntry::Separator), "{menu:?}");
        }
    }
}
