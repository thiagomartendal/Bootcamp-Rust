trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Dog {
    fn speak(&self) {
        println!("Au Au!")
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Miau!")
    }
}

fn speak_now(animal: &dyn Speak) {
    animal.speak();
}

fn speak_now2(animal: &impl Speak) {
    animal.speak();
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    dog.speak();
    // cat.speak();

    // speak_now(&dog); // Não aceita pois Dog não implementa a trait Speak
    speak_now(&cat);
    speak_now2(&cat);
}
