/* At compile time, Rust performs monomorphization of the code using generics - 
turning generic code into specific code by filling in the concrete types */  

#![allow(unused_variables)]
fn main() {
    
    // Option<T> instance:
    let integer= Some (9);          
    let float= Some (9.0);

/*  The compiler reads the instances and identifies two kinds of Option<T>: one holds i32 and the other f64 values,
then it replaces the generic definition of Option<T> for specific ones named Option_i32 and Option_f64. */

    // Monomorphized version of the code: 
    enum Option_i32 {
        Some (i32), 
        None,
    }

    enum Option_f64 {
        Some (f64), 
        None,
    }

    let integer= Option_i32::Some (9); 
    let float= Option_f64::Some (9.0); 

/* Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics -
when the code runs, it performs just as it would if we had duplicated each definition by hand -
the process of monomorphization makes Rust’s generics extremely efficient at runtime    */ 
}
