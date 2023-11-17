use chrono::{DateTime, Utc};
use derive_getters::Getters;

#[derive(Getters)]
pub struct User {
    /// User id in auth system
    id: String,
    /// User first name
    first_name: String,
    /// User second name
    last_name: String,
    /// User unique login
    login: Option<String>,
    /// User unique email
    email: Option<String>,
    /// Is user email verified
    email_verified: bool,
    /// External user id
    external_id: Option<String>,
    /// Password hash
    password: Option<String>,
    /// User create date,time
    created_at: DateTime<Utc>,
    /// User update date,time
    updated: DateTime<Utc>,
}
