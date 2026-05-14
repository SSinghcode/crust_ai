use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{StaticSegment, ParamSegment};

use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::layout::app_bottom_nav::AppBottomNav;
use crate::components::layout::app_sidenav::AppSidenav;
use crate::components::layout::app_wrapper::AppWrapper;
use crate::domain::chat::page_chat::ChatPage;
use crate::domain::home::page_home::HomePage;
use crate::domain::home::routes::HomeRoutes;


#[component]
pub fn App() -> impl IntoView {
    let theme_mode = ThemeMode::init();

    // Start as false — safe default for both server and browser.
    // We cannot read localStorage here because Leptos SSR hydration
    // ignores this init code on the client and reuses the server value.
    // The read Effect below corrects this after hydration.
    let sidenav_open = RwSignal::new(false);

    // This entire block only compiles for the BROWSER build.
    // The server has no localStorage, window, or DOM — so we skip it there.
    #[cfg(not(feature = "ssr"))]
    {
        // READ EFFECT — runs once after the page hydrates on the client.
        // At this point the browser IS available, so we can safely
        // read localStorage. If the user had the sidenav open last time,
        // we set the signal to true so the UI matches what they left.
        Effect::new(move |_| {
            if let Some(stored) = web_sys::window()
                // Step 1: get the browser window (None if not in browser)
                .and_then(|w| w.local_storage().ok().flatten())
                // Step 2: get localStorage from the window
                // .ok() converts Result → Option, .flatten() removes Option<Option<>>
                .and_then(|ls| ls.get_item("sidenav_open").ok().flatten())
                // Step 3: read the key — returns Some("true") / Some("false") / None
            {
                // Step 4: "true" string → true bool, anything else → false
                sidenav_open.set(stored == "true");
            }
            // If key doesn't exist (first visit), we do nothing — signal stays false
        });

        // WRITE EFFECT — runs whenever sidenav_open changes.
        // We use prev: Option<()> to detect the first run:
        //   prev = None  → first run (skip it, read Effect hasn't corrected yet)
        //   prev = Some  → user actually toggled, safe to write
        // Without this skip, the write Effect would run on mount with false
        // and overwrite the "true" we stored last time — breaking persistence.
        Effect::new(move |prev: Option<()>| {
            let is_open = sidenav_open.get(); // subscribe — reruns when signal changes
            if prev.is_some() {              // skip the very first run
                let value = if is_open { "true" } else { "false" };
                if let Some(ls) = web_sys::window()
                    .and_then(|w| w.local_storage().ok().flatten())
                {
                    let _ = ls.set_item("sidenav_open", value);
                    // _ means we intentionally ignore the Result here
                }
            }
        });
    }

    provide_meta_context();
    provide_context(sidenav_open);

    view! {
        <Title text="Crust AI" />

        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            <AppSidenav />

            <AppWrapper>
                <main class="flex-1 overflow-x-clip overflow-y-hidden flex flex-col pb-[var(--bottom__nav__height)] sm:pb-0">
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route path=StaticSegment(HomeRoutes::base_url()) view=HomePage />
                        <Route
                            path=(StaticSegment("chats"), ParamSegment("id"))
                            view=ChatPage
                        />
                    </Routes>
                </main>
            </AppWrapper>

            <AppBottomNav />
        </Router>
    }
}

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! { <p>"Not Found."</p> }
}
