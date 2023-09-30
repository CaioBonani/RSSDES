#![allow(warnings)]

fn P10(key: &String) -> String {

    let mut P10 = String::new();
    
    let key: Vec<_> = key.chars().collect(); // Create a new iterator for key
    
    P10.push(key[2]);
    P10.push(key[4]);
    P10.push(key[1]);
    P10.push(key[6]);
    P10.push(key[3]);
    P10.push(key[9]);
    P10.push(key[0]);
    P10.push(key[8]);
    P10.push(key[7]);
    P10.push(key[5]);

    return P10;
}

fn P8 (key: &String) -> String{

    let mut P8 = String::new();

    let key: Vec<_> = key.chars().collect();

    P8.push(key[5]);
    P8.push(key[2]);
    P8.push(key[6]);
    P8.push(key[3]);
    P8.push(key[7]);
    P8.push(key[4]);
    P8.push(key[9]);
    P8.push(key[8]);

    return P8;
}

fn P4 (key: String) -> String{

    let mut P4 = String::new();

    let  key: Vec<_> = key.chars().collect();

    P4.push(key[1]);
    P4.push(key[3]);
    P4.push(key[2]);
    P4.push(key[0]);

    return P4;
}

fn shift1 (key: String) -> String{

    let mut shift1 = String::new();

    let key: Vec<_> = key.chars().collect();

    shift1.push(key[1]);
    shift1.push(key[2]);
    shift1.push(key[3]);
    shift1.push(key[4]);
    shift1.push(key[0]);
    shift1.push(key[6]);
    shift1.push(key[7]);
    shift1.push(key[8]);
    shift1.push(key[9]);
    shift1.push(key[5]);

    return shift1;
}

fn shift2 (key: String) -> String{

    let mut shift2 = String::new();

    let key: Vec<_> = key.chars().collect();

    shift2.push(key[2]);
    shift2.push(key[3]);
    shift2.push(key[4]);
    shift2.push(key[0]);
    shift2.push(key[1]);
    shift2.push(key[7]);
    shift2.push(key[8]);
    shift2.push(key[9]);
    shift2.push(key[5]);
    shift2.push(key[6]);

    return shift2;
}

fn K1 (key:  &String) -> String{


    let buffer = P10(key);
    let buffer = shift1(buffer);

    let K1 = P8(&buffer);

    return K1;
}

fn K2 (key:  &String) -> String{

    let buffer = P10(key);
    let buffer = shift1(buffer);
    let buffer = shift2(buffer);

    let K2 = P8(&buffer);

    return K2;
}

fn keys (key: &String) -> (String, String){

    let K1 = K1(key);
    let K2 = K2(key);

    return (K1, K2);
}

fn IP (plaintext: &String) -> String{

    let mut IP = String::new();

    let plaintext: Vec<_> = plaintext.chars().collect();

    IP.push(plaintext[1]);
    IP.push(plaintext[5]);
    IP.push(plaintext[2]);
    IP.push(plaintext[0]);
    IP.push(plaintext[3]);
    IP.push(plaintext[7]);
    IP.push(plaintext[4]);
    IP.push(plaintext[6]);

    return IP;
}

fn IP_1 (ciphertext: &String) -> String{

    let mut IP_1 = String::new();

    let ciphertext: Vec<_> = ciphertext.chars().collect();

    IP_1.push(ciphertext[3]);
    IP_1.push(ciphertext[0]);
    IP_1.push(ciphertext[2]);
    IP_1.push(ciphertext[4]);
    IP_1.push(ciphertext[6]);
    IP_1.push(ciphertext[1]);
    IP_1.push(ciphertext[7]);
    IP_1.push(ciphertext[5]);

    return IP_1;
}

fn invert (string: String) -> String {

    let mut invert = String::new();

    let string: Vec<_> = string.chars().collect();

    invert.push(string[4]);
    invert.push(string[5]);
    invert.push(string[6]);
    invert.push(string[7]);
    invert.push(string[0]);
    invert.push(string[1]);
    invert.push(string[2]);
    invert.push(string[3]);

    return invert;
}

fn EP (string: &String) -> String{

    let mut EP = String::new();

    let string: Vec<_> = string.chars().collect();

    EP.push(string[3]);
    EP.push(string[0]);
    EP.push(string[1]);
    EP.push(string[2]);
    EP.push(string[1]);
    EP.push(string[2]);
    EP.push(string[3]);
    EP.push(string[0]);

    return EP;
}

fn XOR (string1: String, string2: &String) -> String{

    let mut xor = String::new();

    let length = string1.len();

    let string1: Vec<_> = string1.chars().collect();
    let string2: Vec<_> = string2.chars().collect();

    let mut i = 0;

    while (i < length) {

        let char1 = string1[i];
        let char2 = string2[i];

        if (char1 == char2) {

            xor.push('0');
        
        } else {
        
            xor.push('1');
        }

        i += 1;
    }

    return xor;
}

