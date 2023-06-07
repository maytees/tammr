# Monki (tammr lang)

# Grammar

The goal of this grammar is to be simple and concise.

## Keywords
- `let`
- `function`
- `if`
- `else`
- `import`
- `true`
- `false`
- `return`
- `loop`
    - `exit`
- `as`
- `do`
- `not`
- `or`
- `and`
- `is`

## Comments

Comments are simple, you will just need a `//` marker.

```c
// This is a comment, this piece of the source code will not run :)
```

## Variables

Once again, the point of this language is to be simple, so creating variables is going to be simple.

Creating a `variable`

```jsx
let type name = value

OR

let name = value
```

Some things to take away from this: 

- `type` is completely optional and is nothing but a marker for the dev
    - The type is just an identifier for the developer, it does not provide anything for the language like methods/functions. The language will not care what the value of `type` is, it only cares about the name and the value.
    - The language won’t care what `type` is, though if it is a certain marker like `str` , `bool`, or `int` (and more), it will give you a warning if the value does not match the type, it is only a warning and you are *going* to be able to turn that off in the compiler settings.
        - These will not do anything, they will just provide some context to the writer

## Conditionals

Nothing to say here, other than the fact that the language doesn't use parentheses, unlike C styled languages.

```jsx
if condition do

end
```

## Conditionals

Nothing to say here other than he fact that there are no parentheses unlike the `C` family of languages.

```jsx
// Normal If without Else, or Else If
if condition do
	...
end

// If with Else and Else If
if condition do
	...
	else if condition do
		...
	else do
		...
end
```

## Functions

Once more, again, ***this language is supposed to be simple and concise,*** so, functions are going to be very simple to define, and call.

```jsx
									
function funcName(args) do  // funcName - identifier
	...
end             

OR

function(args) do
	...
end funcName      // funcName - identifier

```

Here, you can do either way the first way, both will work the same way, the only difference is where the function name is. This is like typedef in C, for example:

```c
typedef struct thing{
	...
} thing;
```

## Loops

Once more again, ***this language is supposed to be simple and concise, so, loops are very simple***

```jsx
loop do                 // 
  ...                  // Not reccomended
end                   //

OR

loop loopName do      // loopName - identifier
	...                //  ...      - the code
end                 //   End      - End of the scope 

OR

loop loopName if condition do      // loopName - identifier
	...
end

// Exiting a loop
exit loopName                // loopName - identifier
```

To exit a loop, you have to put in `exit {loop name}` inside the loop, loop name is required due to cases of nested loops (`inspired by Rust 🙂`).

You can add If, if you want to check a condition before repeating; if you do not put a condition then you are creating an infinite loop (`unless if you add an exit of course`).

## Importing files

Linking will be something that will be worked on later, as it is not as important currently. Linking in this language will only paste in the content of the file being imported into the file that is importing the file. So, how do you do it? Well, it’s easy, all you have to do is this:

```jsx
import "./path/to/file.tmr" as marker   // content in "" is a path, marker is an identifier

doSomething()     // Function in the marker file
let varInMarker = 50 // Variable in the marker file

OR

marker.doSomething()     // Also a function in the marker file
marker.varInMarker = 50 // Also a function in the marker file
```

You are importing this file and setting it to a variable named marker. You can access things from this marker with either `marker. {something}` or with the direct name of the `something` that you want to access, like the example above.

## Types

Now that we’re done with the *“basic”* things, let’s move on to the *“important”* things, such as types. For every language, there are the basic types, such as ints, booleans, etc. In Tammr, the data types are not something the *“author”* of the program should worry about, its more of something that the language needs to worry about. There are few basic data types in this language, and they consist of:

- `str`
- `int`
- `float`
- `boolean`
- `arr`
- `module`
- `hash`

Now, lets explain each one

- `str` is a string, a sequence of characters
- `int` is a number, specifically, a whole number, no decimals
- `float` is a number, specifically, a number with decimals, or no decimals, though defining a float requires a decimal
- `boolean` is simple, it is either `True` or `False`, can also be `0` or `1`
- `arr` is an array (more on that later)
- `module` probably a file, it is the variable that a file gets put into when using `Import as`
- `hash` key value, like json, or dict in python.

## Blocks

`Blocks` in this language are really just what would be scopes in other languages.

Code example:

```jsx
import std as std

function() do
	println("This print function is inside this function block")

	if true do
		println("This is a block inside a block")
	end
end printInfo

if true do
	println("This is true and it is in this condition block")
end
```

Do you recognize a pattern? As you can see, every block is started with `do` and is ended with `end`.  

Here is a visualizer to see the file’s blocks. Keep in mind that the file itself is a block.

```jsx
 // Main Block
||
||   // Function Block
||  ||   
||  ||  // Condition Block
||  || ||
||  ||  \\ Condition Block
||  ||
||   \\ Function Block
||
||
||    // Condition Block
||   ||
||   ||
||    \\ Condition Block
||
 \\ Main Block
```

## Examples
