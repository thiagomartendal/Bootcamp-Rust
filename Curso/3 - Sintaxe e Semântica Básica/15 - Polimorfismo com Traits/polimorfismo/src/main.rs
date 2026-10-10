trait Display {
    fn display(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

struct Circle {
    x: i32,
    y: i32,
    radius: i32, // Um parâmetro a mais em relação a struct Point
}

impl Display for Point {
    fn display(&self) -> String {
        format!("Ponto(x: {}, y: {})", self.x, self.y)
    }
}

impl Display for Circle {
    fn display(&self) -> String {
        // Por formatar uma string com um parâmetro a mais, surge um
        // polimorfismo na implementação da trait Display para a
        // struct circle
        format!(
            "Círculo(x: {}, y: {}, raio: {})",
            self.x, self.y, self.radius
        )
    }
}

// impl deve ser usado para situações mais simples, buscando performance
// como existe um polimorfismo na implementação, o melhor seria usar &dyn
// nesta função
fn print_display_impl(item: &impl Display) {
    println!("{}", item.display());
}

// dyn deve ser usado para objetos mais complexos (como listas) e situações
// mais avançadas (como polimorfismo com traits)
fn print_display_dyn(items: &[&dyn Display]) {
    for item in items {
        println!("{}", item.display());
    }
}

fn main() {
    let point = Point { x: 5, y: 10 };
    let circle = Circle {
        x: 15,
        y: 25,
        radius: 5,
    };

    print_display_impl(&point);
    print_display_impl(&circle);

    println!("-----------------------");

    print_display_dyn(&[&point, &circle]);
}
