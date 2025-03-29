use futures::executor::block_on;

async fn do_something(){
    println!("fire!fire!fire!");
}

fn main() {
    let future = do_something();
    block_on(future);
}
