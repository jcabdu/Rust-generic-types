    // Removing duplication by extracting a function. 

    // Short program that finds the largest number in a list: 
fn main() {
let number_list= vec! [34, 50, 25, 100, 65]; 

let mut largest_number= number_list [0]; 
    
for number in number_list {
    if number > largest_number {
        largest_number= number; 
    }
}
    
println! ("The largest number is {}", largest_number);

    /* To consider a second list, it is possible to duplicate the previous code but that's tedious and error prone. 
        To eliminate this duplication, create an abstraction by defining a function that operates on any list of integers
        1. Identify duplicate code.
        2. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
        3. Update the two instances of duplicated code to call the function instead.    */ 

    // finding the largest number in a list abstractly: 
fn largest (list: &[i32]) -> i32 {           // [i32] represents any concrete slice of i32 values 
    let mut largest= list [0];               // the fn body can operate on an abstract list instead of specific values, generics allow code to operate on abstract types. 

    for &item in list.iter() {
        if item > largest {
            largest= item; 
        }
    }
    largest
}

let number_list= vec! [30, 400, 500, 200, 800]; 
let result= largest (&number_list); 
println! ("The largest number is {}", result); 

let number_list= vec! [200, 300, 400, 100, 33]; 
let result= largest (&number_list); 
println! ("The largest number is {}", result);

fn generic_largest <T: PartialOrd + Copy> (list: &[T]) -> T {       // the fn generic_largest is generic over some type T. 1 parameter named list that is a slice of values of type T 
    let mut largest= list [0];                                      // to move the value out of list[0], the type needs to implement the Copy trait (to be stored on the stack). 

    for &item in list.iter() {
        if item > largest {
            largest= item; 
        }
    }
    largest 
}

let result= generic_largest (&number_list); 
println! ("{}", result); 

let char_list= vec! ['y', 'm', 'a', 'q']; 
let result= generic_largest (&char_list);
println! ("The largest char is {}", result); 

    /*  To compile the previos code is needed a trait called std::cmp::PartialOrd 
     Using the Clone trait instead of Copy - potentially making more heap allocations which can be slow.
     Another solution is changing the return value to &T instead of T - so we dont need any other traits nor heap allocations.  */ 

}