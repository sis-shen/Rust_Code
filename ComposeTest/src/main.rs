//////////////////
// TTL

fn longest<'a>(x:&'a str,y:&'a str)-> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn main()
{
    let x = "FAQ".to_string();
    let result;
    {
        let y = "WHAT".to_string();
        result = longest(&x, &y);
    }
    println!("最长的字符串为{}",result);


}


/////////////////
//HashMap
// use std::collections::HashMap;

// fn main()
// {
//     let mut scores = HashMap::new();
//     scores.insert("blue", 10);

//     //覆盖已有的值
//     let old = scores.insert("blue", 20);

//     let v = scores.entry("yellow").or_insert(50);
// }


//////////////////
// match

// use std::option;

// enum MyEnum{
//     Foo,
//     Bar
// }
// fn main()
// {
//     let arr = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for _ in arr.iter().filter(|x| matches!(x,MyEnum::Foo))
//     {
//         println!("匹配一次");
//     }
// }

// fn plus_one(x:Option<i32>)->Option<i32>{
//     match x{
//         None =>None,
//         Some(i)=> Some(i+1),
//     }
// }


///////////////////
//array

// fn main()
// {
//     let arr1:[i32;5] = [3;5];
//     println!("{:?}",arr1);

//     let arr2:[String;9] = std::array::from_fn(|_i| String::from("rust is good~"));
//     println!("{:#?}",arr2);

//     let slice = &arr2[2..6];
//     println!("{:#?}",slice);

//     loop{
//         println!("I am mad !!!");
//     }
// }


//////////////////////
// enum

// enum PokerCard{
//     Clubs(u8),
//     Spades(u8),
//     Diamonds(char),
//     Hearts(char),
// }

// enum Message {
//     Quit,
//     Move {x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }
// fn main()
// {
//     let c1 = PokerCard::Clubs(5);
//     let c2 = PokerCard::Diamonds('A');

// }


/////////////////////////
//struct

// struct User{
//     active:bool,
//     username:String,
//     email:String,
//     sign_in_count:u64,
// }

// //元组结构体
// struct Point(i64,i64,i64);
// fn main(){
//     let user1 = User{
//         active: true,
//         email: String::from("email@email.com"),
//         username:String::from("supdriver"),
//         sign_in_count: 6,
//     };

//     let user2 = build_user(String::from("hello"), String::from("now"));

//     let user3 = User{
//         email:String::from("another@email.com"),
//         ..user2
//     };

// }

// fn build_user(email:String,username:String)->User{
//     User { active: true, username, email, sign_in_count: 1 }
// }




//////////////////
// tuple
//////////////////
// fn main(){
//     let tup:(i32,f64,bool) = (50,6.66,true);
//     println!("{}",tup.0);
// }


///////////////////
// String
//////////////////


// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from("rust");

//     let s3 = s1.clone()+&s2;
//     println!("{}",s1);
//     println!("{}",s3);
// }
