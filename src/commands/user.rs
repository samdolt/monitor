use subcmd::Command;

struct User {}

impl Command for User {
    fn name<'a>(&self) -> &'a str {
        "user"
    }

    fn help<'a>(&self) -> &'a str {
        "Manage users"
    }

    /// Return a one line description. Used for the program help `bin -h`
    fn description<'a>(&self) -> &'a str {
        "Manage users"
    }

    fn run(&self, argv: &Vec<String>) {

    }
}