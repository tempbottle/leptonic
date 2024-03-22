use leptos::*;

use crate::hooks::*;

#[component]
pub fn Hoverable(
    #[prop(into, optional)] disabled: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    children: ChildrenFn,
) -> impl IntoView {
    let UseHoverReturn {
        props,
        is_hovered: _,
    } = use_hover(UseHoverInput {
        disabled: disabled.unwrap_or(false.into()),
        on_hover_start,
        on_hover_end,
    });

    children().into_view().directive(handlers, props.handlers)
}
