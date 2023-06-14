fn main(){
   //in rust we use snake convention
   //let is used to initialise 
   //so , by initially, the variable is immutable by default, then we have to make the variable mutable by "mut" keyword
   let mut block_chain = "ethereum";
   println!("Hello {}!",block_chain);

   //also we can explicitly change the datatype of variable
   let age:u8 = 9;
   println!("VAr {}!",age);

   //for float variables
   let mut fl:f32 = 0.33434;
    println!("Float:{}",fl);

    //WE can declare same name to variables with name;
    let name = "solana";
    let name = "Ethereum";

 
 println!("Blockchain:{}",name);

 //Constants: value will not be changed throughout the execution of the program
const MY_AGE:i32 = 20;
 println!("Age is:{}",MY_AGE);
}