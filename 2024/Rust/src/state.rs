pub struct App<'a> {
    pub days: Vec<&'a str>,
}

impl<'a> App<'a> {
    pub(crate) fn new() -> App<'a> {
        App {
            days: vec![
                " Day 1: Historian Hysteria 😵‍💫️",
                " Day 2: Red-Nosed Reports 🔴",
                " Day 3: Mull It Over 🔎",
                " Day 4: Ceres Search 🕵️‍♂️",
                " Day 5: Print Queue 🖨️",
                " Day 6: Guard Gallivant 👮",
                " Day 7: Bridge Repair 🌉",
                " Day 8: Resonant Collinearity 📡",
                " Day 9: ",
                " Day 10: ",
                " Day 11: ",
                " Day 12: ",
                " Day 13: ",
                " Day 14: ",
                " Day 15: ",
                " Day 16: ",
                " Day 17: ",
                " Day 18: ",
                " Day 19: ",
                " Day 20: ",
                " Day 21: ",
                " Day 22: ",
                " Day 23: ",
                " Day 24: ",
                " Day 25: ",
            ],
        }
    }
}
