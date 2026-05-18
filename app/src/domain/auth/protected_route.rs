use leptos::prelude::*;
use leptos_router::components::{Outlet, Redirect};

use crate::domain::auth::model::User;
use crate::domain::auth::server_fns::get_current_user;

#[component]
pub fn ProtectedRoute() -> impl IntoView {
    let current_user = use_context::<Resource<Result<Option<User>, ServerFnError>>>()
        .expect("current_user resource missing from context");

    view! {
        <Suspense fallback=|| ()>
            {move || {
                current_user.get().map(|result| match result {
                    Ok(Some(_)) => view! { <Outlet /> }.into_any(),
                    _ => view! { <Redirect path="/login" /> }.into_any(),
                })
            }}
        </Suspense>
    }
}
