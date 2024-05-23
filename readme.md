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
	<li> Takes the struct, verified the BNF (the order of the commands) syntax and creates a token stream</li>
    <li> <strong>Parser</strong></li>
	<li> Takes the stream, verified the grammar (what command accepts what flag etc) and creates a token stream understood by the invoker.</li>
    <li> <strong>Invoker</strong></li>
	<li> Modified the stream into more generic types and calls commands that "get the job done"</li>
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