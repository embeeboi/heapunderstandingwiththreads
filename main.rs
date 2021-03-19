use std::thread;


//Rust version of threads
fn main()
{
    let _threadboi = thread::spawn(||
        {
            for i in 1..10
            {
                println!("{}",i)
            }
        }
    );
    let _threadboithesequel = thread::spawn(||
        {
            for i in 1..10
            {
                println!("{}",i)
            }
        }
    );
    _threadboi.join().unwrap();
    _threadboithesequel.join().unwrap();
}