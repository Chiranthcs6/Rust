#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main(){
    println!("{:?} months in a year",12);
    println!("{1:?}{0:?} are the actor","Reddy","Reddy");
    println!("Now{:?}will be printed",Structure(3));
    println!("Now{:?}will be printed!",Deep(Structure(2)));
}
