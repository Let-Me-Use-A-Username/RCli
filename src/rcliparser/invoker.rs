use std::collections::HashMap;
use std::fs::{self, DirBuilder, DirEntry, OpenOptions};
use std::io::{self, Error, ErrorKind};
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::rcliparser::objects::grammar_objects::CommandType;
use crate::rcliparser::objects::token_objects::GetValue;
use crate::rcliparser::utils::windows_file_attributes;
use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::data_types::Data;
use super::objects::grammar_objects::FlagType;
use super::objects::token_objects::{InvocationToken, Token, TokenObject};


pub fn invoke(core: InvocationToken, object: Token, mut flags: HashMap<FlagType, Option<TokenObject>>, terminal_instance: &mut Terminal) -> Result<Data, Error>{

    //Prints for debug purposes
    // println!("\nCORE: {:?}", core.clone());
    // println!("OBJECT: {:?}", object.clone());
    // println!("FLAGS: {:?}", flag_vector.clone());

    //Object extraction
    let object_value = object.get_value();
    let path_value = Path::new(&object_value);
    

    let mut operation_status: Result<Data, Error> = Ok(Data::StatusData(0));

    match core.get_type(){
        CommandType::HOME => {
            operation_status = home(terminal_instance);
        },
        CommandType::CWD => {
            operation_status = cwd(terminal_instance);
        },
        CommandType::TOUCH => {
            operation_status = touch(path_value);
        },
        CommandType::MKDIR => {
            println!("flags {flags:?}");
            let recursive = flags.get(&FlagType::DESTINATION);
            if recursive.is_some(){
                operation_status = mkdir(path_value, true);
            }
            else{
                operation_status = mkdir(path_value, false);
            }
            
            
        },
        CommandType::REMOVE => todo!(),
        CommandType::COPY => todo!(),
        CommandType::MOVE => todo!(),
        CommandType::READ => todo!(),
        CommandType::LIST => todo!(),
        CommandType::CD => {
            operation_status = traverse_directory(path_value, terminal_instance);
        },
        CommandType::GREP => todo!(),
        CommandType::EXIT => todo!(),
        CommandType::INVALID => todo!(),
    }
    
    if operation_status.is_err(){
        handle_error(&operation_status.as_mut().err().unwrap());
    }
    
    return operation_status
}


fn handle_error(error: &Error){
    match error.kind(){
        ErrorKind::PermissionDenied => {
            eprintln!("ERROR HANDLER: Permission denied.")
        },
        ErrorKind::NotFound => {
            eprintln!("ERROR HANDLER: Object not found.")
        },
        ErrorKind::AlreadyExists => {
            eprintln!("ERROR HANDLER: Object already exists.")
        },
        ErrorKind::InvalidInput => {
            eprintln!("ERROR HANDLER: Invalid input.")
        },
        ErrorKind::Other => {
            eprintln!("ERROR HANDLER: Other error encountered.")
        }
        _ => {
            eprintln!("ERROR HANDLER: Unknown error occured.")
        }
    }

}

///Shows the users home directory. Returns path.
fn home(terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let home_dir = terminal_instance.get_home_directory();
    println!("{}", home_dir.display().to_string());
    Ok(Data::PathData(home_dir))
}


///Shows current working dir. Returns path.
fn cwd(terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let current_path = terminal_instance.get_current_directory();
    println!("{}", current_path.display().to_string());
    Ok(Data::PathData(current_path))
}


///Creates a file at the given path. Returns file.
fn touch(file_path: &Path) -> Result<Data, Error>{
    //could not need the open() clause unless pipelining
    let file = OpenOptions::new().write(true).read(true).create(true).open(file_path);

    match file{
        Ok(_) => {
            return Ok(Data::FileData(file.unwrap()));

        },
        Err(error) => {
            return Err(error);
        }
    }
}


///Creates a directory at the given path. Returns path to top level directory.
fn mkdir(path: &Path, recursive: bool) -> Result<Data, Error>{
    let mut builder = DirBuilder::new();
    builder.recursive(recursive);
    
    let directory = builder.create(path);

    match directory{
        Ok(_) => {
            return Ok(Data::PathData(path.to_path_buf()));
        },
        Err(error) => {
            return Err(error);
        }
        
    }
}


