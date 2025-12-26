use std::io;
use std::process;
use rand::Rng;

fn read_ln(rg: &mut String) -> i32
{
    io::stdin().read_line(rg)
    .expect("Error in input");
    let rg: i32 = rg.trim().parse()
    .expect("Not an integer, Schwein!");
    rg
}
fn main() 
{
    let mut a = String::new();
    let a: i32 = read_ln(&mut a);
    let mut b = String::new();
    let b: i32 = read_ln(&mut b);
    let mut c = String::new();
    let c: i32 = read_ln(&mut c);
    loop
    {
        let mb: i32 = (b/a)* -1 as i32;
        let ma: i32 = (c/a) as i32;
        let i = rand::thread_rng().gen_range(-500,501); 
        let x = rand::thread_rng().gen_range(-500,501); 
        if i+x == mb && i*x == ma
        {
            println!("x1={} and x2={}",i,x);
            break;
            
        }
         else {
            println!("with {} and {} its wrong", i, x);
            println!("{} {}", mb, ma);
        }
    }
}
