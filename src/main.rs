const F0: u32 = 0;
const F1: u32 = 1;

fn main() {
    println!("Hello, world!");
    println!("for n=11: {}", calc_n_elem(11));
}

fn calc_n_elem(n: u32) -> u32 {
    if n == 0 {
        F0
    } else if n == 1 {
        F1
    } else {
        let mut count: u32 = 2;
        let mut f_n_1: u32 = F1;
        let mut f_n_2: u32 = F0;
        loop {
            let cur_sum: u32 = f_n_1 + f_n_2;
            if count == n {
                break cur_sum;
            }
            f_n_2 = f_n_1;
            f_n_1 = cur_sum;
            count += 1;
        }
    }
}
