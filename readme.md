# RCLI

<p>Command line interface for windows that mimicks Linux/Unix functionality with a little added spice.<br>
This projects aim to be an interoperable command line tool that requires little to no external dependencies.
</p>

---
### Rust CLI objects:
<ul>
    <li> <strong>Input Parser</strong> </li>
	<li> Accepts user input and creates a UserInput struct</li>
	<li> <strong>Lexical Analyzer</strong></li>
	<li> Takes the struct, verifies the BNF syntax (the order of the commands) and creates a token stream</li>
    <li> <strong>Parser</strong></li>
	<li> Takes the stream provided by the lexer. Verified the accepted flags (what command accepts what flag) and creates a token stream that is then recursively parsed to create an Invocator object that is used by the invoker.</li>
    <li> <strong>Invoker</strong></li>
	<li> Accepts the Invocator object provided by the parser and breaks it down to more usable variables. Variables are passed to a "Middleware" that are interpreted by their order (for example cd command accepts a path as the next object but not a flag). The middleware then calls the functions that return a Result which is either a Data type or an error. This result is passed back to the terminal to print it to the user.</li>
	<li> <strong>Window</strong> - Ratatui </li>
	<li> TUI</li>
</ul>

### Basic Commands of RCli
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
</ul>

### Future additions
<ul>
	<li> shortcut </li>
	<li> todo </li>
	<li> ai </li>
	<li> pipeline | or > </li>
	<li> find </li>
</ul>