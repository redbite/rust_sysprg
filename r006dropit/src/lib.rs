pub struct DropIt {
    s: String
}

impl DropIt {
    pub fn new(s_init: String) -> Self{
        Self {
            s: s_init
        }
    }
    pub fn set_s_to_void(&mut self){
        self.s.clear();
    }

    pub fn get_s(&self) -> &String {
        &self.s
    }
}

impl Drop for DropIt {
    //default trait that's called, by default, in the end to free the allocated space, just as a destructor
    fn drop(&mut self){
        println!("This is the content of self: {}", self.s);
    }
}