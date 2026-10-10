trait Print {
    fn print(&self);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

impl Print for Point<i32> {
    fn print(&self) {
        println!("(x: {}, y: {})", self.x, self.y)
    }
}

impl Print for Point<f32> {
    fn print(&self) {
        println!("(x: {}, y: {})", self.x, self.y)
    }
}

impl Print for Point<String> {
    fn print(&self) {
        println!("(x: {}, y: {})", self.x, self.y)
    }
}

fn point() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let string_point = Point {
        x: String::from("1.1"),
        y: String::from("2.2"),
    };

    println!("{} - {}", int_point.get_x(), int_point.get_y());

    int_point.print();
    float_point.print();
    string_point.print();
}

struct Pair<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pair<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
}

fn pair() {
    let pair1 = Pair::new(5, 10.5);
    let pair2 = Pair::new(4, "Y");

    println!("x = {}, y = {}", pair1.x, pair1.y);
    println!("x = {}, y = {}", pair2.x, pair2.y);
}

trait DatabaseService {
    fn save_message(&self, message: &str);
    fn show_message(&self) -> String;
}

struct MySQLService;

impl DatabaseService for MySQLService {
    fn save_message(&self, message: &str) {
        println!("Salvando '{}' no Mysql", message)
    }

    fn show_message(&self) -> String {
        let message = "Mensagem do MySQL";
        println!("Buscando: {}", message);
        message.to_string()
    }
}

struct PostgreSQLService;

impl DatabaseService for PostgreSQLService {
    fn save_message(&self, message: &str) {
        println!("Salvando '{}' no PostgreSQL", message)
    }

    fn show_message(&self) -> String {
        let message = "Mensagem do PostgreSQL";
        println!("Buscando: {}", message);
        message.to_string()
    }
}

struct GenericService<T: DatabaseService> {
    database_service: T,
}

impl<T: DatabaseService> GenericService<T> {
    fn new(database_service: T) -> Self {
        GenericService { database_service }
    }

    fn save_message(&self, message: &str) {
        self.database_service.save_message(message);
    }

    fn show_message(&self) -> String {
        self.database_service.show_message()
    }
}

fn database_drivers() {
    let mysql_service = MySQLService;
    let postgres_service = PostgreSQLService;

    let mysql_generic_service = GenericService::new(mysql_service);
    let postgres_generic_service = GenericService::new(postgres_service);

    mysql_generic_service.save_message("Olá Mundo!");
    let message_from_mysql = mysql_generic_service.show_message();
    println!("{}", message_from_mysql);

    postgres_generic_service.save_message("Olá Mundo!");
    let message_from_postgre = postgres_generic_service.show_message();
    println!("{}", message_from_postgre);
}

fn main() {
    point();
    println!("-----------------");
    pair();
    println!("-----------------");
    database_drivers();
}
