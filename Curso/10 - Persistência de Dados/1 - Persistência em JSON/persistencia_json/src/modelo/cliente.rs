use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cliente {
    pub id: Uuid,
    pub nome: String,
    pub telefone: String,
}
