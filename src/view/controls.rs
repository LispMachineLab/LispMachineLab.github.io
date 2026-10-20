//! Platinum's controls inside windows: push buttons, group boxes, text fields
//! holding code, and buttons opening other windows.

use yew::prelude::*;

use crate::model::command::{Action, Link};
use crate::model::window::WindowId;

use super::app::Desk;

/// Returns `button` ringed if it is the `default` one, as Mac OS 9 ringed
/// the push button Return pressed.
fn ringed(default: bool, button: Html) -> Html {
    if default {
        html! { <span class="default-ring">{ button }</span> }
    } else {
        button
    }
}

/// Returns a push button visiting `link` in a new tab.
pub fn push_link(label: &'static str, link: Link, default: bool) -> Html {
    ringed(
        default,
        html! { <a class="push-button" href={link.url()} target="_blank" rel="noopener">{ label }</a> },
    )
}

/// Returns a group box titled `title` round `children`.
pub fn group(title: &'static str, children: Html) -> Html {
    html! {
        <fieldset class="group">
            <legend>{ title }</legend>
            { children }
        </fieldset>
    }
}

/// Returns `text` as code, in an inset text field.
pub fn code(text: &'static str) -> Html {
    html! { <pre class="field"><code>{ text }</code></pre> }
}

/// The properties of an [`Opens`] button.
#[derive(Properties, PartialEq)]
pub struct OpensProps {
    /// The window opened.
    pub window: WindowId,
    /// The button's face.
    pub children: Html,
    /// Draws it as a push button rather than as a link.
    #[prop_or_default]
    pub push: bool,
    /// Rings it as the default push button.
    #[prop_or_default]
    pub default: bool,
}

/// A button opening a window, or bringing it to the front.
#[function_component(Opens)]
pub fn opens(props: &OpensProps) -> Html {
    let Some(desk) = use_context::<Desk>() else {
        return Html::default();
    };
    let window = props.window;
    let open = Callback::from(move |_: MouseEvent| desk.perform(Action::Open(window).into()));
    let class = if props.push { "push-button" } else { "opens" };
    ringed(
        props.default,
        html! { <button type="button" {class} onclick={open}>{ props.children.clone() }</button> },
    )
}
