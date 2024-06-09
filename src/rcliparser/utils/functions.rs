use std::{fmt::format, fs::{self, DirBuilder, DirEntry, OpenOptions}, io::{self, BufRead, Error, ErrorKind}, os::windows::fs::MetadataExt, path::{Path, PathBuf}};
use regex::Regex;

use crate::{rcliparser::objects::data_types::Data, rcliterminal::terminal::Terminal};

use crate::rcliparser::utils::windows::windows_file_attributes;


///Shows the users home directory. Returns path.
pub fn home(terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let home_dir = terminal_instance.get_home_directory();
    Ok(Data::PathData(home_dir))
}


///Shows current working dir. Returns path.
pub fn cwd(terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let current_path = terminal_instance.get_current_directory();
    Ok(Data::PathData(current_path))
}

///Prints to terminal
pub fn echo(input: &String) -> Result<Data, Error>{
    return Ok(Data::StringData(input.to_string()))
}


///Creates a file at the given path. Returns file.
pub fn touch(file_path: &Path, data: Option<String>) -> Result<Data, Error>{
    //could not need the open() clause unless pipelining
    let file_operation = OpenOptions::new().write(true).read(true).create(true).open(file_path);

    match file_operation{
        Ok(_) => {
            if data.is_some(){
                match fs::write(file_path, data.clone().unwrap()){
                    Ok(_) => {
                        return Ok(Data::StringData(data.unwrap()))
                    },
                    Err(error) => {
                        return Err(error)
                    }
                }
            }
            return Ok(Data::PathData(file_path.to_path_buf()));

        },
        Err(error) => {
            return Err(error);
        }
    }
}


