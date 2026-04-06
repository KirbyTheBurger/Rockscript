Rockscript is an esolang made by me. Below is some documentation and code examples.  
  
## Variables  
There are 3 types of variables, **strings**, **boleans** and **numbers**.  
  
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
  
**Strings**  
```rockscript
throw rock named "Hello world!" at x
```  
pseudocode:  
```rockscript
let x = "Hello world!"
```  
You are technically able to replace `rock` with `rocks`, but you shouldn't do so. Rockscript should always look gramatically correct.  
  
**Booleans**  
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
*output: `12`*  
  
```rockscript
throw rock named "Hello world!" at x
present x
```  
*output: `Hello world!`*  
  
## Binary operations  
The only way of performing binary operations is by mutating a variable. These are all possible operations:  
  
**Addition**  
```rockscript
throw 5 rocks at x
smash 2 into x
present x
```  
*output: `7`*  
  
```rockscript
throw 3 rocks at x
throw 2 rocks at y
smash y into x
present x
```  
*output: `5`*  
  
Addition also works on strings, which will concatenate them:  
```rockscript
throw rock named "Hello" at x
smash "World" into x
present x
```  
*output: `HelloWorld`*  
  
**Subtraction**  
```rockscript
throw 4 rocks at x
chip 3 off x
present x
```  
*output: `1`*  
  
**Multiplication**  
```rockscript
throw 5 rocks at x
throw 6 rocks at y
mate y with x
present x
```  
*output: `30`*  
  
You are also able to multiply strings with numbers, resulting in a repeated string:  
```rockscript
throw rock named "rock " at x
mate 3 with x
present x
```  
*output: `rock rock rock `*  
  
**Division**  
```rockscript
throw 12 rocks at x
throw 3 rocks at y
split y from x
present x
```  
*output: `4`*
