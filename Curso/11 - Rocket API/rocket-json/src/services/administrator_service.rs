use crate::models::administrator::Administrator;
use crate::models::administrator_token::AdministratorToken;
use crate::services::jwt_service;

pub fn find_by_email(email: String, password: String) -> Option<Administrator> {
    let found_admin = email == "email@email.com" && password == "abcde";

    if found_admin {
        Some(Administrator {
            id: 1,
            name: String::from("Nome"),
            email: email,
            password: password,
        })
    } else {
        None
    }
}

pub fn login(email: String, password: String) -> Result<AdministratorToken, String> {
    let find_adm = find_by_email(email, password);

    match find_adm {
        Some(adm) => Ok(AdministratorToken {
            id: adm.id,
            name: adm.name,
            email: adm.email,
            token: jwt_service::generate_token(adm.id),
        }),
        None => Err(String::from("Email ou senha incorretos.")),
    }
}
