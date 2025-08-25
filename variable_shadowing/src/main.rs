fn main() {
    // Variable shadowing : 
    let grams_of_protein:&str = "100.345" ; 
    println!("{}" , grams_of_protein) ; 

    // here shadowing has happened and the earlier the type of the grams_of_protein was string now re-declaring with the let as an float so the grams_of_protein will become float now.

    let grams_of_protein:f64 = 100.345 ; 
    println!("{}" , grams_of_protein+20.00) ; 


    // converting the floating point to the integer using the variable shadowing : 
    let mut grams_of_protein : i32 = 100 ; 
    println!("{}" , grams_of_protein) ;
    grams_of_protein = 45.23 ; 
}
