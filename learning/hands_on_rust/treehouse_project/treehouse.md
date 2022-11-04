# Interesting points

1. You need to create the empty variable to store the name value first, otherwise, when read_line wants to write its value to an existing string, it will error if there's nothing there

2. Prefacing a variable with an & creates a reference to the variable, which is a pointer to the variable's value, not the variable itself. You're lending the variable to the function which is being called. Lending with & permits the borrowing function to mutate your variable.

Passing `&mut your_name` to `readline` allows the `read_line` function to write directly to `your_name`

You expect the read_line function to work correctly but if it doesn't then `.expect` will print the error message and exit the program.

3. the `name` at the end of a function doesn't end with ; which is Rust shorthand for return

4. in the function `what_is_your_name` you're calling the function and storing the result in `name` by using the `let name = what_is_your_name();` line.

5. When you type in your name you're actually also going to hit the 'return' key which means you're sending stdin a return and newline so you need to get rid of those with the `.trim()` function

6. There are two types of string in Rust. A `str` which is a string literal that can't be changed (generally) and `String` which are _dynamic_ because they store a length, location and capacity. You can append Strings and edit them. An array of string literals `let visitor_list = ["bert", "steve", "jane"]` and you would refer to visitor_list[1] to get "steve".

7. Using `for visitor in &visitor_list` compares every name in the list to the name you entered. The `&` means you're borrowing the visitor_list and not taking ownership of it. You can't change the visitor_list because you're only borrowing it. If the `&name == visitor` then `allow_them_in` is set to true

8. Use a struct to add more data about each visitor. A struct isn't a variable, it's a data structure which actually describes what a variable of that type can contain. You make a visitor type and then it'll be used over and over for different visitors.

9. Structs have associated functions and methods. `String::new` and `StdIn::read_line` are both methods associated with a structure. To make a 'visitor' you need a constructor which are functions associated with a struct type which allow you to quickly create an instance of that type. You can use the `impl` keyword to implement methods on a struct. You can also use the `impl` keyword to implement methods on an enum.

10. 