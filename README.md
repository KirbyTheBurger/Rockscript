Rockscript is an esolang made by me. Below is some documentation and code examples.  
  
## Variables  
There are 2 types of variables, **strings** and **numbers**.  
  
**Number variables**  
```throw 12 rocks at x```  
is the equivalent of  
```let x = 12```  
You can replace `rocks` with `rock`, which is preferred when the value is 1. Example:  
```throw 1 rock at y```  
  
**String variables**  
```throw rock named "Hello world!" at x```  
is the equivalent of  
```let x = "Hello world!"```  
You are technically able to replace `rock` with `rocks`, but you shouldn't do so. Rockscript should always look gramatically correct.  
  
## Printing  
You can print expressions using the `present` keyword. Examples:  
  
```present 12```  
*output: `12`*  
  
```
throw rock named "Hello world!" at x
present x
```  
*output: `Hello world!`*  
  
## Binary operations  
The only way of performing binary operations is by mutating a variable. These are all possible operations:  
  
**Addition**  
```
throw 5 rocks at x
smash 2 into x
present x
```  
*output: `7`*  
  
```
throw 3 rocks at x
throw 2 rocks at y
smash y into x
present x
```  
*output: `5`*  
  
Addition also works on strings, which will concatenate them:  
```
throw rock named "Hello" at x
smash "World" into x
present x
```  
*output: `HelloWorld`*  
