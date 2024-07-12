
///Returned structure when a command is executed.
pub struct Status<T>{
    data: T,
    code: i32
}

impl<T> Status<T>{
    pub fn get_data(&self) -> &T{
        return &self.data
    }

    pub fn get_code(&self) -> &i32{
        return &self.code
    } 
}