fn S0 (string: &str) -> String{

    let matrix: [[i32; 4]; 4] = [

        [1, 0, 3, 2],
        [3, 2, 1, 0],
        [0, 2, 1, 3],
        [3, 1, 3, 2],
    ];

    let mut S0 = String::new();
    let string: Vec<_> = string.chars().collect();

    let row = (((string[0].to_digit(10).unwrap()) * 2) + (string[3].to_digit(10).unwrap())) as usize;
    let column = (((string[1].to_digit(10).unwrap()) * 2) + (string[2].to_digit(10).unwrap())) as usize;

    let bin = matrix[row][column];

    if (bin == 0) {

        S0.push('0');
        S0.push('0');

    } else if (bin == 1) {

        S0.push('0');
        S0.push('1');
    
    } else if (bin == 2) {

        S0.push('1');
        S0.push('0');
    
    } else if (bin == 3) {

        S0.push('1');
        S0.push('1');
    }

    return S0;
}

fn S1 (string: &str) -> String{

    let matrix: [[i32; 4]; 4] = [

        [0, 1, 2, 3],
        [2, 0, 1, 3],
        [3, 0, 1, 0],
        [2, 1, 0, 3],
    ];

    let mut S1 = String::new();

    let string: Vec<_> = string.chars().collect();

    let row = (((string[0].to_digit(10).unwrap()) * 2) + (string[3].to_digit(10).unwrap())) as usize;
    let column = (((string[1].to_digit(10).unwrap()) * 2) + (string[2].to_digit(10).unwrap())) as usize;

    let bin = matrix[row][column];

    if (bin == 0) {

        S1.push('0');
        S1.push('0');

    } else if (bin == 1) {

        S1.push('0');
        S1.push('1');
    
    } else if (bin == 2) {

        S1.push('1');
        S1.push('0');
    
    } else if (bin == 3) {

        S1.push('1');
        S1.push('1');
    }    

    return S1;
}

fn Crypto (ip: &String, key: &String) -> String{

    let ip: Vec<_> = ip.chars().collect();

    let mut L = String::new();
    L.push(ip[0]);
    L.push(ip[1]);
    L.push(ip[2]);
    L.push(ip[3]);
    
    let mut R = String::new();
    R.push(ip[4]);
    R.push(ip[5]);
    R.push(ip[6]);
    R.push(ip[7]);

    let buffer = EP(&R);

    let XOR1 = XOR(buffer, &key);

    let s0 = S0(&XOR1[0..4]);
    let s0: Vec<_> = s0.chars().collect();

    let s1 = S1(&XOR1[4..8]);
    let s1: Vec<_> = s1.chars().collect();

    let mut s0s1 = String::new();
    s0s1.push(s0[0]);
    s0s1.push(s0[1]);
    s0s1.push(s1[0]);
    s0s1.push(s1[1]);

    let p4 = P4(s0s1);

    let XOR2 = XOR(p4, &L);

    let XOR2: Vec<_> = XOR2.chars().collect();

    let mut finalString = String::new();
    let R: Vec<_> = R.chars().collect();
    
    finalString.push(XOR2[0]);
    finalString.push(XOR2[1]);
    finalString.push(XOR2[2]);
    finalString.push(XOR2[3]);
    finalString.push(R[0]);
    finalString.push(R[1]);
    finalString.push(R[2]);
    finalString.push(R[3]);

    return finalString;
}

fn Encrypt (plaintext: &String, key: &String) -> String{

    let (K1, K2) = keys(&key);

    let ip = IP(&plaintext);

    let buffer = Crypto(&ip, &K1);
    let buffer = invert(buffer);
    let buffer = Crypto(&buffer, &K2);

    let ciphertext = IP_1(&buffer);

    return ciphertext;
}

fn Decrypt (ciphertext: &String, key: &String) -> String{

    let (K1, K2) = keys(&key);

    let ip = IP(&ciphertext);

    let buffer = Crypto(&ip, &K2);
    let buffer = invert(buffer);
    let buffer = Crypto(&buffer, &K1);

    let plaintext = IP_1(&buffer);

    return plaintext;
}

fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    let mode:&str  = &args[1].trim().to_lowercase();
    
    let key: &String = &args[2];
    let input: &String = &args[3];

    match mode {

        "e" => {

            let ciphertext = Encrypt(&input, &key);

            println!("{}", ciphertext);
        },

        "d" => {

            let plaintext = Decrypt(&input, &key);

            println!("{}", plaintext);
        },

        _ => {

            println!("Invalid mode.");
        }
    }
        
}