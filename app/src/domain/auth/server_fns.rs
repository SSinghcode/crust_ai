use leptos::prelude::*;
use crate::domain::auth::model::User;
#[cfg(feature = "ssr")]
use crate::common::app_state::AppState;
#[cfg(feature = "ssr")]
type AuthSession = axum_session_auth::AuthSession<
    User,
    uuid::Uuid,
    axum_session_sqlx::SessionPgPool,
    sqlx::PgPool,
>;

#[server]
pub async fn register(
    email: String,
    username: String,
    password: String,
) -> Result<(), ServerFnError> {
    let state: AppState = use_context::<AppState>()
        .ok_or_else(|| ServerFnError::new("No state"))?;

    let password_hash: String = pwhash::bcrypt::hash(&password)
        .map_err(|e: pwhash::error::Error| ServerFnError::new(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3)",
        email,
        username,
        password_hash
    )
    .execute(&state.pool)
    .await
    .map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server]
pub async  fn login(email:String, password: String)->Result<(), ServerFnError>{
    let appstate = use_context::<AppState>()
    .ok_or_else(|| ServerFnError::new("No state"))?;
    let auth = use_context::<AuthSession>()
    .ok_or_else(|| ServerFnError::new("No auth"))?;

    let user  = sqlx::query_as!(
        User,
        "SELECT * FROM users where email = $1",
        email
    ).fetch_optional(&appstate.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .ok_or_else(|| ServerFnError::new("Invalid email or password"))?;
    
    if !pwhash::bcrypt::verify(&password,&user.password_hash){
        return Err(ServerFnError::new("Invalid email or password"));
    }
    auth.login_user(user.unid);
    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
     let auth = use_context::<AuthSession>()
     .ok_or_else(||ServerFnError::new("Invalid state"))?;
    // call auth.logout_user()
    auth.logout_user();
    Ok(())
}


#[server]
pub async fn get_current_user() -> Result<Option<User>, ServerFnError> {
     let auth =  use_context::<AuthSession>()
     .ok_or_else(||ServerFnError::new("Invalid state"))?;
    Ok(auth.current_user.clone())

}

















