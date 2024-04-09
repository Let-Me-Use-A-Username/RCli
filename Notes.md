Idea: Lexer will parse the input to see if 
the structure of the command(tree or grammar) is correct. If it is, then it will
be passed to the parser to be executed.

This will be done by using a form of grammar. When inputting the 
command the lexer will check against a grammar to see its validity.
Grammar will have some form of tree Or perhaps context-free?

Note: Perhaps the lexical analyzer will pattern check whilst 
the synactic analyzer will check the structure.

============================
<br>SUBJECTS TO CONSIDER
============================
- pipeline (redirecting data to other destinations)
- write "this is a nice day" pipeline file.txt 


============================
<br>GRAMMAR COMPONENTS
============================
core command -> main issued commands
    - create file
    - move directory
    - etc

sub command -> core command of a sub module
    - ai for example has a command chat, create etc,  to invoke ai chat the core command is ai the    sub command is the command to issue to the previous object
    - sub command could also be a form of pipeline

object -> referenced object of a core or sub command

flag -> core , sub, or object flags
    

============================
<br>BASIC GRAMMAR
============================
Basic grammar syntax example:
(create file -p "C://Desktop")

core command -> object
core command -> object flag
core command -> object flag*

object -> flag* 
(zero to inf)

flag -> {}

============================
<br>NESTED GRAMMAR
============================
Nested grammar syntax example:
(ai chat "string input")

core command -> sub command flag
core command -> sub command flag*

core command -> 

============================
<br>GRAMMAR GOAL
============================
<br>

Supports commands like:
- create readme.txt -p "C://Desktop"
- copy RustProjects/playground RustProjects/otherDirectory

============================
Goal: Ideally the parser will receive a form of "command" to 
execute the order that was given.
The "commands" could be functions that with some generics 
match the core commands specified to the user.
