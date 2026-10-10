#[derive(serde::Serialize)]
pub struct AdministratorToken {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub token: String,
}
