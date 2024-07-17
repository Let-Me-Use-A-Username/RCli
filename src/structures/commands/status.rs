
// Note : Commands will return Status struct. With the status code we will implemenet && and ||
// Note: Operation will let the engine know what to do with the returned data
///Returned structure when a command is executed.
pub struct Status<T>{
    data: T,
    code: i32,
    operation: Operation
}

impl<T> Status<T>{
    pub fn new(data: T, code: i32, operation: Operation) -> Self{
        Status { 
            data: data, 
            code: code, 
            operation: operation 
        }
    }

    pub fn get_data(&self) -> &T{
        return &self.data
    }

    pub fn get_code(&self) -> &i32{
        return &self.code
    } 
}

pub enum Operation{
    ChangeDirectory,
    PrintResult,
    RedirectInput,
    RedirectOutput
}