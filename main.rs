#![allow(unused_variables)]
fn main() {

    // Other ways of using generic type parameters:  

    //  1. In struct definitions (in one or more fields using the <> syntax)
struct Point <T> {           // to hold x and y coordinate values of any type - the Point<T> struct is generic over some type T, and both fields are that same type
    x: T, 
    y: T, 
}

let integer= Point {x: 5, y: 10}; 
let float= Point {x: 1.2, y: 5.3}; 

    // Struct with fields of different generic type: 
struct Point2 <T, U> {
    x: T, 
    y: U, 
}

let integer_float= Point2 {x: 10, y: 10.0}; 

    //  2. In enum definitions - enums to hold generic data types in their variants: 
enum Option <T> {           // Option is a std lib enum generic over type T 
    Some (T),              // variant Some that hold one value of type T, but could hold more - abstraction of having an optional value/s.  
    None,                 // null variant
}

enum Result <T, E> {      // Std lib Result enum is generic over multiple types - 2 in this case. Good for error handling to return a value of type T or E.  
    Ok (T),              // Ok variant holds a value type T (e.g. T: std::fs::File when opening a file successfully)
    Err (E),            // Err holds a value type E (e.g. E: std::io::Error)
}

    //  3. In method definitions:
impl <T> Point <T> {           // method y implemented on the struct Point with fields of type generic
    fn y (&self) -> &T {      // method that returns a reference to the data in the field y of type T  
        &self.y              
    }
}

let p= Point {x: 10, y: 20}; 
println! ("El método y que me devuelve el valor y de p: p.y()= {}, y el valor del campo y en notación de índice es p.y= {}", p.y(), p.y); 

    //  implementing methods on Point <f32> instances only
impl Point <f32> {
    fn distance_from_origin (&self) -> f32 {         // returns a concrete type f32 for the generic type parameter T
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

println! ("La distancia al origen desde el punto (5.3, 9.5) es: {}", Point{x:5.3, y:9.5}.distance_from_origin()); 

    // Implementing methods with parameters T and U:
impl <T, U> Point2 <T, U> {
    fn mixup <V, W> (self, extra_point: Point2 <V, W>) -> Point2 <T, W> {        // mixup method with arguments self and extra_point 
        Point2 {x: self.x, y: extra_point.y}                                    // some generic parameters T and U are declared with impl and some (V and W) are declared with the method definition and only relevant there
        }
    }

let p1= Point2 {x: 5, y: 10.6}; 
let p2= Point2 {x: "hola!", y:'j'}; 

let p3= p1.mixup (p2);
println! ("p3.x= {}, p3.y= {}", p3.x, p3.y); 

}