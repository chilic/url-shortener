pub fn encode(mut n: usize) -> String {
    let map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut buf = String::new();

    while n > 0 {
        buf.push(
            map.chars().nth(n%62).unwrap()
        );
        n = n / 62;
    }

    return buf.chars().rev().collect()
}

pub fn decode(str: String) -> usize {
    let mut n = 0;
    for i in 0..str.len() {
        let symbol = str.chars().nth(i).unwrap();
        let ord = symbol as usize;
        if 'a' <= symbol && symbol <= 'z' {
            n = n*62 + ord - 'a' as usize;
        } else if 'A' as usize <= ord && ord <= 'Z' as usize {
            n = n*62 + ord - 'A' as usize + 26;
        } else if '0' as usize <= ord && ord <= '9' as usize {
            n = n*62 + ord - '0' as usize + 52;
        }
    }

    return n
}
