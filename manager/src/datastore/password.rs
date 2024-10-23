use sqlx::sqlite::SqlitePool;
#[derive(sqlx::FromRow, Debug)]
pub struct PasswordEntry {
    username: String,
    password: String,
    app_filter: String,
}

pub enum AccessLevel {
    W,
    RW,
    R,
}
pub struct UnlockedState {
    username: String,
    access: AccessLevel,
}

pub struct LockedState;

pub enum AccessState {
    Locked(UnlockedState),
    Unlocked(LockedState),
}
pub struct PasswordStore {}
