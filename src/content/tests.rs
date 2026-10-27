//! The page's data, checked against itself and the assets.

use super::*;
use std::path::Path;

/// Every section, top to bottom.
const ALL: [Anchor; 9] = [
    Anchor::Film,
    Anchor::Idea,
    Anchor::Agents,
    Anchor::Desktop,
    Anchor::Store,
    Anchor::Remote,
    Anchor::Trust,
    Anchor::Start,
    Anchor::Questions,
];

fn clips() -> impl Iterator<Item = Clip> {
    std::iter::once(FILM).chain(illustrations().filter_map(|i| i.media.clip()))
}

fn pictures() -> impl Iterator<Item = Picture> {
    illustrations().filter_map(|i| match i.media {
        Media::Picture(picture) => Some(picture),
        Media::Loop(_) => None,
    })
}

#[test]
fn illustrations_are_every_stage_then_every_spotlight_clip() {
    let mut told = illustrations();
    for chapter in CHAPTERS {
        for stage in chapter.stages {
            assert_eq!(
                told.next(),
                Some(Illustration {
                    headline: stage.headline,
                    media: stage.media,
                })
            );
        }
        for spotlight in chapter.spotlights {
            if let Some(clip) = spotlight.clip {
                assert_eq!(
                    told.next(),
                    Some(Illustration {
                        headline: spotlight.headline,
                        media: Media::Loop(clip),
                    })
                );
            }
        }
    }
    assert_eq!(told.next(), None);
}

#[test]
fn every_clip_was_published_once_the_page_opened() {
    for clip in clips() {
        assert!(clip.published >= OPENED, "{}", clip.video);
    }
}

#[test]
fn anchors_have_distinct_ids() {
    for (i, a) in ALL.iter().enumerate() {
        for b in ALL.iter().skip(i + 1) {
            assert_ne!(a.id(), b.id());
        }
    }
}

#[test]
fn every_chapter_has_its_own_section() {
    for (i, a) in CHAPTERS.iter().enumerate() {
        assert!(ALL.contains(&a.anchor), "{}", a.eyebrow);
        for b in CHAPTERS.iter().skip(i + 1) {
            assert_ne!(a.anchor, b.anchor);
        }
    }
}

#[test]
fn every_clip_and_poster_is_in_the_assets() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for clip in clips() {
        assert!(
            clip.video.ends_with(".mp4") && root.join(clip.video).is_file(),
            "{}",
            clip.video
        );
        assert!(
            clip.poster.ends_with(".jpg") && root.join(clip.poster).is_file(),
            "{}",
            clip.poster
        );
        assert!(!clip.description.is_empty());
    }
}

#[test]
fn every_picture_is_in_the_assets_at_its_size() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut seen = 0;
    for picture in pictures() {
        let bytes = std::fs::read(root.join(picture.image)).unwrap_or_default();
        assert_eq!(
            crate::blog::size::webp(&bytes),
            Some((picture.width, picture.height)),
            "{}",
            picture.image
        );
        assert!(!picture.description.is_empty());
        seen += 1;
    }
    assert!(seen > 0);
}

#[test]
fn every_clip_is_told_once() {
    let all = clips().map(|clip| clip.video).collect::<Vec<_>>();
    for (i, a) in all.iter().enumerate() {
        assert!(!all.iter().skip(i + 1).any(|b| b == a), "{a}");
    }
}

#[test]
fn links_stay_in_the_organization() {
    for link in PROJECTS
        .iter()
        .map(|p| p.repository)
        .chain(CHAPTERS.iter().map(|c| c.repository))
        .chain([ORGANIZATION, BACKENDS])
    {
        assert!(link.starts_with(ORGANIZATION), "{link}");
    }
}

#[test]
fn snippets_install_from_their_own_repository() {
    for project in PROJECTS {
        if let Some(install) = project.install {
            assert!(
                install.snippet.code.contains(project.repository),
                "{}",
                project.name
            );
            assert!(!install.snippet.code.ends_with('\n'), "{}", project.name);
        }
    }
}

#[test]
fn the_projects_on_their_own_come_first() {
    let first_without = PROJECTS
        .iter()
        .position(|p| p.install.is_none())
        .unwrap_or(PROJECTS.len());
    assert!(
        PROJECTS
            .iter()
            .skip(first_without)
            .all(|p| p.install.is_none())
    );
    assert!(first_without > 0);
}

#[test]
fn every_chapter_makes_its_points() {
    for chapter in CHAPTERS {
        assert!(!chapter.points.is_empty(), "{}", chapter.eyebrow);
    }
}

#[test]
fn the_experimental_lines_are_the_install_line_choosing_an_emacs() {
    for (emacs, line) in [("32", INSTALL_EXPERIMENTAL.code), ("31", INSTALL_BACK)] {
        assert_eq!(
            line,
            INSTALL
                .code
                .replace("| sh", &format!("| LISPM_EMACS={emacs} sh"))
        );
    }
}

#[test]
fn the_install_line_runs_the_installer_served_here() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(INSTALL.code, format!("curl -fsSL {SITE}{INSTALLER} | sh"));
    let served = std::fs::read_to_string(root.join("static").join(INSTALLER)).unwrap_or_default();
    assert!(served.starts_with("#!/bin/sh\n"));
    let source = INSTALLER_SOURCE
        .replace("github.com", "raw.githubusercontent.com")
        .replace("/blob/main/", "/${release:-main}/");
    assert!(served.contains(&source), "{source}");
    assert!(
        served
            .contains("https://api.github.com/repos/LispMachineLab/lispm-desktop/releases/latest")
    );
    let index = std::fs::read_to_string(root.join("index.html")).unwrap_or_default();
    assert!(index.contains(&format!("rel=\"copy-file\" href=\"static/{INSTALLER}\"")));
    assert!(INSTALLER_SOURCE.starts_with(ORGANIZATION));
}

#[test]
fn the_steps_hold_the_install_line_once() {
    let lines = DOWNLOAD_STEPS
        .iter()
        .filter_map(|step| step.snippet)
        .collect::<Vec<_>>();
    assert_eq!(lines, [INSTALL]);
    assert!(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(APP_ICON)
            .is_file()
    );
}

#[test]
fn hrefs_point_at_ids() {
    assert_eq!(Anchor::Start.href(), "#start");
    assert_eq!(Anchor::Agents.href(), "#agent");
}

#[test]
fn everywhere_lispm_is_told_it_is_built_on_emacs_and_the_lisp_machine() {
    for text in [DESCRIPTION, DEFINITION, IDEA] {
        assert!(
            text.contains("Emacs") && text.contains("Lisp Machine"),
            "{text}"
        );
    }
    let first = QUESTIONS.first().map(|question| question.answer);
    assert!(first.is_some_and(|answer| answer.starts_with(DEFINITION)));
    for question in QUESTIONS {
        assert!(question.question.ends_with('?'), "{}", question.question);
        assert!(!question.answer.is_empty());
    }
}

#[test]
fn the_title_and_description_fit_where_search_engines_show_them() {
    assert!(TITLE.chars().count() <= 60, "{TITLE}");
    assert!(DESCRIPTION.chars().count() <= 160, "{DESCRIPTION}");
    assert!(REQUIREMENTS.starts_with(OPERATING_SYSTEM));
}
