#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct UserRepository {
    users: Vec<User>,
}

impl UserRepository {
    // Create a new user repository
    fn new() -> Self {
        UserRepository { users: Vec::new() }
    }

    // Create a new user
    fn create(&mut self, id: u32, name: String, email: String) {
        let user = User { id, name, email };
        self.users.push(user);
    }

    // Read/Get a user by ID
    fn read(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|&user| user.id == id)
    }

    // Update a user by ID
    fn update(&mut self, id: u32, new_name: String, new_email: String) -> bool {
        if let Some(user) = self.users.iter_mut().find(|user| user.id == id) {
            user.name = new_name;
            user.email = new_email;
            true
        } else {
            false
        }
    }

    // Delete a user by ID
    fn delete(&mut self, id: u32) -> bool {
        let len_before = self.users.len();
        self.users.retain(|user| user.id != id);
        len_before != self.users.len()
    }

    // List all users
    fn list_all(&self) {
        for user in &self.users {
            println!("{:?}", user);
        }
    }
}

fn main() {
    // Initialize a new UserRepository
    let mut repo = UserRepository::new();

    // Create users
    repo.create(1, "Alice".to_string(), "alice@example.com".to_string());
    repo.create(2, "Bob".to_string(), "bob@example.com".to_string());

    // List all users
    println!("All Users:");
    repo.list_all();

    // Read user by ID
    if let Some(user) = repo.read(1) {
        println!("\nUser with ID 1: {:?}", user);
    }

    // Update user
    if repo.update(2, "Bob Updated".to_string(), "bob_updated@example.com".to_string()) {
        println!("\nUser with ID 2 updated.");
    }

    // List all users after update
    println!("\nAll Users after update:");
    repo.list_all();

    // Delete user by ID
    if repo.delete(1) {
        println!("\nUser with ID 1 deleted.");
    }

    // List all users after delete
    println!("\nAll Users after delete:");
    repo.list_all();
}
