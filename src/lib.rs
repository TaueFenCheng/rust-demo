use std::fmt::Display;

pub trait Summary {
    fn output_detail(&self) -> &String;
    fn output_detail2(&self) -> String {
        println!("这是output detail2的默认实现");
        String::from("default")
    }
}

pub struct Class {
    pub stu_number: i32,
    pub name: String,
}
// 结构体方法

impl Class {
    fn ouput(&self) {
        println!("{},{}", &self.name, &self.stu_number)
    }
}

// trait

impl Summary for Class {
    fn output_detail(&self) -> &String {
        println!("{},{}",self.name,self.stu_number);
        // self.name.clone()
        &self.name
    }
    fn output_detail2(&self) -> String {
        println!("{},{}",self.name,self.stu_number);
        self.name.clone()
        // self.name
    }
}


// trait 作为参赛

pub fn notify(item: impl Summary){

}

pub fn notify2<T: Summary>(item: T){

}

// trait bound

pub fn notify3<T: Summary + Display>(item: T){

}

// where 语法糖
pub fn notify4<T>(item:T)
    where T: Summary + Display
{

}