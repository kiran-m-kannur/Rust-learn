//This generates a random password with alphanumeric characters





use rand::Rng;




fn main() {
    const charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789!@#$%^&*)(";



    let mut rng = rand::thread_rng();
    const len:i32 = 10;
    let password: String = (0..len).map(|_|{
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
    }).collect();

    println!("Your Password is {} : ",password);
}

