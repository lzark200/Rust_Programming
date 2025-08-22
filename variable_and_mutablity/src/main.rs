fn main() {
    // declaring variable with let 
    let apple = 50 ; 
    let mangoes = 45+10 ; 
    let fruits = apple+mangoes ; 
    
    println!("My Garden is right here!") ; 
    println!("we have these {} numbers of the fruits in the garden out of these {} are apple and {} are mangoes differnce in apple and mangoes " , fruits , apple ,  mangoes) ; 

    // we can do in  this way also : 

    println!("we have these {fruits} numbers of the fruits in the garden out of these {apple} are apple and {mangoes} are mangoes differnce in apple and mangoes is {}." , apple-mangoes) ;


    /* In rust we cannot put the operation directly inside the string inside the {} , for that we have to write it serperately */
    // println!("difference between the apple and manoges {apple-mangoes}") -- >> illegal


    // legal : 
    println!("difference between the apple and mangoes: {}" , apple-mangoes) ; 


    /*Variables after the string can referenced by the index value starting from 0 to n-1 , where n is no. of the variables */
    println!("There are {0} apples and {1} mangoes" , apple , mangoes) ; 
    println!("There are {1} apples and {0} mangoes" , apple , mangoes) ; 

    let vegies = 56 ; // Here warning is shown because variable is not been used yet
    // to supress the variable not used warning we can us start variable with the _vegies 

    let _tomato = 85 ; 

    //Immutable and mutable variable in the rust:
    let gym_reps = 85 ; 
    println!("{}" , gym_reps) ; 
    
    // gym_reps = 85 ; illegal : cannot mutate immutable variable `gym_reps` we cannot change the variable value since it is immutable.

    // to perform this thing we have to use the mut keyword right after the let keyword : 

    let mut gym_repos = 89 ; 
    print!("{}\n" , gym_repos) ;  // print! is used to print the string without the newline character at the last of the string.
    gym_repos = 8525  ;  // here we can change the value of the variable since it is mutable.
    println!("{}" , gym_repos) ; 

    // Even if we are able to change the value but we cannot change type of the value.

    // conclusion every variable directly written with the let variable is immutable.




}