# Interesting points

1. You need to create the empty variable to store the name value first, otherwise, when read_line wants to write its value to an existing string, it will error if there's nothing there

2. Prefacing a variable with an & creates a reference to the variable, which is a pointer to the variable's value, not the variable itself. You're lending the variable to the function which is being called. Lending with & permits the borrowing function to mutate your variable.

Passing `&mut your_name` to `readline` allows the `read_line` function to write directly to `your_name`

You expect the read_line function to work correctly but if it doesn't then `.expect` will print the error message and exit the program.

3. the `name` at the end of a function doesn't end with ; which is Rust shorthand for return

4. in the function `what_is_your_name` you're calling the function and storing the result in `name` by using the `let name = what_is_your_name();` line.

5. When you type in your name you're actually also going to hit the 'return' key which means you're sending stdin a return and newline so you need to get rid of those with the `.trim()` function