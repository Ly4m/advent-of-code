pub struct App<'a> {
    pub days: Vec<&'a str>,
}

impl<'a> App<'a> {
    pub(crate) fn new() -> App<'a> {
        App {
            days: vec![
                " Day 1: Historian Hysteria 😵‍💫️",
            ],
        }
    }
}
