use std::io::SeekFrom;

pub trait Draw {
    fn draw(&self);
}

impl Draw for u8 {
    fn draw(&self) {
        format!("u8:{}",*self);
    }
    
}

impl Draw for f64 {
    fn draw(&self) {
        format!("f64:{}",*self);
    }
}

pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String
}

impl Draw for Button{
    fn draw(&self){
        format!("Button:{}",self.label);
    }
}

struct SelectBox{
    pub width:u32,
    pub height:u32,
    options:Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self) {
        println!("SelectBox");
    }
}

pub struct Screen{
    pub components:Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

fn draw1(x:Box<dyn Draw>)
{
    x.draw();
}

fn draw2(x:&dyn Draw)
{
    x.draw();
}

fn main()
{
    let x = 1.1f64;
    let y = 8u8;

    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}

//////////////////////////////////////////////////

// use std::{fmt::Display, iter::Sum};


// pub trait Summary {
//     fn summarize(&self)->String;
// }

// pub struct Post{
//     pub title:String,//标题
//     pub author:String,//作者
//     pub content:String,//内容
// }

// impl Summary for Post {
//     fn summarize(&self)->String {
//         format!("文章{},作者是{}",self.title,self.author)
//     }
// }
// pub struct Weibo{
//     pub username:String,    //用户名
//     pub content:String, //内容
// }

// impl Summary for Weibo {
//     fn summarize(&self)->String {
//         format!("{}发表了微博{}",self.username,self.content)
//     }
// }

// pub fn notify(item:&impl Summary){
//     println!("震惊!{}",item.summarize());
// }

// pub fn nooo(item:&impl Summary,item2:&impl Summary)
// {
//     println!("晶振！{},{}",item.summarize(),item2.summarize());
// }

// pub fn yeahhh<T:Summary>(item1:&T,item2:&T){
//     println!("晶振！{},{}",item1.summarize(),item2.summarize());
// }

// pub fn special(item:&(impl Summary+Display)){}

// pub fn ssr<T:Summary+Display>(item:&T){}

// fn some_func<U,V>(t:&U,s:&V)
//     where U:Display + Clone,
//             V:Clone + Summary
//         {
            
//         }

// fn returns_summerizable()-> impl Summary{
//     Weibo{
//         username:String::from("supdriver"),
//         content:String::from("666")
//     }
// }
// fn main()
// {

// }



///////////////////////////////////////////////////////////////////////////////////////

// fn add<T: std::ops::Add<Output = T>>(a:T,b:T)->T{
//     a+b
// }

// // fn largest<T:std::cmp::PartialOrd>
// use std::fmt::Display;
// fn create_and_print<T>()where T:From<i32> + Display{
//     let a:T = 100.into();
//     println!("a is :{}",a);
// }
// #[derive(Debug)]
// struct Point<T>{
//     x:T,
//     y:T
// }

// impl<T> Point<T> {
//     fn x(&self)->&T{
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_to_origin(&self)->f32{
//         (self.x.powi(2)+self.y.powi(2)).sqrt()
//     }
// }

// // fn debug_display_array<T:std::fmt::Debug>(arr:&[T]){
// //     println!("{:?}",arr);
// // }

// fn debug_display_array<T:std::fmt::Debug,const N:usize>(arr:&[T;N]){
//     println!("{:?}",arr);
// }

// const fn const_add(a:usize,b:usize)->usize{
//     a+b
// }

// const RESULT:usize = const_add(5,10);

// fn main() {
//     create_and_print::<i64>();

//     let p = Point::<i64> {x:64,y:32};
//     println!("{:#?}",p);
// }
