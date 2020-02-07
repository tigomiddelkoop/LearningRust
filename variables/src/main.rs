fn main() {
    /*
    SHADOWING
    you can declare a new variable with the same name as a previous variable,
    and the new variable shadows the previous variable. Rustaceans say that the first variable is shadowed by the second,
    which means that the second variable’s value is what appears when the variable is used.
    We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:
    */
    let x = 5;
    let x = x + 5;
    let x = x * 2;

    println!("The value of x is: {}", x);
    /*

    Shadowing is different from marking a variable as mut,
    because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.
    By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
    we can change the type of the value but reuse the same name.
    For example, say our program asks a user to show how many spaces they want between some text by inputting space characters,
    but we really want to store that input as a number:

    let spaces = "   ";
    let spaces = spaces.len();
    This construct is allowed because the first spaces variable is a string type and the second spaces variable,
    which is a brand-new variable that happens to have the same name as the first one, is a number type.
    Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead,
    we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here, we’ll get a compile-time error:

    let mut spaces = "   ";
    spaces = spaces.len();
    THIS WILL ERROR!

    ##########################
            DATA TYPES
    ##########################

    A Constant can not be mutated, they are always UPPERCASE and you always need to define its type;

    Unsigned integers can not be negative
    Signed integers can be negative if you present their values when setting a variable
    Available bit sizes are 8/16/32/64/128/size

    If for some reason integer overflow would occur rust lets it overflow but then adds the left count to it as well
    So 256 would become 0 and 257 would become 1 and so on.

    Char types can store more than just ASCII, It supports Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid
    and Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

    The default type per type/variable
    integer types default to i32
    float types default to f64
    bool type does not have default, duh.
    char type does not default to anything.

    ##########################
          COMPOUND TYPES
    ##########################

    #### TUPLE
    A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    We create a tuple by writing a comma-separated list of values inside parentheses.
    Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

    Signed integer of 32 bits
    Floatpoint of 64 bits
    And an unsigned 8 bit integer.
    */
    let tup: (i32, f64, u8) = (500, 6.4, 255);

    /*
    A tuple is considered a single compound element. To get the individual values out of a tuple you need to do the following.
    We are destructuring the tuple in seperate variables. And assign the values of the tuple to var1, var2 and var3
    */

    let (var1, var2, var3) = tup;
    println!("The value of var1 is {}, The value of var2 is {} and the value of var 3 is {}", var1, var2, var3);

    /*
    You can also get the values from a tuple like this, we are getting the first value from the tuple
    The first index in a tuple is 0, as with most programming languages
    */
    let five_hundred = tup.0;
    println!("The chosen tuple value is {}", five_hundred);

    /*

    #### ARRAY
    The other way to store data is with arrays
    unlike the tuple, every element in an array must be the same.

    The entries in an array with rust are seperated with comma's

    Like this
    */
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // This array can have 5 signed integers
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"]; // This is an string array with 12 items

    /*
        If you need to access a element of the array you do it like so
    */
    let jan: &str = months[0];
    println!("The chosen month is {}", jan);

    /*
        If you select a entry in the array that does not exist the program will panic and will throw an error or will crash.

    */
}