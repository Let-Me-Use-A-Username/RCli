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
    - update file/directory<br>
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
<br>SYNTAX
============================
(list --hidden)
(directory is the current working dir)
core -> flag

(create "read.me" -p "C://Dekstop")
core -> object -> flag

(ai input -p "path/to/file")
core -> sub -> flag

(ai input "path/to/file" -f json)
core -> sub -> object -> flag

(create "read.me")
core -> object<br>

(create "read.me" -p 111) (read write execute)
core -> object flag*

(copy "content.txt" > "other_file")
core -> sub 
or
copy -> object -> sub -> object

============================
<br>ADVANCED GRAMMAR
============================
Advanced grammar syntax examples:

(ai chat -m Ultra -i "string input")
core -> sub -> flag -> flag

(create project "project name" -w rust, gradle)
core -> flag


============================
<br>GRAMMAR GOAL
============================
<br>

Goal: Ideally the parser will receive a form of "command" to 
execute the order that was given.
The "commands" could be functions that with some generics 
match the core commands specified to the user.
