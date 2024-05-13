use std::collections::VecDeque;
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::{self, Error, ErrorKind};
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::rcliparser::utils::windows_file_attributes;
use crate::rcliterminal::terminal_singlenton::Terminal;

use super::objects::tokens::{FlagObjectPair, GetTupleValue, GetValue, TokenCommandType, TokenCommands, TokenObjects};


pub fn invoke(core: TokenCommands, path: TokenObjects, mut flag_vector: VecDeque<FlagObjectPair>, terminal_instance: &mut Terminal) -> Result<i32, Error>{

    // println!("\nCORE: {:?}", core.clone());
    // println!("OBJECT: {:?}", path.clone());
    // println!("FLAGS: {:?}", flag_vector.clone());

    let token_value = path.get_value();
    let path_value = Path::new(&token_value);
    let mut operation_status: Result<i32, Error> = Ok(100);

    match core.get_type(){
        TokenCommandType::HOME => {
            operation_status = home(terminal_instance);
        }
        TokenCommandType::CWD => {
            operation_status = cwd(terminal_instance);
        }
        TokenCommandType::TOUCH => {
            operation_status = touch(path_value);
        },
        TokenCommandType::MKDIR => {
            let flag: bool = match flag_vector.pop_front() {
                Some(flag) => {
                    match flag.get_value().0 {
                        Some(value) => {
                            core.containt_flag(&value)
                        },
                        None => false
                    }
                }
                None => false
            };

            operation_status = mkdir(path_value, flag);
        },
        TokenCommandType::REMOVE => {
            let flag: bool = match flag_vector.pop_front() {
                Some(flag) => {
                    match flag.get_value().0 {
                        Some(value) => {
                            core.containt_flag(&value)
                        },
                        None => false
                    }
                }
                None => false
            };

            operation_status = remove(path_value, flag);
        },
        TokenCommandType::COPY => {
            match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();

                    let flag = flag_value.0.unwrap_or("INVALID".to_string());
                    let obj = flag_value.1.unwrap_or("INVALID".to_string());

                    if core.containt_flag(&flag) & !obj.eq("INVALID"){
                        let new_path = terminal_instance.get_current_directory().join(path_value);
                        let destination_path = terminal_instance.get_current_directory().join(obj);
                        
                        operation_status = copy(new_path.as_path(), destination_path.as_path(), terminal_instance);
                    }
                    else{
                        operation_status = Err(Error::new(ErrorKind::InvalidInput, "INTERNAL ERROR: Unknown flag {flag:?}."));
                    }
                },
                None => {
                    operation_status = Err(Error::new(ErrorKind::InvalidInput, "INTERNAL ERROR: No destination provided"));
                }
            };
        },
        TokenCommandType::MOVE => {
            //move or rename file
            //after confirming it moved, delete origin
            match flag_vector.pop_front() {
                Some(flag) => {
                    let flag_value = flag.get_value();

                    let flag = flag_value.0.unwrap_or("INVALID".to_string());
                    let obj = flag_value.1.unwrap_or("INVALID".to_string());

                    if core.containt_flag(&flag) & !obj.eq("INVALID"){
                        let new_path = terminal_instance.get_current_directory().join(path_value);
                        let destination_path = terminal_instance.get_current_directory().join(obj);
                        
                        operation_status = r#move(new_path.as_path(), destination_path.as_path(), terminal_instance);
                       
                    }
                    else{
                        operation_status = Err(Error::new(ErrorKind::InvalidInput, "INTERNAL ERROR: Unknown flag {flag:?}."));
                    }
                },
                None => {
                    operation_status = Err(Error::new(ErrorKind::InvalidInput, "INTERNAL ERROR: No destination provided"));
                }
            };
        },
        TokenCommandType::READ => {
            operation_status = read(path_value);
        },
        TokenCommandType::LIST => {
            let flag: bool = match flag_vector.pop_front() {
                Some(flag) => {
                    match flag.get_value().0 {
                        Some(value) => {
                            core.containt_flag(&value)
                        },
                        None => false
                    }
                }
                None => false
            };

            operation_status = list(path_value, flag);
            
        },
        TokenCommandType::CD => {
            let new_path = terminal_instance.get_current_directory().join(path_value);

            let original_path_exists = new_path.try_exists().unwrap_or(false);
            
            if original_path_exists{
                operation_status = traverse_directory(new_path.as_path(), terminal_instance);
            }
            else{
                operation_status = Err(Error::new(ErrorKind::NotFound, "INTERNAL ERROR: Path {new_path:?} not found"));
            }
        },
        TokenCommandType::EXIT => {
            operation_status = exit();
        },
        TokenCommandType::INVALID => {
            operation_status = invalid();
        },
    }
    
    if operation_status.is_err(){
        handle_error(&operation_status.as_mut().err().unwrap().kind().clone());
    }
    
    return operation_status
}


