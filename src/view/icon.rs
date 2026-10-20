//! An icon on the desk: a click selects it, a double-click or Return opens it,
//! and in the stacked layout a tap opens it.

use yew::prelude::*;

use crate::model::command::IconId;
use crate::model::desktop::Event;
use crate::model::geometry::Layout;

use super::app::Desk;

/// The properties of a [`DeskIcon`].
#[derive(Properties, PartialEq)]
pub struct IconProps {
    /// Which icon.
    pub icon: IconId,
    /// Whether it is selected, darkened and its name inverted.
    pub selected: bool,
}

/// An icon with its name under it.
#[function_component(DeskIcon)]
pub fn desk_icon(props: &IconProps) -> Html {
    let Some(desk) = use_context::<Desk>() else {
        return Html::default();
    };
    let icon = props.icon;
    let open = {
        let desk = desk.clone();
        move || desk.perform(icon.action().into())
    };
    let select = {
        let desk = desk.clone();
        Callback::from(move |event: PointerEvent| {
            event.stop_propagation();
            desk.on_event.emit(Event::Select(icon));
        })
    };
    let tap = {
        let open = open.clone();
        let stacked = matches!(desk.layout, Layout::Stacked);
        Callback::from(move |_: MouseEvent| {
            if stacked {
                open();
            }
        })
    };
    let double = {
        let open = open.clone();
        Callback::from(move |_: MouseEvent| open())
    };
    let key = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            // Return opens; the click a button would also get is held back.
            event.prevent_default();
            open();
        }
    });
    html! {
        <button type="button" onpointerdown={select} onclick={tap} ondblclick={double} onkeydown={key}
                class={classes!("icon", props.selected.then_some("selected"), icon.is_small().then_some("small"))}>
            <img src={icon.image()} alt="" width="32" height="32" draggable="false" />
            <span class="label">{ icon.label() }</span>
        </button>
    }
}
