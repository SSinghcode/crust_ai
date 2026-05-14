use icons::{Plus, X};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::domain::home::routes::HomeRoutes;

#[component]
pub fn AppSidenav() -> impl IntoView {
    let sidenav_open = use_context::<RwSignal<bool>>()
        .expect("AppSidenav: sidenav_open context not found");
    let navigate = use_navigate();

    view! {
        // ── Backdrop ─────────────────────────────────────────────────
        <div
            class=move || {
                if sidenav_open.get() {
                    "fixed inset-0 z-40 bg-black/40 transition-opacity"
                } else {
                    "fixed inset-0 z-40 bg-black/40 transition-opacity opacity-0 pointer-events-none"
                }
            }
            on:click=move |_| sidenav_open.set(false)
        />

        // ── Panel ─────────────────────────────────────────────────────
        <div class=move || {
            format!(
                "fixed top-0 left-0 z-50 h-full w-72 bg-background border-r border-border flex flex-col transition-transform duration-300 {}",
                if sidenav_open.get() { "translate-x-0" } else { "-translate-x-full" }
            )
        }>
            // Header
            <div class="flex items-center justify-between px-4 py-3 border-b border-border shrink-0">
                <span class="font-semibold text-sm">"Crust AI"</span>
                <button
                    class="p-1 rounded-md hover:bg-accent text-muted-foreground"
                    on:click=move |_| sidenav_open.set(false)
                >
                    <X class="size-4" />
                </button>
            </div>

            // New chat button
            <div class="px-3 pt-3 shrink-0">
                <button
                    class="w-full flex items-center gap-2 px-3 py-2 rounded-md text-sm hover:bg-accent text-muted-foreground border border-border"
                    on:click=move |_| {
                        sidenav_open.set(false);
                        navigate(HomeRoutes::base_url(), Default::default());
                    }
                >
                    <Plus class="size-4" />
                    "New Chat"
                </button>
            </div>

            // History will go here in the future
            <div class="flex-1" />
        </div>
    }
}
