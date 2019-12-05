

trait Ticker {
	fn tick(&mut self) -> bool;
}

impl Ticker for u64 {
	fn tick(&mut self) -> bool {
		if *self > 0 {
			*self -= 1;
		}
		return *self > 0;
	}
}

trait Scheduler<T: Ticker> {
	fn schedule(&mut self, ticker : T);
}

impl <T: Ticker> Scheduler<T> for () {
	fn schedule(&mut self, _ticker : T) {
	}
}

impl <T: Ticker> Scheduler<T> for Vec<T> {
	fn schedule(&mut self, ticker: T) {
		self.push(ticker);
	}
}

// it is convient to be able to use a dynamic ticker
impl Ticker for Box<dyn Ticker> {
	fn tick(&mut self) -> bool {
		return (**self).tick();
	}
}

fn main() {
	let m = 5;
	let mut v = Vec::new();
	v.schedule(m);
	while v.tick() {
		println!("{:?}", v);
	}
}
