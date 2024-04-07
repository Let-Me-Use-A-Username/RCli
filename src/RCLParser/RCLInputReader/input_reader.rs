pub struct UserInput{
    vectorInput: Vec<String>,
    vectorLength: usize,
    coreCommand: String,
    subCommands: Vec<String>,
    peekIndex: usize,
    consumeIndex: usize
}


//Accepts user input and vectorizes
fn acceptInput(input: String) -> UserInput{
    let mut input_parts: Vec<&str> = input.split(' ').collect();
    //EOF character is "?"
    input_parts.push("?");

    let size = input_parts.len();

    if size < 1 {
        panic!("ERROR! No arguments provided")
    }

    let main: String = input_parts[0];
    let rest: Vec<String> = input_parts[1..];

    let string_parts: Vec<String>::from() = input_parts.iter().map(|v| String::from(v)).collect::<Vec<String>>();

    return UserInput {
        vectorInput:string_parts, 
        vectorLength:size, 
        coreCommand:main, 
        subCommands:rest, 
        peekIndex:0, 
        consumeIndex:0};
}

//Peeks the vector at a certain index
fn peek(obj: UserInput, index: usize) -> Result<String, &'static str>{
    if index < obj.vectorLength{
        return Ok(obj.vectorInput[index])
    }
    return Err("ERROR: Unable to peek.");
}

//Peeks next character
fn peek_next(obj: UserInput) -> Result<String, &'static str>{
    let obj_index = obj.peekIndex + 1;

    if obj_index < obj.vectorLength{
        obj.peekIndex = obj_index;
        return  Ok(obj.vectorInput[obj_index]);
    }
    return Err("ERROR: Unable to peek next");
}

//Consumes one command at a time
fn consume(obj: UserInput) -> Result<String, &'static str>{
    let cIndex = obj.consumeIndex;
    let vLength = obj.vectorLength;

    if cIndex < vLength{
        cIndex += 1;
        return Ok(obj.vectorInput[cIndex]);
    }
    return Err("ERROR: Unable to consume");
}

//Checks for EOF
fn isEOF(character: String) -> bool{
    if character == "?"{
        return true;
    }
    return false;
}