
fn main()
{
    let mut s1 = String::from("Aloha");
    // println!("{}长度是{}",s1,s1.len());
    print_len(&mut s1);
    println!("{}长度是{}",s1,s1.len());

}   

fn print_len(str:&mut String){
    //引用
    str.push_str("嘿嘿嘿");
    println!("{}长度是{}",str,str.len());

}

// const _MAX_POINTS:u32 = 100_000;

// fn main() {
//     let mut str = String::from("hello");
//     str.push_str(" from supdriver");

//     take_ownership(str);
//     // println!("{}",some_str);//报错

//     let mut str2 = String::from("hello222");
//     str2.push_str(" from supdriver");

//     let str2 = take_and_give_back(str2);
//     println!("{}",str2);


// }

// fn take_ownership(some_string:String){
//     println!("{}",some_string);
// }

// fn take_and_give_back(some_string:String)->String{
//     println!("{}",some_string);
//     some_string
// }

// fn add_with_extra(x:i32,y:i32)->i32
// {
//     let x = x+1;
//     let y = y + 1;
//     x+y
// }

// fn dead_end()->!{
//     panic!("你已经穷途末路了，崩溃吧！");
// }

// fn forerver()->!{
//     loop{

//     };
// }