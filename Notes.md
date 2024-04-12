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
    - used characters ">" or "|"


============================
<br>GRAMMAR COMPONENTS
============================
core command = main issued commands<br>
    - create file/directory<br>
    - delete file/directory<br>
    - copy file/directory<br>
    - move file/directory<br>
    - read file<br>
    - list directory<br>

sub command = core command of a sub module<br>
    - ai for example has a command chat, create etc,  to invoke "ai chat" the core command is "ai" the sub command is "chat"<br>
    - sub command could also be a form of pipeline

object = referenced object of a core or sub command<br>
    - File<br>
    - Directory

flag = core , sub, or object flags<br>
    - path
    


============================
<br>BNF SYNTAX
============================
BASIC INVOCATION:

CREATE/DELETE:
	1) create C://Desktop/readme.txt
	2) create readme.txt
		<command> ::= <core> <object>
LIST:
	1)list "C://Desktop"
		<command> ::= <core> <object>
	2)list --hidden
	    <command> ::= <core> <object> <flag> :: object here is the current directory

COPY:
	1)copy readme.txt -p "C://Desktop/readme.txt"
    2)copy "C://Desktop/readme.txt" -p "C://Files"
		<command> ::= <core> <object> <flag>
		
MOVE:
	1)move readme.txt -p "C://Desktop"
		<command> ::= <core> <object> <flag>
	2)move "C://Desktop/readme.txt" -p "C://Files"	
		<command> ::= <core> <object> <flag>


READ
	1)read readme.txt
		<command> ::= <core> <object>


ADVANCED INVOCATION:
1)ai chat -m Ultra
	<command> ::= <core> <sub> <flag>
2)ai read "C://Desktop/readme.txt" -t json -m Ultra
    <command> ::= <core> <sub> <object> <flag> <flag>
3)todo show_week
	<command> ::= <core> <sub>


============================
<br>GRAMMAR GOAL
============================
<br>

Goal: Ideally the parser will receive a form of "command" to 
execute the order that was given.
<br>
The "commands" could be functions that with some generics 
match the core commands specified to the user.
