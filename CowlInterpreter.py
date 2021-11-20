import os
import sys
import enum

class Vartype(enum.Enum):
    Mutable = 0
    Mut = 0
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
    Enum = 10

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
        if type(self.value) == tuple:
            return True
        return False
        
class List(Variable):

    def integrity(self):
        if type(self.value) == list:
            return True
        return False
        
class String(Variable):

    def integrity(self):
        if type(self.value) == list:
            return True
        return False
        
class Dictionary(Variable):

    def integrity(self):
        if type(self.value) == Dictionary:
            return True
        return False

class Function(Variable):

    def integrity(self):
        
class Class(Variable):

    def integrity(self):
        
class Enum(Variable):

    def integrity(self):



class Interpreter():

    def __init__(self, file):
        #file is the Cowl file that we're opening with this interpreter
        pass

    def operation(self, operation, operand, operand1 = None):
        if operation == "-": #negate
            if operand.type.value = 1: #Boolean
                if operand.value:
                    return
                pass
        pass