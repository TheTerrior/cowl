import os
import sys
import enum
from functools import reduce

class Vartype(enum.Enum):
    #Mutable = 0
    #Mut = 0
    Boolean = 1
    Bool = 1
    Integer = 2
    Int = 2
    Number = 3
    Num = 3
    Tuple = 4
    Tup = 4
    List = 5
    String = 6
    Str = 6
    Dictionary = 7
    Dict = 7
    Function = 8
    Func = 8
    Fn = 8
    Class = 9

#########################################################
#                       VARIABLES                       #
#########################################################

#parent class of all variables
class Variable():
    type = None
    value = None

    def __init__(self, val):
        #val will always be given as a string
        pass

class Boolean(Variable):

    def integrity(self):
        if self.value == True or self.value == False:
            return True
        return False
        
class Integer(Variable):

    def integrity(self):
        try:
            self.value = int(self.value) #if originally a float, turn it into an integer
            return True
        except:
            return False

class Number(Variable):

    def integrity(self):
        try:
            float(self.value) #don't set value to the result
            return True
        except:
            return False

class Tuple(Variable):

    def integrity(self):
        if isinstance(self.value, tuple):
            return True
        return False
        
class List(Variable):

    def integrity(self):
        if isinstance(self.value, list):
            return True
        return False

    '''
    methods:

    length, reverse, append [to end], pop(index), insert(index, value),
    contains(value), find(value), rfind(value), copy


    other:

    allow the use of []
    single integer means retrieve item at that index
    negative integer means subtract that index from length of string, -1 is final item in list
    allow the use of : in []
    single colon means return the entire list (returns reference, unlike .copy())
    integer before means index from start of list until the second integer
    integer after means index from first index until end of list
    all of these will return a reference to the sublist or item in list
    
    
    
    '''
        
class String(List):

    def integrity(self): #a string must be a list of characters
        return (super().integrity() and reduce((lambda x, y: type(y) == str and len(y) == 1 and x), self.value, True))
        
class Dictionary(Variable):

    def integrity(self):
        if isinstance(self.value, dict):
            return True
        return False

class Function(Variable):

    def integrity(self):
        if isinstance(self.value, str):
            return True
        return False
        
class Class(Variable):

    def integrity(self):
        pass


###########################################################
#                       INTERPRETER                       #
###########################################################

'''
TODO

walrus operator: :=
    assigns a value to a variable, can do so in any statement anywhere really, but most often in a loop or conditional

assigning more than one variable at a time: (x, y) = f(0)
    requires f to return a tuple, (x, y) will be read as a parameter tuple
    NOTE: assignment operator will look for value before and after, function will accept both
        the first parameter will allow tuples, such as this one

lists should be able to span multiple lines, along with dictionaries and tuples

""" should allow multiline strings, triple apostrophe too

allow the use of references, clarify how references work

univeral functions:
    length
    int
    string
    list
    tuple
    dictionary
    number
    bool
    pointer


'''

class Lexer():

    #tokens that could be written within a variable name
    STRINGTOKENS = ['if', 'else', 'elif', 'while', 'for',
        'boolean', 'bool', 'integer', 'int', 'number', 'num', 'tuple', 'tup',
        'list', 'string', 'str', 'dictionary', 'dict', 'function', 'func', 'fn', 'class']

    #tokens that (as long as they're not in a string), will always be the operator they represent
    OPERATORTOKENS = ['+', '-', '/', '*', '%', '^', '(', ')', '[', ']', '{', '}', "'", '"',
        '=', '==', '>', '<', '>=', '<=', '!', '!=', ':=', ',', '&']

    '''
    GENERAL RULES

    any buffer that starts with an integer will always be some sort of number
    '''

    def is_token(self, buffer):
        #having whitespace at the end means the token is finalized in terms of the code
        #no whitespace means we need to be careful about certain possibilities, like 'if'
        if buffer[len(buffer) - 1] in [' ', '\n']:
            if 

    def lex(self, input):
        #input is given as a file, which will be interpreted word by word
        with open(input, "r") as file:
            #read file char by char
            buffer = "" #where chars will be placed if they fit no token
            cur = file.read(1)
            tokens = []
            while cur != "": #while we haven't reached the end of the file
                buffer += cur
                #check if buffer matches any token
                if token := self.is_token(buffer): #if a value was returned, clear buffer
                    buffer = ""
                    tokens.append(token)

class Parser():

    def __init__(self):
        pass

class Interpreter():

    def __init__(self, file):
        #file is the Cowl file that we're opening with this interpreter
        pass

    def operation(self, operation, operand, operand1 = None):
        if operation == "-": #negate
            if operand.type.value == 1: #Boolean
                if operand.value:
                    return
                pass
        pass