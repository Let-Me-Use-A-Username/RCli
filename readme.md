# RCLI

<p>Command line interface for windows that mimicks Linux/Unix functionality.<br>
This projects aims to be an interoperable command line tool that requires little to no external dependencies.
</p>

---
### Rust CLI objects:
<ul>
    <li> <strong>Input Parser</strong> </li>
	Accepts user input and creates a UserInput struct
	<br></br>
	<li> <strong>Lexical Analyzer</strong></li>
	Takes the struct, verifies the BNF syntax (the order of the commands) and creates a token stream</li>
	<br></br>
    <li> <strong>Parser</strong>
	Takes the stream provided by the lexer. Verified the accepted flags (what command accepts what flag) and creates a token stream that is then recursively parsed to create an Invocator object that is used by the invoker.
	<br></br>
    <li> <strong>Invoker</strong></li>
	Accepts the Invocator object provided by the parser and breaks it down to more usable variables. Variables are passed to a "Middleware" that are interpreted by their order (for example cd command accepts a path as the next object but not a flag). The middleware then calls the functions that return a Result which is either a Data type or an error. This result is passed back to the terminal to print it to the user.
</ul>

### Basic Commands
<ul>
	<li> cwd </li>
	<li> touch </li>
	<li> mkdir </li>    
    <li> remove </li>
	<li> copy </li>
    <li> move </li>
    <li> read </li>
    <li> list </li>
    <li> cd </li>
	<li> grep </li>
	<li> find </li>
	<li> pipeline</li>
</ul>

### Future additions
<ul>
	<li> shortcut / alias</li>
</ul>