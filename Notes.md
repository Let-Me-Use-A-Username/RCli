============================
<br>SUBJECTS TO CONSIDER
============================
- pipeline (redirecting data to other destinations)
    - write "this is a nice day" pipeline file.txt 
    - used characters ">" or "|"

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
<br>Coding Notes
============================
<br>
Singlenton: Currently the paths are displayed like the file
explorer would show them. However they way paths are used internally
is canonicalized, meaning we use WIN32 namespaces and so paths
display \\?\ at the beginning.

============================
<br>Path Interpretation
============================
<br>
Local directories have to be specified by using ./ <br>
Parent directories have to be specified by using ../ <br>
Paths that are like ./Path/to/dir/readme will have to be 
interpreted differently based on the operating system. In unix
this will be a file, in windows this will be a directory