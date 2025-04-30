# auctan

A simple hobby language

# Syntax

```
// Basic Auctan example
print "Hello World!\n";

favorite_food = "Biryani";
print "My favorite food is " + favorite_food + "\n";

// `proc` stands for 'procedure', it is kind of similar to a function, but it is not a function.
// Like functions, it executes a block of code.
// But unlike functions, it does not return a value nor takes arguments.
proc print_hello {
    print "Hello";
}

call print_hello;
print " " + favorite_food + " eater!\n";

// Auctan doesn't support booleans, instead, you use 1 or 0 to represent true or false respectively.
age = 18;

if (age > 17) {
    print "You can drive";
} else {
    print "You can't drive";
}
```
