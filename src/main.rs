use std::env;
use std::fs::File;
use std::io::{Read, Write};
use rand::Rng;
fn main(){
    let args:Vec<String> = env::args().collect();

    if args.len() != 2{
        println!("Usage: {} <inputfile>",args[0])
    }

    let mut f = File::open(&args[1]).expect("Failed to open file");

    let mut buffer:Vec<u8> = vec![];

    f.read_to_end(&mut buffer).expect("Failed to read file");

    let path1 = format!("{}{}", &args[1],".part1");
    let path2 = format!("{}{}", &args[1],".part2");
    let mut file1:File = File::create(&path1).expect("Failed to create key file");
    let mut file2:File = File::create(&path2).expect("Failed to create code file");

    split_files(&buffer, &mut file1, &mut file2);

    let mut file1:File = File::open(&path1).expect("Failed to create key file");
    let mut file2:File = File::open(&path2).expect("Failed to create code file");

    merge_files(&mut file1,&mut file2);
}

fn encode(key: &Vec<u8>, message: &Vec<u8>) -> Vec<u8>{

    let mut encrypted_data :Vec<u8> = vec![];
    for i in 0..key.len(){
        encrypted_data.push(key[i] ^ message[i]);
    }

    encrypted_data
}


fn split_files(buffer:&Vec<u8>,keyFile :&mut File,encrytedFile:&mut File){
    let mut rand_bytes:Vec<u8> = vec![];
    println!("split files");
    for i in 0..buffer.len(){
        let random_byte = rand::thread_rng().gen::<u8>();
        rand_bytes.push(random_byte);
    }
    print!("write1");
    keyFile.write(&rand_bytes).expect("failed to create key file");
    print!("write2");
    encrytedFile.write(&encode(&rand_bytes,&buffer)).expect("failed to create encrypted file");
    print!("write13");
}

fn merge_files(key_file:&mut File,msg_file:&mut File){
    let mut key_buffer:Vec<u8> = vec![];
    let mut msg_buffer:Vec<u8> = vec![];

    key_file.read_to_end(&mut key_buffer).expect("Failed to read key file");
    msg_file.read_to_end(&mut msg_buffer).expect("Failed to read message file");

    let mut file1:File = File::create("mergedfile.merge").expect("Failed to create merge file");

    file1.write(&encode(&key_buffer,&msg_buffer)).expect("failed to merge files");
}