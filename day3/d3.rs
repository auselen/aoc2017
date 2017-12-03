fn main() {
    let mut input = 368078;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = 'r';
    let mut stride = 1;

    loop {
        for _ in 0..2 {
            for _ in 0..stride {
                match dir {
                    'u' => y = y - 1,
                    'd' => y = y + 1,
                    'l' => x = x - 1,
                    'r' => x = x + 1,
                    _ => (),
                }
                input = input - 1;
                if input == 1 {
                    println!("{}, {} = {}", x, y, x.abs() + y.abs());
                    return;
                }
            }
            dir = match dir {
                'r' => 'u',
                'u' => 'l',
                'l' => 'd',
                'd' => 'r',
                _ => ' ',
            }
        }
        stride = stride + 1;
    }
}