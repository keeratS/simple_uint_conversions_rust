use std::io;

fn main() {
    println!("Hello! Please enter a hex or dec number below 0xff.");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let (firsttwo, rest) = input.trim().split_at(2);

    println!("Input: {}firsttwo: {}rest:{}", input, firsttwo, rest);

    if firsttwo == "0x"{
        if check_over_max(rest.parse().unwrap()){
            return;
        }
        let hexconversion = u8::from_str_radix(rest, 16).unwrap();
        print_bin(hexconversion);
    } else {
        let decconversion = input.trim().parse().unwrap();
        if check_over_max(decconversion){
            return;   
        }
        print_hex(decconversion);
    }
    return;
}

fn print_bin(hexinput: u8){
    println!("0x{:x} in binary is 0b{:b}",hexinput,hexinput);

}

fn print_hex(decinput: u8){
    println!("{} in hex is 0x{:x}", decinput, decinput);
}

fn check_over_max(value_to_check:u8)->bool{
    if value_to_check > u8::MAX{
        println!("The input is too big. Exiting.");
        return true;
    }
    return false;
}