fn handle_error(errorkind: &ErrorKind){
    match errorkind{
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

///Shows the users home directory
fn home(terminal_instance: &mut Terminal) -> Result<i32, Error>{
    let home_dir = terminal_instance.get_home_directory().display().to_string();
    println!("{home_dir}");
    Ok(100)
}


///Shows current working dir
fn cwd(terminal_instance: &mut Terminal) -> Result<i32, Error>{
    let current_path = terminal_instance.get_current_directory_to_string();
    println!("{current_path}");
    Ok(100)
}


///Creates a file at the given path
fn touch(file_path: &Path) -> Result<i32, Error>{
    //could not need the open() clause unless pipelining
    let file = OpenOptions::new().write(true).read(true).create(true).open(file_path);

    match file{
        Ok(_) => {
            return Ok(100);

        },
        Err(error) => {
            return Err(error);
        }
    }
}


///Creates a directory at the given path
fn mkdir(path: &Path, recursive: bool) -> Result<i32, Error>{
    let mut builder = DirBuilder::new();
    
    builder.recursive(recursive);
    
    let directory = builder.create(path);


    match directory{
        Ok(_) => {
            return Ok(100);
        },
        Err(error) => {
            return Err(error);
        }
        
    }
}


///Removes a file or dir.
fn remove(path: &Path, recursive: bool) -> Result<i32, Error>{
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
            return Ok(100)
        },
        Err(error) => {
            return Err(error);
        },
    }
}


///Copies the content of either a file or a directory. This is move powerful over move
///due to the fact that it can copy without requiring permissions (on Windows) whereas 
///move requires.
fn copy(path: &Path, destination: &Path, terminal_instance: &mut Terminal) -> Result<i32, Error>{
    let path_exists = path.try_exists()?;
    let dest_exists = destination.try_exists()?;

    //Destination doesn't exist, so copy the content to a new file/directory
    if path_exists & !dest_exists{
        if path.is_file(){
            match fs::copy(path, destination){
                Ok(_) => {
                    return Ok(100)
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
                    return Ok(100)
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
                    return Ok(100)
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
fn r#move(path: &Path, destination: &Path, terminal_instance: &mut Terminal) -> Result<i32, Error>{
    let path_exists = path.try_exists()? | path.exists();
    let dest_exists = destination.try_exists()?;
    //simple rename
    if path_exists & !dest_exists{
        match fs::rename(path, destination){
            Ok(_) => {
                return Ok(100)
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
fn read(path: &Path) -> Result<i32, Error>{
    if path.exists() | path.try_exists()?{
        if path.is_dir(){
            return Err(Error::new(ErrorKind::InvalidInput, "INTERNAL ERROR: Cannot read directory. Use ls instead."))
        }
        let file = OpenOptions::new().write(true).read(true).open(path)?;

        match io::read_to_string(file){
            Ok(content) => {
                println!("{content}");
                return Ok(100)
            },
            Err(error) => {
                return Err(error)
            },
        }
        
    }
    return Err(Error::new(ErrorKind::NotFound, "INTERNAL ERROR: Invalid path."))
}


///Lists items in a directory
fn list(dir_path: &Path, hidden: bool) -> Result<i32, Error>{
    let mut outputbuffer: Vec<String> = vec![];

    match fs::read_dir(dir_path) {
        Ok(paths) => {
            for path in paths{
                let dir_path = path.unwrap().path();

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
        },
        Err(error) => {
            return Err(error)
        },
    };

    for obj in outputbuffer{
        println!("{}", obj);
    }
    Ok(100)
}


///Traverses given path if valid
fn traverse_directory(path: &Path, terminal_instance: &mut Terminal) -> Result<i32, Error>{
    let mut pathbuffer = PathBuf::new();
    pathbuffer.push(path);
    
    match terminal_instance.set_current_directory(pathbuffer) {
        Ok(status) => {
            Ok(status)
        },
        Err(error_path) => {
            let error = Error::new(ErrorKind::InvalidData, error_path.display().to_string());
            return Err(error)
        },
    }
}


///Exits RCli
fn exit() -> Result<i32, io::Error> {
    Ok(1)
}

///Processes invalid commands
fn invalid() -> Result<i32, io::Error> {
    Err(Error::new(ErrorKind::InvalidInput, "ERROR: Invalid command."))
}


/*
    HELPER FUNCTIONS
*/

///Helper function to recursively copy a directory with its content.
///Mimics DFS algorithms. Used in cp/copy
fn copy_dir(original_path: &Path, destination: Option<&Path>) -> Result<i32, Error>{

    if destination.is_some(){
        fs::create_dir_all(destination.unwrap()).ok();
    }
    
    match fs::read_dir(original_path){
        Ok(dir_paths) => {
            for path in dir_paths{
                let path_type = path.unwrap().path();

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
            Ok(100)
        },
        Err(error) => {
            return Err(error)
        },
    }
}