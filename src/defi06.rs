pub fn do_work(input : String) -> u32 {
    let letters = input.chars().collect::<Vec<char>>();

    let mut a = letters[0];
    let mut b = letters[1];
    let mut c = letters[2];
    let mut d = letters[3];

    for i in 4..letters.len() {
        if a != b && b != c && c != d && b != d && a != d && a != c {
            println!("ok");
            return (i).try_into().unwrap();
        }

        a = b;
        b = c;
        c = d;
        d = letters[i];
    }

    0
}