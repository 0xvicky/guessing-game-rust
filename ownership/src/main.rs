fn block_word(block_info:&String){
println!("Block Info:{}",block_info);
}


fn main() {
    println!("Ownership");
    
    let s1 =String::from("hello");
    //Ownership issue
    // let s2 = s1;

    // println!("Greets {}", s1);
    // println!("Greets {}", s2);

    //actually,above we're getting error because of the concept of "ownership" in the rust, rust compiler prevents to give one variable ownership to another variable because it may leads to a crash because every function have some space allocated while runtime and then it deallocated back to the OS when execution is done.
    //This is because the string in rust is stored in stack and Heap, the address to the heap of the string data is stored in the stack for every variable.
  //ultimately we can't have two variables pointing to the same data in the heap as rust has automatic memory deallocationn
   /**********************************
    * Here, s1 is now moved to s2, so it
    * is use
    *
    **********************************    */

    let s2 = &s1;//References, we can do this copy with the references, 
    //Here reference variable is pointing to the stack of the variable s1 where the address of the data is stored
    //references don't drop their value, no deallocation requires but s1 values need to be deallocated
    //so reference are harmless, they're read only

    // println!("Greetings from {}", s1);
    // println!("Greetings from {}", s2);
    // block_word(s1); //This won't work because, this function is expecting a reference but we're passing a string variable
    block_word(&s1);//This willl work
    block_word(s2);//This will also work as there are mainly in both cases are calling references
}
