trait ContaCaracteres {
    fn conta_caracteres(&self) -> usize;
}

struct Elemento {
    x: i32,
}

impl ContaCaracteres for i32 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl ContaCaracteres for f64 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}

impl<'a> ContaCaracteres for &'a str {
    fn conta_caracteres(&self) -> usize {
        self.chars().count()
    }
}

impl ContaCaracteres for String {
    fn conta_caracteres(&self) -> usize {
        self.chars().count()
    }
}

impl ContaCaracteres for Elemento {
    fn conta_caracteres(&self) -> usize {
        self.x.to_string().chars().count()
    }
}

fn total_caracteres<T: ContaCaracteres>(valor: T) -> usize {
    valor.conta_caracteres()
}

fn tamanho<T>(array: &[T]) -> usize {
    array.len()
}

fn tamanho_array() {
    let inteiros: [i32; 5] = [1, 2, 3, 4, 5];
    let floats: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
    let strings: [&str; 3] = ["abc", "def", "ghi"];

    println!("Tamanho do array {:?}: {}", inteiros, tamanho(&inteiros));
    println!("Tamanho do array {:?}: {}", floats, tamanho(&floats));
    println!("Tamanho do array {:?}: {}", strings, tamanho(&strings));
}

fn caracteres() {
    let int: i32 = 12345;
    let float: f64 = 123.456;
    let slice_string: &str = "Olá Mundo!";
    let string: String = String::from("Um dia...");
    let elemento: Elemento = Elemento { x: 1234567890 };

    let x = elemento.x;

    println!("Total de caracteres de {}: {}", int, total_caracteres(int));
    println!(
        "Total de caracteres de {}: {}",
        float,
        total_caracteres(float)
    );
    println!(
        "Total de caracteres de {}: {}",
        slice_string,
        total_caracteres(slice_string)
    );
    println!(
        "Total de caracteres de {}: {}",
        string.clone(),
        total_caracteres(string)
    );
    println!(
        "Total de caracteres de Elemento(x: {}): {}",
        x,
        total_caracteres(elemento)
    );
}

fn main() {
    tamanho_array();
    println!("---------------");
    caracteres();
}