///Creates a directory at the given path. Returns path to top level directory.
pub fn mkdir(path: &Path, recursive: bool) -> Result<Data, Error>{
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
pub fn remove(path: &Path, recursive: bool) -> Result<Data, Error>{
    let mut res : Result<(), Error> = Result::Err(Error::new(ErrorKind::NotFound, "Invoker Error: Object doesn't exist."));
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


///Copies the content of either a file or a directory. This is more powerful over move
///due to the fact that it can copy without requiring permissions (on Windows) whereas 
///move requires.
///Returns either the destination file or directory.
pub fn copy(path: &Path, destination: &Path, terminal_instance: &mut Terminal) -> Result<Data, Error>{

    //if origin doesnt exist error
    if !path.try_exists()?{
        return Err(Error::new(ErrorKind::NotFound, "Invoker Error: Path |{path:?}| doesn't exist."))
    }

    //if destination exists
    if destination.try_exists()?{
        //and origin is a directory
        if path.is_dir(){
            let mut destination_path = terminal_instance.get_current_directory().join(destination);
            destination_path.push(path.components().last().unwrap().as_os_str());
            
            return copy_dir(path, Some(destination_path.as_path()));
        }
        //else it can be a file/symlink etc doesn't matter
        else{
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
    }
    //if destination doesn't exist
    else{
        //if origin is path simply copy it (with data of course)
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
        //if origin is dir copy all
        else{
            return copy_dir(path, Some(destination));
        }
    }
}


///Moves and renames a file or directory.
///If destination doesn't exist it renames a file. If it does exist it copies it.
///Returns destination path.
pub fn r#move(path: &Path, destination: &Path, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let path_exists = path.try_exists()? | path.exists();
    let dest_exists = destination.try_exists()?;
    //simple rename
    let mut result: Result<Data, Error>= Err(Error::new(ErrorKind::Other, "Invoker Error: Error occured while moving."));

    if path_exists{
        let origin_canonicalized = path.canonicalize()?;
        let destintion_canonicalized = PathBuf::from(path.canonicalize().unwrap()).join(destination);

        //Origin and destination are on a different hierarchical level so we copy(because rename has problems)
        if !destintion_canonicalized.components().last().eq(&origin_canonicalized.components().last()){
            result = copy(path, destination, terminal_instance);
        }
        //Origin and destination are on the same hierarchy
        else{
            if !dest_exists{
                if fs::rename(path, destination).is_ok(){
                    result = Ok(Data::PathData(destination.into()))
                }
            }
            else{
                result = copy(path, destination, terminal_instance);
            }
        }

        if result.is_ok(){
            //If result is valid, cd to father dir
            let _ = traverse_directory(Path::new(".."), terminal_instance);
            //Delete underlying dir
            let _ = remove(&origin_canonicalized, true);
            
            return Ok(result.unwrap())
        }
    }

    return Err(result.unwrap_err())
}

///Reads the content of a file to terminal
///Returns content as string.
pub fn read(path: &Path) -> Result<Data, Error>{
    if path.exists() | path.try_exists()?{
        if path.is_dir(){
            return Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Cannot read directory. Use ls instead."))
        }
        let file = OpenOptions::new().write(true).read(true).open(path);

        if file.is_ok(){
            let lines = io::BufReader::new(file.unwrap()).lines();

            let mut output_string = Vec::<String>::new();
            
            for line in lines.flatten(){
                output_string.push(format(format_args!("[ {} ]", line)));
            }
            return Ok(Data::VecStringData(output_string));
        }
        return Err(Error::new(ErrorKind::NotFound, "Invoker Error: Error while opening file."));


    }
    return Err(Error::new(ErrorKind::NotFound, "Invoker Error: Invalid path."))
}


///Lists items in a directory
pub fn list(dir_path: &Path, hidden: bool, recursive: bool) -> Result<Data, Error>{
    let mut outputbuffer: Vec<PathBuf> = vec![];
    
    match fs::read_dir(dir_path) {
        Ok(paths) => {
            for path in paths{
                let dir_path = path.unwrap().path();

                match fs::metadata(dir_path.clone()) {
                    Ok(meta) => {
                        let attributes = meta.file_attributes();
                        
                        let entry_attributes = windows_file_attributes::match_attributes(attributes);
                        
                        let canonicalized_path = match dir_path.canonicalize() {
                            Ok(path) => path,
                            Err(_) => dir_path.clone(),
                        };

                        //if hidden is true push everything
                        if hidden{
                            outputbuffer.push(canonicalized_path);
                        }
                        //else if hidden is false, then append dirs that arent marked as hidden
                        else if !entry_attributes.contains(&windows_file_attributes::WindowsAttributes::HIDDEN){
                            outputbuffer.push(canonicalized_path);
                        }
                        
                        if recursive & dir_path.is_dir(){
                            match list(&dir_path, hidden, recursive)?{
                                Data::DirPathData(path_vec) => {
                                    path_vec.iter().for_each(|y| {

                                        let normalized_path = match y.canonicalize() {
                                            Ok(path) => path,
                                            Err(_) => y.to_path_buf(),
                                        };

                                        outputbuffer.push(normalized_path)
                                    })
                                },
                                _ => unreachable!()
                            } 
                        }
                    },
                    Err(error) => {
                        return Err(error)
                    }
                }; 
            }
            return Ok(Data::DirPathData(outputbuffer))
        },
        Err(error) => {
            return Err(error)
        },
    }
}


///Traverses given path if valid
pub fn traverse_directory(path: &Path, terminal_instance: &mut Terminal) -> Result<Data, Error>{
    let pathbuffer = PathBuf::from(path);
    
    match terminal_instance.set_current_directory(pathbuffer) {
        Ok(status) => {
            return Ok(status)
        },
        Err(error_path) => {
            return Err(Error::new(ErrorKind::InvalidData, error_path.to_string()));
        },
    }
}

///For given data returns match.
///Data can be either a dir or path.
///Therefore matches are either files or Strings.
pub fn grep(path: &Path, regex_string: &String) -> Result<Data, io::Error> {
    let pattern_string = format!(r"\b\w*{}\w*\b", regex_string);
    let pattern = Regex::new(pattern_string.as_str()).unwrap();
    
    if path.try_exists()?{
        if path.is_dir(){
            let mut output_string = Vec::<String>::new();
            
            let dir_entries = match list(path, true, false)? {
                Data::DirPathData(entries) => entries,
                _ => vec![],
            };

            for entry in dir_entries{
                let entry_name = entry.as_path().to_str().unwrap();
                if pattern.is_match(entry_name){
                    output_string.push(format(format_args!("[ {} ]", entry_name)));
                }
            }

            return Ok(Data::VecStringData(output_string))
        }
        else if path.is_file(){
            //todo! change with read?
            let res = read(path);

            if res.is_ok(){
                let mut output_string = Vec::<String>::new();

                match res.unwrap(){
                    Data::VecStringData(vector) => {
                        for line in vector{
                            if pattern.is_match(line.as_str()){
                                output_string.push(format(format_args!("[ {} ]", line)));
                            }
                        }
                        return Ok(Data::VecStringData(output_string))
                    },
                    _ => unreachable!()
                }
            }
            return Err(res.unwrap_err())
        }
    }
    Err(Error::new(ErrorKind::NotFound, "Invoker Error: Invalid path."))
}

///Temporary function until grep becomes generic
pub fn match_string(input: String, regex_string: &String) -> Option<String> {
    let pattern_string = format!(r"\b\w*{}\w*\b", regex_string);
    let pattern = Regex::new(pattern_string.as_str()).unwrap();
    
    if pattern.is_match(input.as_str()){
        return Some(input)
    }
    
    return None
}

///Exits RCli
pub fn exit() -> Result<Data, io::Error> {
    Ok(Data::StatusData(1))
}

///Processes invalid commands
pub fn invalid() -> Result<Data, io::Error> {
    Err(Error::new(ErrorKind::InvalidInput, "Invoker Error: Invalid command."))
}


/*
    HELPER FUNCTIONS
*/

///Helper function to recursively copy a directory with its content.
///Mimics DFS algorithms. Used in cp/copy and in move.
///Returns top level path of copied dir.
fn copy_dir(original_path: &Path, destination: Option<&Path>) -> Result<Data, Error>{
    
    if destination.is_some(){
        fs::create_dir_all(destination.unwrap()).ok();
    }

    let mut entries: Vec<DirEntry> = vec![];
    
    match fs::read_dir(original_path){
        Ok(dir_paths) => {
            //Collect all current dir entries
            for entry in dir_paths.map(|entry| entry.unwrap()){
                entries.push(entry);
            }

            //Double for loop is needed because some elements are skipped when the matching
            //is done in parallel with the loop.

            //for every entry found
            for entry in entries{
                let new_path = destination.unwrap().join(entry.file_name());
                    
                if entry.file_type().unwrap().is_file(){
                    let _ = fs::copy(&entry.path(), new_path);
                }
                else if entry.file_type().unwrap().is_dir(){
                    let _ = copy_dir(entry.path().as_path(), Some(&new_path));
                }
                else{
                    let _ = fs::copy(&entry.path(), new_path);
                }
            }
            return Ok(Data::PathData(destination.unwrap().to_path_buf()))
        }
        Err(error) => {
            return Err(error)
        }
    }
}