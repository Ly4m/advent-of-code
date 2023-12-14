pub struct App<'a> {
    pub days: Vec<&'a str>,
}

impl<'a> App<'a> {
    pub(crate) fn new() -> App<'a> {
        App {
            days: vec![
                " Day 1: Trebuchet?! ☄️",
                " Day 2: Cube Conundrum 🟥🟩🟦",
                " Day 3: Gear Ratios ⚙️",
                " Day 4: Scratchcards 🗃",
                " Day 5: If You Give A Seed A Fertilizer 🌱",
                " Day 6: Wait For It ⏳",
                " Day 7: Camel Cards 🐪",
                " Day 8: Haunted Wasteland 👻",
                " Day 9: Mirage Maintenance 🏝",
                " Day 10: Pipe Maze 🪈",
                " Day 11: Cosmic Expansion 🪐",
            ],
        }
    }
}
