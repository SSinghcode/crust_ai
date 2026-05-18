use app::app::App;
use app::common::app_state::AppState;
use app::domain::auth::model::User;
use app::shell::shell;
use axum::Router;
use axum::body::Body as AxumBody;
use axum::extract::State;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::SessionPgPool;
use leptos::prelude::*;
use leptos_axum::{
    LeptosRoutes, generate_route_list, handle_server_fns_with_context,
    render_app_to_stream_with_context,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::AuthSession;

use crate::fallback::file_and_error_handler;

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

pub async fn build_app_router(conf_file: ConfFile, pool: PgPool) -> anyhow::Result<Router> {
    let leptos_options = conf_file.leptos_options;

    let routes = generate_route_list(|| view! { <App /> });
    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_lifetime(chrono::Duration::hours(2));
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config).await?;
    let auth_config = AuthConfig::<Uuid>::default();

    let app_state = AppState { leptos_options, pool: pool.clone() };

    Ok(Router::new()
        .route("/api/{*fn_name}", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(
            AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(app_state))
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[axum_macros::debug_handler]
pub async fn server_fn_handler(
    State(state): State<AppState>,
    auth:AuthSession,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(auth.clone());
            provide_context(state.clone());
        },
        request,
    )
    .await
}

#[axum_macros::debug_handler]
pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
     auth:AuthSession,
    req: Request<AxumBody>,
) -> Response {
    let leptos_options = app_state.leptos_options.clone();

    let handler = render_app_to_stream_with_context(
        move || {
            provide_context(app_state.clone());
              provide_context(auth.clone());
        },
        move || shell(leptos_options.clone()),
    );
    handler(req).await.into_response()
}
