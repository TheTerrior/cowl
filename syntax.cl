//this is a comment

#this is also a comment

''' this is a multiline comment

and it can go however long you want'''

/* this is also a multiline comment

so you can do python or c type commentation

*/

var x #this declares a new variable as mutable and untyped

all variables can be declared or simply thrown into use on any line
all variables must be declared using 'var' or any variable type:
    integer, array, list, number, function, class, string, tuple, dictionary, enum, boolean

some variable types have nicknames:
    function:func, string:str, tuple:tup, dictionary:dict, integer:int, number:num, boolean:bool

some variable types can be declared in other ways:

function x(parameter, num parameter) {
    //inner code
}

it is not necessary to declare a variable's type. If the type is unknown or you'd rather the variable be untyped, you can do this:
    var x
in this case, the variable will be dynamically typed during runtime
variables declared in this way:
    int x
will hold onto their type and throw any errors if a value different from its type is attempted to be saved to it
any variables declared as dynamic or a certain type will remain that way for the remainder of the program, cannot change its dynamicicity

it is not necessary to declare a variable at all, in which case it will be automatically declared as dynamic, like this:
if (y == 0) {
    //code
}
any variable used before declaration will be set to dynamic, and its value will be null

lists: [x, y , z]           //completely mutable, all values internally can be changed, and they can all be of any type
dictionaries: {x:y, y:z}    //mutable, keys must be ints, nums, tuples, or strings
tuples: (x, z, o, y, l)     //immutable, whatever the values are set to, it will remain that way until the tuple is re-assigned


OPERATIONS:
Addition: x + y //as well as all other basic stuff like division, and modulus (%), exponents (^)
other conventions: x += y, x++, x--, -x, --x (okay this one is important, the two negatives cancel out and are as if neither were there to begin with)
NUM / NUM = NUM
NUM / INT = NUM
INT / NUM = NUM
INT / INT = INT //integer division




MORE DETAILS:
c++ convention says if a line where a { is expect and the program instead finds a statement or anything after, a {} is not required
ex:
    if (x > 0) return "hi"
this is valid

functions do not need to specify what values they will return, if any, just like python. A function can also return different things on different return statements
ex:
func f(int x) { #function that can return an integer or a tuple of two integers
    if (x > 0) return x
    elif (x < 0) return -x
    else return (x, -x)
}

parentheses will always be required around places where a statement is expected


PYTHONIZATION:
if any line where a { is expected, the program finds a :, then it will pythonize that statement
for example:
if (x > 0):
    print(this is valid)

oddly enough, you can also mix these if you'd like
if (x > 0):
    if (type(x) == Integer) {
        print("bruh this literally works lmao)
    }
print("and now we're back down here")

this can also work in combination with c++ convention
example:
    if (x > -1): print("will not pythonize after this line")


NOTE ABOUT NUMBER:
number is a special type, which internally is simply a float-like number. It can hold integers, whole numbers, etc
integers can be saved into a number, which will automatically be converted into a number

NOTE ABOUT STRINGS:
internally, strings will be saved as lists of characters. Python does not have characters, so the interpreter will make use of single-length strings
this classification of strings as lists means all operations that can be done on strings can be done on lists and vice versa
strings are mutable, since they are lists, so doing
    "hello"[2] = 'm'
results in
    "hemlo"

since we're working with python, "" and '' can be used to represent strings, nothing weird about it
internally, we can't just convert a string into a string, we must interpret it one by one and create a list

strings will have special functions that lists do not, such as .upper() and .lower() etc


POINTERS
all data will be saved in a special way as to allow for the use of "pointers"
when you pass a variable into a function, its value will be passed, not a link to that value
for example, in python, lists are transferred by address, however in Cowl, that will not be the case
to pass any value by pointer, the user will have to put * after the variable name/value
example:
    function(myvariable*)
this is not allowed though:
    function(3*)
it would make no sense to do that, since 3 is not a variable and cannot be saved as one
now this is a good example of using previous info:
    function(undeclaredvariable*)
what this line will do is it will declare a new var called undeclaredvariable, and give it the value null, and then pass its reference to function


IMPORTING
in the background, imported functions will be saved as variables prior to running the opened script
ie any code will run after the importation of the imported files