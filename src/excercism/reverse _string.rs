use unicode_segmentation::UnicodeSegmentation;
/*
Introduction
Reversing strings (reading them from right to left, rather than from left to right) is a surprisingly common task in programming.

For example, in bioinformatics, reversing the sequence of DNA or RNA strings is often important for various analyses, such as finding complementary strands or identifying palindromic sequences that have biological significance.

Instructions
Your task is to reverse a given string.

Some examples:

Turn "stressed" into "desserts".
Turn "strops" into "sports".
Turn "racecar" into "racecar".
Bonus
Test your function on this string: uüu and see what happens. Try to write a function that properly reverses this string. Hint: grapheme clusters

To get the bonus test to run, remove the ignore flag (#[ignore]) from the last test, and execute the tests with:

$ cargo test --features grapheme
You will need to use external libraries (a crate in rust lingo) for the bonus task. A good place to look for those is crates.io, the official repository of crates.

Check the documentation for instructions on how to use external crates in your projects.

How to debug
When a test fails, a message is displayed describing what went wrong and for which input. You can also use the fact that anything printed to "standard out" will be shown too. You can write to standard out using println!:

println!("Debug message");
Use braces {} to print the value of a variable or expression:

let secret_number = 4321;
println!("My banking password is {}, which is better than {}.", secret_number, 1200 + 34);
Note that not all types can be printed this way, only the ones that can be presented to users unambiguously. For example, vectors (lists) cannot be printed like this, because it's not clear how a vector should be displayed to users! For these other types, you can tell the println! macro to use debug-formatting with: {:?}. This representation is only intended to be seen by developers. An example:

let words = vec!["hello", "world"];
println!("words that are often combined: {:?}", words);
There is even the handy dbg! macro, which can be used to debug large expressions inline, without having to pick it apart into separate variable declarations.

Assume we have the following expression:

"hello world".split(' ').collect::<Vec<_>>().join("-")
Now we would like to see what the call to collect returns. We could do the following:


Explain
let my_temporary_debug_variable = "hello world".split(' ').collect::<Vec<_>>();
println!("{:?}", my_temporary_debug_variable);
my_temporary_debug_variable.join("-")
But that's tedious! We don't wan't to rewrite our nice expressions just to be able to debug them. Please welcome to the stage: dbg!


Explain
dbg!(
    "hello world".split(' ').collect::<Vec<_>>()
).join("-")
The whitespace is only for clarity. You can wrap any expression in dbg!(), which will print that value (with debug-formatting). Afterwards you can keep using the value in a larger expression, as if the dbg! wasn't even there.

Happy debugging!
*/

//Answers

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

/*
    input.chars() turns our string into an iterator over its characters.
    .rev() reverses this iterator.
    .collect() gathers the characters back into a String.
*/
pub fn run(){
    println!("{}", reverse("stressed"));
    println!("{}", reverse("strops"));
    println!("{}", reverse("racecar"));
    println!("{}", reverse_graphemes("uüu"));  // "üuu"

}

/*
The string "uüu" is tricky because the 'ü' character is actually composed of two Unicode scalars: 'u' 
and the combining diaeresis '̈'. 
When we reverse the string naively, these scalars get separated, messing up the intended characters.
*/

//Add unicode-segmentation

pub fn reverse_graphemes(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true)
        .rev()
        .collect::<String>()
}
