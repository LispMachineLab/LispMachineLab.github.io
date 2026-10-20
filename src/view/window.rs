//! A Platinum window: its outline, title bar, boxes and frame about its content.

use yew::prelude::*;

use crate::effects;
use crate::model::chrome::{BOX_SIZE, BoxKind, grey_hex, title_box};
use crate::model::desktop::Event;
use crate::model::geometry::Layout;
use crate::model::window::{WindowId, WindowState};

use super::app::{Desk, place};

/// What a window's content is drawn on.
const fn surface(id: WindowId) -> &'static str {
    match id {
        WindowId::About | WindowId::Agent | WindowId::Desktop | WindowId::Browser => "panel",
        WindowId::ReadMe | WindowId::Trash => "document",
    }
}

/// The properties of a [`TitleBox`].
#[derive(Properties, PartialEq)]
pub struct TitleBoxProps {
    /// Which box.
    pub kind: BoxKind,
    /// What it does, for assistive technology.
    pub label: &'static str,
    /// Sent when the box is let go of over it.
    pub on_press: Callback<()>,
}

/// A title bar box. It sinks while held and acts when let go over it, as
/// Mac OS 9's did: the click, which only comes when pressed and released
/// over the same box, is what acts.
#[function_component(TitleBox)]
pub fn title_box_view(props: &TitleBoxProps) -> Html {
    let pressed = use_state_eq(|| false);
    let press = {
        let pressed = pressed.clone();
        Callback::from(move |event: PointerEvent| {
            event.stop_propagation();
            pressed.set(effects::is_primary(&event));
        })
    };
    let release = {
        let pressed = pressed.clone();
        Callback::from(move |_: PointerEvent| pressed.set(false))
    };
    let click = props.on_press.reform(|_: MouseEvent| ());
    let keep_shade = Callback::from(|event: MouseEvent| event.stop_propagation());
    let kind = match props.kind {
        BoxKind::Close => "close",
        BoxKind::Zoom => "zoom",
        BoxKind::Collapse => "collapse",
    };
    let size = BOX_SIZE.to_string();
    let runs = title_box(props.kind, *pressed);
    html! {
        <button class={classes!("title-box", kind)} type="button" aria-label={props.label}
                onpointerdown={press} onpointerup={release.clone()} onpointerleave={release.clone()}
                onpointercancel={release} onclick={click} ondblclick={keep_shade}>
            <svg width={size.clone()} height={size.clone()} viewBox={format!("0 0 {size} {size}")}
                 shape-rendering="crispEdges" aria-hidden="true">
                { for runs.into_iter().map(|run| html! {
                    <rect x={run.x.to_string()} y={run.y.to_string()}
                          width={run.width.to_string()} height={run.height.to_string()}
                          fill={grey_hex(run.grey)} />
                }) }
            </svg>
        </button>
    }
}

/// The properties of a [`PlatinumWindow`].
#[derive(Properties, PartialEq)]
pub struct WindowProps {
    /// Which window.
    pub id: WindowId,
    /// Its frame, shade and zoom.
    pub state: WindowState,
    /// Whether it is the window in front, its title bar striped and boxed.
    pub active: bool,
    /// What it holds.
    pub children: Html,
}

/// A window of the desk, or of the page when stacked.
#[function_component(PlatinumWindow)]
pub fn platinum_window(props: &WindowProps) -> Html {
    let Some(desk) = use_context::<Desk>() else {
        return Html::default();
    };
    let id = props.id;
    let stacked = matches!(desk.layout, Layout::Stacked);
    let rolled = props.state.is_rolled_up();
    let title_id = format!("{}-title", id.element_id());
    let style = if stacked {
        String::new()
    } else {
        place(props.state.frame(), rolled)
    };
    // Pressing any window closes the menu open, pressing one behind raises
    // it, and the desk under it is not pressed.
    let raise = {
        let on_event = desk.on_event.clone();
        Callback::from(move |event: PointerEvent| {
            event.stop_propagation();
            on_event.emit(Event::Raise(id));
        })
    };
    html! {
        <section id={id.element_id()} aria-labelledby={title_id.clone()} {style}
                 class={classes!("window", props.active.then_some("active"), rolled.then_some("rolled"))}
                 onpointerdown={raise}>
            { title_bar(&desk, id, props.active, stacked, &title_id) }
            if !rolled {
                <div class="window-body">
                    <div class={classes!("window-content", surface(id))}>
                        { props.children.clone() }
                    </div>
                </div>
            }
        </section>
    }
}

/// Returns a window's title bar: boxes and ridges when active, the title between.
fn title_bar(desk: &Desk, id: WindowId, active: bool, stacked: bool, title_id: &str) -> Html {
    let on_event = desk.on_event.clone();
    let begin = (!stacked).then(|| {
        let on_event = on_event.clone();
        Callback::from(move |event: PointerEvent| {
            if effects::is_primary(&event) {
                effects::capture_pointer(&event);
                on_event.emit(Event::BeginDrag {
                    window: id,
                    at: effects::pointer(&event),
                });
            }
        })
    });
    let follow = (!stacked).then(|| {
        let on_event = on_event.clone();
        Callback::from(move |event: PointerEvent| {
            on_event.emit(Event::DragTo(effects::pointer(&event)))
        })
    });
    let boxed = |kind: BoxKind, label: &'static str, event: Event| {
        html! { <TitleBox {kind} {label} on_press={desk.emits::<()>(event)} /> }
    };
    html! {
        <div class="title-bar" onpointerdown={begin} onpointermove={follow}
             onpointerup={desk.emits::<PointerEvent>(Event::EndDrag)}
             onpointercancel={desk.emits::<PointerEvent>(Event::Escape)}
             ondblclick={desk.emits::<MouseEvent>(Event::ToggleShade(id))}>
            if active && !stacked { { boxed(BoxKind::Close, "Close", Event::Close(id)) } }
            if active { <span class="ridges" aria-hidden="true" /> }
            <h2 class="title" id={title_id.to_owned()}>{ id.title() }</h2>
            if active { <span class="ridges" aria-hidden="true" /> }
            if active && !stacked { { boxed(BoxKind::Zoom, "Zoom", Event::ToggleZoom(id)) } }
            if active { { boxed(BoxKind::Collapse, "Collapse", Event::ToggleShade(id)) } }
        </div>
    }
}
