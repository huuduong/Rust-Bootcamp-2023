fn main() {
    println!("Hello, world!");
    return_reference_string();
    print_generic(10u8);
    print_generic(20u64);
    print_generic("Hello");
    print_generic::<&str>("Hello");
    print_generic("Hello".to_string());
    // error
    print_generic::<u64>(10);

    let point= Point1::<i32> { x: (10), y: (20) };
    let point1= Point1::new(1.0, 2.0);
    let point1= Point1::<f32>::new (1.0, 2.0);

    //
    // let honda = Car{category:"Honda".to_string()};
    // let nissan = Car{category:"Nissan".to_string()};
    let vios = Sedan{};
    println!("Sedan speed: {}", vios.speed());
    let circle = Circle{radius:10.0};
    let rectangle = Rectangle{
        width: 10.0,
        height: 20.0,
    };
    let a = get_area(circle);
    println!("Circle area: {}", a);

    let b = get_area_2(rectangle);
    println!("Rectangle area: {}", b);

}


fn print_generic<T: std::fmt::Debug>(input: T)
{
    println!("input {:?}", input);
}

struct Point {
    x: u32,
    y: u32,
}

struct Point1 <T>{
    x: T,
    y: T,
}

impl<U> Point1<U> {
    fn new (x: U, y:U)-> Self{
        Self { x, y }
    }
}

fn return_reference_string<'a>() ->&'a str{
    //let my_string: String = String::from("Nguyen Huu Duong");
    "return &my_string"

}

// fn return_reference_string1() ->&str{
//     //let my_string: String = String::from("Nguyen Huu Duong");
//     "return &my_string".to_string()

// }

//Trait
// pub struct Car{
//     category: String
// }

// impl Car {
//     fn get_category(&self){
//         println!("category:{}", self.category);
//     }
// }

trait Car {
    fn get_category(&self)->String;
    fn speed(&self)->u32;
}

struct Sedan{}
impl Car for Sedan {
    fn get_category(&self)->String{
        "Sedan".to_string()
    }

    fn speed(&self)->u32{
        120
    }
}
struct Coupe{}
impl Car for Coupe {
    fn get_category(&self)->String{
        "Coupe".to_string()
    }

    fn speed(&self)->u32{
        80
    }
}
trait Drawable {
    fn area(&self)-> f64;
}
struct Rectangle{
    width: f64,
    height:f64,
}
impl Drawable for Rectangle {
    fn area(&self)-> f64{
        self.width * self.height
    }
}
struct Circle{
    radius: f64,
}
impl  Drawable for Circle {
    fn area(&self)-> f64{
        self.radius * self.radius * 3.14
    }
}

fn get_area<T:Drawable>(method:T)-> f64{
    method.area()
}

fn get_area_2(method:impl Drawable)-> f64{
    method.area()
}

struct Square<T>{
    x: T
}

// cong nhieu trait
impl<T:std::ops::Mul<Output = f64> + Copy>  Drawable for Square<T> {
    fn area(&self)-> f64 {
        self.x * self.x
    }
}
// impl <T> std::ops::Mul for Square<T> {
//     type Output  = f64;

//     fn mul(self, rhs: Self) -> Self::Output {
//         let res = self.x * self.x;
//         res
//     }
// }