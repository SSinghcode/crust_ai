use icons::{History, MessageSquare};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::hooks::use_is_current_path::use_is_current_path;
use crate::components::ui::bottom_nav::{
    BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel,
};
use crate::domain::home::routes::HomeRoutes;

#[component]
pub fn AppBottomNav() -> impl IntoView {
    let navigate = use_navigate();
    let is_current_path = use_is_current_path();

    let nav_items: [(&'static str, &'static str, AnyView); 2] = [
        (HomeRoutes::base_url(), "Chat", view! { <MessageSquare class="size-5" /> }.into_any()),
        ("/history", "History", view! { <History class="size-5" /> }.into_any()), // route not yet implemented
    ];

    view! {
        <BottomNav class="fixed right-0 bottom-0 left-0 sm:hidden">
            <BottomNavGrid>
                {nav_items
                    .into_iter()
                    .map(|(path, label, icon)| {
                        let navigate = navigate.clone();
                        let is_current_path = is_current_path.clone();
                        view! {
                            <BottomNavButton
                                on:click=move |_| {
                                    navigate(path, Default::default());
                                }
                                attr:aria-current=move || is_current_path(path)
                            >
                                {icon}
                                <BottomNavLabel>{label}</BottomNavLabel>
                            </BottomNavButton>
                        }
                    })
                    .collect_view()}
            </BottomNavGrid>
        </BottomNav>
    }
}