///Removes a file or dir. Returns path of removed object.
fn remove(path: &Path, recursive: bool) -> Result<Data, Error>{
    let mut res : Result<(), Error> = Result::Err(Error::new(ErrorKind::NotFound, "INTERNAL ERROR: Object doesn't exist."));
    if path.try_exists().is_ok(){
        if path.is_dir() & recursive{
            res = fs::remove_dir_all(path);            
        }
        else if path.is_dir(){
            res = fs::remove_dir(path);
        }
        else if path.is_file(){
            res = fs::remove_file(path);
        }
    }

    match res{
        Ok(_) => {
            return Ok(Data::PathData(path.to_path_buf()))
        },  
        Err(error) => {
            return Err(error);
        },
    }
}


///Copies the content of either a file or a directory. This is move powerful over move
///due to the fact that it can copy without requiring permissions (on Windows) whereas 
///move requires.
///Returns either the destination file or directory.
fn copy(path: &Path, destination: &Path, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let path_exists = path.try_exists()?;
    let dest_exists = destination.try_exists()?;

    //Destination doesn't exist, so copy the content to a new file/directory
    if path_exists & !dest_exists{
        if path.is_file(){
            match fs::copy(path, destination){
                Ok(_) => {
                    return Ok(Data::PathData(destination.to_path_buf()))
                },
                Err(error) => {
                    return Err(error)
                },
            }
        }
        else if path.is_dir(){
            return copy_dir(path, Some(destination));
        }
        //tricky clause.
        else{
            Err(Error::new(ErrorKind::Other, "INTERNAL ERROR: Path not recognized as a file or a directory."))
        }
    }
    //Destination does exists
    else if path_exists & dest_exists{
        if path.is_file() & destination.is_file(){
            //todo! this overrites the content of to
            //add flag to merge.. ?
            match fs::copy(path, destination){
                Ok(_) => {
                    return Ok(Data::PathData(destination.to_path_buf()))
                },
                Err(error) => {
                    return Err(error)
                },
            }
        }
        //create the from dir to destination dir
        else if path.is_dir() & destination.is_dir(){
            let mut destination_path = terminal_instance.get_current_directory().join(destination);
            destination_path.push(path.components().last().unwrap().as_os_str());
            
            return copy_dir(path, Some(destination_path.as_path()));
        }
        else if path.is_file() & destination.is_dir(){
            //create new path
            let mut file_path = terminal_instance.get_current_directory().join(destination);
            file_path.push(path.components().last().unwrap().as_os_str());

            //create file in destination
            let _ = OpenOptions::new().write(true).read(true).create(true).open(file_path.clone())?;
            
            //copy data
            match fs::copy(path, file_path.clone()){
                Ok(_) => {
                    return Ok(Data::PathData(file_path))
                },
                Err(error) => {
                    return Err(error)
                },
            }
        }
        else{
            Err(Error::new(ErrorKind::InvalidData, "INTERNAL ERROR: Unknown objects."))
        }
    }
    else{
        Err(Error::new(ErrorKind::NotFound, "INTERNAL ERROR: Path |{path:?}| doesn't exist."))
    }
}

///Moves and renames a file or directory.
///If destination doesn't exist it renames a file. If it does exist it copies it.
///Returns destination path.
fn r#move(path: &Path, destination: &Path, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let path_exists = path.try_exists()? | path.exists();
    let dest_exists = destination.try_exists()?;
    //simple rename
    if path_exists & !dest_exists{
        match fs::rename(path, destination){
            Ok(_) => {
                return Ok(Data::PathData(destination.to_path_buf()))
            },
            Err(error) => {
                return Err(error)
            },
        }
    }
    //both paths exists so let copy handle it.
    else if path_exists & dest_exists{
        return copy(path, destination, terminal_instance);
    }
    else{
        Err(Error::new(ErrorKind::NotFound, "INTERNAL ERROR: Path |{path:?}| doesn't exist."))
    }
}

///Reads the content of a file to terminal
///Returns content as string.
fn read(path: &Path) -> Result<Data, Error>{
    if path.exists() | path.try_exists()?{
        if path.is_dir(){
            return Err(Error::new(ErrorKind::InvalidInput, "INTERNAL ERROR: Cannot read directory. Use ls instead."))
        }
        let file = OpenOptions::new().write(true).read(true).open(path)?;

        match io::read_to_string(file){
            Ok(content) => {
                println!("{content}");
                return Ok(Data::BufferedStringData(content))
            },
            Err(error) => {
                return Err(error)
            },
        }
        
    }
    return Err(Error::new(ErrorKind::NotFound, "INTERNAL ERROR: Invalid path."))
}


