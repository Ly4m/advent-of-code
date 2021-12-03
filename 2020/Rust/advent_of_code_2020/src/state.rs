pub struct App<'a> {
    pub days: Vec<&'a str>
}

impl<'a> App<'a> {
    pub(crate) fn new() -> App<'a> {
        App {
            days: vec![
                " Day 1: Report Repair",
                " Day 2: Password Philosophy",
                " Day 3: Toboggan Trajectory",
                " Day 4: Passport Processing",
                " Day 5: Binary Boarding",
                " Day 6: Custom Customs",
                " Day 7: Handy Haversacks",
                " Day 8: Handheld Halting",
                " Day 9: Encoding Error",
                " Day 10: Adapter Array",
                " Day 11: Seating System",
                " Day 12: Rain Risk",
                " Day 13: Shuttle Search",
                " Day 14: Docking Data",
                " Day 15: Combo Breaker",
                " Day 16",
                " Day 17",
                " Day 18",
                " Day 19",
                " Day 20",
                " Day 21",
                " Day 22",
                " Day 23",
                " Day 24",
                " Day 25"
            ]
        }
    }
}
