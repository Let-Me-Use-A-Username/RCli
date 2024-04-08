Idea: Lexer will parse the input to see if 
the structure of the command is correct. If it is, then it will
be passed to the parser to be executed.

This will be done by using a form of grammar. When inputting the 
command the lexer will check against a grammar to see its validity.
Grammar will have some form of tree.
Or perhaps context-free?

Note: Perhaps the lexical analyzer will pattern check whilst 
the synactic analyzer will check the structure.

============================
Grammar example:
core command -> object
core command -> object flag
core command -> object flag*

object -> flag* 
(zero to inf)

flag -> {}
============================

Goal: Ideally the parser will receive a form of "command" to 
execute the order that was given.
The "commands" could be functions that with some generics 
match the core commands specified to the user.
