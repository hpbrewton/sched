impl <T: Ticker> Scheduler<T> for Vec<T> {
        fn schedule(&mut self, ticker: T) {
                self.push(ticker);
        }
}

impl <T: Ticker> Ticker for Vec<T> {
        fn tick(&mut self) -> bool {
                match self.pop() {
                Some(mut t) => {
                        if t.tick() {
                                self.schedule(t);
                        }
                        true
                },
                None => false
                }
        }
}

