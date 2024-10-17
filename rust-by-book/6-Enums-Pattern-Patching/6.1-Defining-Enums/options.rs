// enums don't have any types for nulls, but the null state can be implemented and represented
// using Options as below:

enum Option<T> { //its included in prelude, you don't need to bring it into scope explicitly.
    None,
    Some(T),
}
// we can use None and Some directly without prefixing with Option::
// <T> syntax is a generic type parameter.
// it means 'Some' variant of the Option enum can hold one piece of data of any type
// each concrete type that gets used in place of T makes the overall Option<T> have a different
// type. Ex - 
let some_number = Some(5); // type of Option<i32>
let some_char = Some('e'); // type of Option<char>
let absent_number: Option<i32> = None; // absent_number to be of type Option<i32>, as rust compiler
                                       // can't infer the type of the corresponding Some variant
                                       // will hold by looking only at None value.
// why is it better to use Option<T> than having null type?
// because Option<T> and T are different types, the compiler won't let us use an Option<T> value as
// if it were definitely a valid value.
// Ex -
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
// this code won't compile some we are trying to add i8 to an Option<i8>
// when we have a value of type i8 in rust, the compiler will ensure that we always have a valid
// value.
// Only when we have Option<i8>, do we need to worry about possibly not having a value and compiler
// will make sure we handle the case before using the value! Nice
// i.e. convert Option<T> to T before performing T operations with it.
