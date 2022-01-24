use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("猜数!!!");
	let secret_number = rand::thread_rng().gen_range(1..101);

	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("无法读取");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("请输入一个数字!!!");
				continue
			}
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("太小"),
			Ordering::Greater => println!("太大"),
			Ordering::Equal => {
				println!("正确");
				break;
			}
		}
		println!("你猜测的数是: {}", guess);
	}
}
