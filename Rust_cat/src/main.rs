use std::fs;
use std::env;
use std::io::Read;

fn main() {
    let args : Vec<String> = env::args().collect();
    if moreThanOneFile(&args){
        println!("You provided more than one file name!!");
        println!("Here is the usage : cat <The path to the file>" );
    return ();
    }
    let p = &args[1];
    let f_check = fs::metadata(p);
    let mut res = String::new();
    match f_check {
        Ok(c)=> {
            if c.is_dir(){
                println!("The path is correct but this is a directory dude!!");
            }else{
                let mut f = fs::File::open(p).expect("Error while opening the file!!");
                match f.read_to_string(&mut res) {
                    Ok(_)=>{},
                    Err(_)=>{println!("Error occuring while reading the file!!");}
                }
            }

        },
        Err(_)=>{
            println!("an error occured and did not let us open the file");
        }

    }
    println!("{}",res);



}
fn moreThanOneFile(args: &Vec<String>)->bool{
args.len()>=3 
}



