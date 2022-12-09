pub struct App<'a> {
    pub days: Vec<&'a str>,
}

impl<'a> App<'a> {
    pub(crate) fn new() -> App<'a> {
        App {
            days: vec![" Day 1: Calorie Counting ⚖️",
                       " Day 2: Rock Paper Scissors ✂️🪨📄️",
                       " Day 3: Rucksack Reorganization 🎒"],
        }
    }
}