///Lists items in a directory
fn list(dir_path: &Path, hidden: bool) -> Result<Data, Error>{
    let mut outputbuffer: Vec<String> = vec![];
    let mut entrybuffer: Vec<DirEntry> = vec![];

    match fs::read_dir(dir_path) {
        Ok(paths) => {
            for path in paths{
                entrybuffer.push(path.unwrap());

                let dir_path = entrybuffer.last().clone().unwrap().path();

                match fs::metadata(dir_path.clone()) {
                    Ok(meta) => {
                        let attributes = meta.file_attributes();

                        let entry_attributes = windows_file_attributes::match_attributes(attributes);
                        
                        //if hidden is true show everything
                        if hidden{
                            outputbuffer.push(dir_path.display().to_string().replace("\\\\?\\", ""));
                        }
                        //else hidden flag is false. if dir DOESNT have hidden flag append it
                        else if !hidden & !entry_attributes.contains(&windows_file_attributes::WindowsAttributes::HIDDEN){
                            outputbuffer.push(dir_path.display().to_string().replace("\\\\?\\", ""));
                        }

                    },
                    Err(error) => {
                        eprintln!("INTERNAL ERROR: Couldn't read object metadata {error:?}");
                        return Err(error)
                    }
                }; 
            }

            for obj in outputbuffer{
                println!("{}", obj);
            }
    
            return Ok(Data::DirVecData(entrybuffer))
        },
        Err(error) => {
            return Err(error)
        },
    }
}


///Traverses given path if valid
fn traverse_directory(path: &Path, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);
    
    match terminal_instance.set_current_directory(pathbuffer) {
        Ok(status) => {
            Ok(Data::StatusData(status))
        },
        Err(error_path) => {
            let error = Error::new(ErrorKind::InvalidData, error_path.display().to_string());
            return Err(error)
        },
    }
}

///For given data returns match.
///Data can be either a dir or a stream.
///Therefore matches are either files or Strings.
fn grep(path: &Path, regex_string: String) -> Result<Data, io::Error> {
    println!("grep");
    if path.try_exists()?{
        println!("path exists");
        if path.is_dir(){
            println!("path is dir");
            //todo! list recursive
            let dir_entries = match list(path, true)? {
                Data::DirVecData(entries) => entries,
                _ => vec![],
            };

            for entry in dir_entries{
                println!("{}", entry.file_name().to_str().unwrap())
            }
        }
        else if path.is_file(){
            println!("path is file");
            let content = read(path);
            
            if content.is_ok(){
                println!("{:?}", content.unwrap());
            }
        }
    }
    Ok(Data::StatusData(1))
}


///Exits RCli
fn exit() -> Result<Data, io::Error> {
    Ok(Data::StatusData(1))
}

///Processes invalid commands
fn invalid() -> Result<Data, io::Error> {
    Err(Error::new(ErrorKind::InvalidInput, "ERROR: Invalid command."))
}


/*
    HELPER FUNCTIONS
*/

///Helper function to recursively copy a directory with its content.
///Mimics DFS algorithms. Used in cp/copy.
///Returns ReadDir object.
fn copy_dir(original_path: &Path, destination: Option<&Path>) -> Result<Data, Error>{

    if destination.is_some(){
        fs::create_dir_all(destination.unwrap()).ok();
    }

    let mut entrybuffer: Vec<DirEntry> = vec![];
    
    match fs::read_dir(original_path){
        Ok(dir_paths) => {
            for path in dir_paths{
                entrybuffer.push(path.unwrap());

                let path_type = entrybuffer.last().clone().unwrap().path();

                let new_file = path_type.file_name().unwrap();
                let new_path = destination.unwrap().join(new_file);

                if path_type.is_dir(){
                    //recurse
                    if destination.is_some(){
                        return copy_dir(&path_type, Some(&new_path));
                    }
                }
                else{
                    //add to stack
                    let _ = fs::copy(path_type, new_path);
                }
            }
            return Ok(Data::DirVecData(entrybuffer))
        },
        Err(error) => {
            return Err(error)
        },
    }
}