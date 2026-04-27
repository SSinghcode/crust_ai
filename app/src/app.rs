use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::layout::app_bottom_nav::AppBottomNav;
use crate::components::layout::app_wrapper::AppWrapper;
use crate::domain::home::page_home::HomePage;
use crate::domain::home::routes::HomeRoutes;

#[component]
pub fn App() -> impl IntoView {
    let theme_mode = ThemeMode::init();

    provide_meta_context();

    view! {
        <Title text="Crust AI" />

        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            <AppWrapper>
                <main class="flex-1 overflow-x-clip overflow-y-hidden flex flex-col pb-[var(--bottom__nav__height)] sm:pb-0">
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route path=StaticSegment(HomeRoutes::base_url()) view=HomePage />
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
