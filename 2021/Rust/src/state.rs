pub struct App<'a> {
    pub days: Vec<&'a str>
}

impl<'a> App<'a> {
    pub(crate) fn new() -> App<'a> {
        App {
            days: vec![
                " Day 1: Sonar Sweep",
                " Day 2: Dive!",
                " Day 3: Binary Diagnostic",
                " Day 4: Giant Squid",
                " Day 5: Hydrothermal Venture",
                " Day 6: Lanternfish",
            ]
        }
    }
}
