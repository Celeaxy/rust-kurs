#[derive(Debug)]
enum PermissionLevel {
    User,
    Instructor,
    Admin
}

impl PermissionLevel {
    fn description(&self) -> String {
        String::from(
            match self {
                PermissionLevel::User => "PermissionLevel::User",
                PermissionLevel::Instructor => "PermissionLevel::Instructor",
                PermissionLevel::Admin => "PermissionLevel::Admin"
            }
        )
    }
}

fn main() {
    let user1 = PermissionLevel::Admin;
    println!("{:?}", user1);
    println!("{}", user1.description());

    let user2 = PermissionLevel::Instructor;
    println!("{:?}", user2);
    println!("{}", user2.description());

    let user3 = PermissionLevel::User;
    println!("{:?}", user3);
    println!("{}", user3.description());
}
