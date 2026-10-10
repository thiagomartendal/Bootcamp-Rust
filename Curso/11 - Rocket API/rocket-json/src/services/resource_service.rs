use crate::dtos::resource_dto::ResourceDto;
use crate::models::resource::Resource;

pub fn resources_list() -> Vec<Resource> {
    vec![
        Resource {
            id: 1,
            title: String::from("Recurso 1"),
            description: String::from("O recurso 1"),
        },
        Resource {
            id: 2,
            title: String::from("Recurso 2"),
            description: String::from("O recurso 2"),
        },
    ]
}

pub fn get_resource(id: u32) -> Resource {
    Resource {
        id: id,
        title: format!("Recurso {}", id),
        description: format!("O recurso {}", id),
    }
}

pub fn register_resource(resource: ResourceDto) -> Result<Resource, String> {
    println!("Titulo: {}", resource.title);
    println!("Descrição: {}", resource.description);

    if true {
        Ok(Resource {
            id: 1,
            title: resource.title,
            description: resource.description,
        })
    } else {
        Err(String::from("Erro ao cadastrar."))
    }
}

pub fn update_resource(id: u32, resource: ResourceDto) -> Result<Resource, String> {
    println!("ID: {}", id);
    println!("Titulo: {}", resource.title);
    println!("Descrição: {}", resource.description);

    if true {
        Ok(Resource {
            id: id,
            title: resource.title,
            description: resource.description,
        })
    } else {
        Err(String::from("Erro ao atualizar."))
    }
}

pub fn delete_resource(id: u32) -> Result<(), String> {
    println!("Recurso {} excluído.", id);

    if true {
        Ok(())
    } else {
        Err(String::from("Erro ao excluir."))
    }
}
