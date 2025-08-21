fn main() {
    // declaring variable with let 
    let apple = 50 ; 
    let mangoes = 45+10 ; 
    let fruits = apple+mangoes ; 
    println!("My Garden is right here!") ; 
    println!("we have these {} numbers of the fruits in the garden out of these {} are apple and {} are mangoes differnce in apple and mangoes " , fruits , apple ,  mangoes) ; 

    // we can do in  this way also : 

    println!("we have these {fruits} numbers of the fruits in the garden out of these {apple} are apple and {mangoes} are mangoes differnce in apple and mangoes is {}." , apple-mangoes) ;


    /* In rust we cannot put the operation directly inside the string , for that we have to write it serperately */
    // println!("difference between the apple and manoges {apple-mangoes}") -- >> illegal

    println!("difference between the apple and mangoes: {}" , apple-mangoes) ; 



}