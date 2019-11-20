extern{
    fn call_lua();
}

pub fn unsafe_call() {
    unsafe {
        call_lua();
    }
}

fn main() {
    println!("Hello, world!");

    unsafe_call();
}
