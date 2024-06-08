/*\
    Resources: 

    Macros cheat sheet:https://cheats.rs/#macros-attributes
    Writting macros: https://dhghomon.github.io/easy_rust/Chapter_61.html 

    Synchronous Logger to log output
        -terminal operations
            -later on parser operations too if debuging
        -When printing paths remove the prefix \\?\ and double slashes \\ from paths
*/
use std::sync::mpsc::{sync_channel, Receiver, RecvError, SendError, SyncSender};


struct Logger{
    pub sync_sender: SyncSender<String>,
    pub receiver: Receiver<String>
}
impl Logger{
    pub fn new() -> Self{
        let (sender, receiver) = sync_channel(1024);
        return Logger{sync_sender: sender, receiver: receiver};
    }

    pub fn add_sender(&self) -> SyncSender<String>{
        return self.sync_sender.clone()
    }

    pub fn add_message(self, message: String) -> Result<(), SendError<String>>{
        return self.sync_sender.send(message);
    }

    pub fn receive_message(&self) -> Result<String, RecvError>{
        return self.receiver.recv()
    }
}
