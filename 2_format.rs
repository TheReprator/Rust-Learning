fn main() {

	 /* 1)In general, the `{}` will be automatically replaced with any arguments. These will be stringified.
	 	  if arguments is greater or lesser, then format exception is throw at compile time

	 	2)Without a suffix, 31 becomes an i32. You can change what type 31 is by providing a suffix. The number 31i64 for example has the type i64.
	 		i.e. println!("{} {} {}", 31i64, "vikram", 'v');
    */
	println!("Any type Placeholder");
	println!("{} {} {}", 31, "vikram", 'v');	


	/* There are various optional patterns this works with. Positional
     arguments can be used.
    */
    println!("\n\nPositional Placeholder");
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");


	/* Here you define a named placholder, to which the value will be assigned later
    */
	println!("\n\nNamed arguments Placeholder");
    println!("{subject} {verb} {object} whose age is {age}",
             object="the lazy dog",
             age = 31,
             subject="the quick brown fox",
             verb="jumps over");

    /*
		Here it convert the second argument into binary number with placholder ':b'. So 2 is converted into 10
    */
    println!("\n\nSpecial Formatting Placeholder");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);


     /* 
     	To move any type(ie. number, string, char) to a particular indentation. Use{placeholder:> width}.
     	a)Here, it will move char 'v' to 6 places on right side, so output will be like: "      v"
     	b)If the statement is like below,
     		println!("{number:>width$}", number="vikram", width=6);
     		then, it is not going to move at all, because, vikram size is 6 and width is also 6, so output will be same
     	c) In below scenario
     		println!("{number:>width$}", number="vikram", width=6);
     			outpute:"      vikram"
    */
    println!("\n\nIndent or Align Space Placeholder");
    println!("{number:>width$}", number='v', width=6);


    /*
    	It is same as above the only difference is it place 0 in place or space
     Output:
	  0vikram
    */
    println!("\n\nIndent or Align With 0 Placeholder");
    println!("{number:0>width$}", number="vikram", width=7);
    
}


