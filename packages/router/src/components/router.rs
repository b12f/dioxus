use dioxus_core::Element;

use dioxus_core as dioxus;
use dioxus_core::prelude::*;
use dioxus_core_macro::*;
use dioxus_html as dioxus_elements;

use crate::RouterService;

#[derive(Props)]
pub struct RouterProps<'a> {
    children: Element<'a>,

    #[props(default, strip_option)]
    onchange: Option<&'a Fn(&'a str)>,
}

#[allow(non_snake_case)]
pub fn Router<'a>(cx: Scope<'a, RouterProps<'a>>) -> Element {
    cx.use_hook(|_| {
        let update = cx.schedule_update_any();
        cx.provide_context(RouterService::new(update, cx.scope_id()))
    });

    cx.render(rsx!(
        div { &cx.props.children }
    ))
}
