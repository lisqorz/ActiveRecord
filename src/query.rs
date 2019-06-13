pub struct Query {
    select: String,
    from: String,
    group_by: String,
    join: String,
    having:String,
    union:String,
    params :Vec<String>
}

impl Query {
    pub fn create_command(&mut self) {

    }
    pub fn all(&mut self){
        self.create_command()
    }
}