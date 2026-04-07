## Variables  
There are 3 types of variables, **strings**, **booleans** and **numbers**.  
  
### Numbers  
```rockscript
throw 12 rocks at x
```  
pseudocode:  
```rockscript
let x = 12
```  
You can replace `rocks` with `rock`, which is preferred when the value is 1. Example:  
```rockscript
throw 1 rock at y
```  
  
### Strings  
```rockscript
throw rock named "Hello world!" at x
```  
pseudocode:  
```rockscript
let x = "Hello world!"
```  
You are technically able to replace `rock` with `rocks`, but you shouldn't do so. Rockscript should always look gramatically correct.  
  
### Booleans  
```rockscript
throw big rock at x
throw small rock at y
```  
pseudocode:  
```rockscript
let x = true
let y = false
```  
  
## Printing  
You can print expressions using the `present` keyword. Examples:  
  
```rockscript
present 12
```  
*output:* `12`  
  
```rockscript
throw rock named "Hello world!" at x
present x
```  
*output:* `Hello world!`  
  
## Arithmetic 
The only way of performing Arithmetic is by mutating a variable. These are all possible operations:  
  
### Addition  
```rockscript
throw 5 rocks at x
smash 2 into x
present x
```  
*output:* `7`  
  
```rockscript
throw 3 rocks at x
throw 2 rocks at y
smash y into x
present x
```  
*output:* `5`  
  
Addition also works on strings, which will concatenate them:  
```rockscript
throw rock named "Hello" at x
smash "World" into x
present x
```  
*output:* `HelloWorld`  
  
### Subtraction  
```rockscript
throw 4 rocks at x
chip 3 off x
present x
```  
*output:* `1`  
  
### Multiplication  
```rockscript
throw 5 rocks at x
throw 6 rocks at y
mate y with x
present x
```  
*output:* `30`  
  
You are also able to multiply strings with numbers, resulting in a repeated string:  
```rockscript
throw rock named "rock " at x
mate 3 with x
present x
```  
*output:* `rock rock rock `  
  
### Division  
```rockscript
throw 12 rocks at x
throw 3 rocks at y
split y from x
present x
```  
*output:* `4`
  
## Functions  
Functions are defined using `carve instruction into`:
```rockscript
carve instruction into foo
    throw 5 rocks at x
enough
```  
pseudocode:  
```rockscript
fn foo() {
    let x = 5
}
```  
The `enough` keyword is essentially the same as a closing bracket, which tells the interpreter where the function ends.  
  
You're able to access arguments with the `retrieve` keyword:  
```rockscript
carve instruction into foo
    retrieve bar
enough
```  
pseudocode:  
```rockscript
fn foo(bar) {

}
```  
  
After retrieving the argument, you can use it like you would with any other variable:  
```rockscript
throw 5 rocks at x

carve instruction into y
    retrieve z
    smash z into x
enough
```  
  
You can return a value from a function with `engrave`:  
```rockscript
carve instruction into foo
    engrave "Hello world!"
enough
```  
pseudocode:  
```rockscript
fn foo() {
    return "Hello world!"
}
```  
  
To call functions, use `follow`:  
```rockscript
carve instruction into foo
    engrave 5
enough

present follow foo
```  
*output:* `5`  
  
Pass arguments into the function with `with` and `and`:  
```rockscript
carve instruction into x
    retrieve y
    smash 5 into y
    engrave y
enough

present follow x with 3
```  
*output:* `8`  

```rockscript
carve instruction into add
  retrieve x
  retrieve y
  smash x into y
  engrave y
enough

present follow add with 5 and 6
```
*output:* `11`  
  
### Comparisons  
A comparison evaluates to either big or small. These are all comparisons:  
  
## Weighing  
By weighing a value against another, you can see which is heavier:  
```rockscript
throw 3 rocks at x
throw 2 rocks at y
present weigh x against y
```  
*output:* `big`  
Pseudocode:  
```rockscript
let x = 3
let y = 2
print(x >= y)
```  
  
## If statements  
```rockscript
inspect big
    present "Hello world"
enough
```  
*output:* `Hello world`  
Pseudocode:  
```rockscript
if (true) {
    print("Hello world")
}
```  
  
```rockscript
throw 4 rocks at x
throw 5 rocks at y

inspect weigh x against y
    present "x >= y"
refine
    present "x < y"
enough
```  
*output:* `x < y`  
Pseudocode:  
```rockscript
let x = 4
let y = 5

if (x >= y) {
    print("x >= y")
} else {
    print("x < y")
}
```  
