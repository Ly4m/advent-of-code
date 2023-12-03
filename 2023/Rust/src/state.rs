pub struct App<'a> {
    pub days: Vec<&'a str>,
}

impl<'a> App<'a> {
    pub(crate) fn new() -> App<'a> {
        App {
            days: vec![" Day 1: Trebuchet?! ☄️",
                       " Day 2: Cube Conundrum 🟥🟩🟦",
                       " Day 3: TODO",
                       " Day 4: TODO",
                       " Day 5: TODO",
                       " Day 6: TODO",
                       " Day 7: TODO",
            ],
        }
    }
}
