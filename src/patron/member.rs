use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
pub struct Member {
    name: String,
    gender: String,
    dob: String,
    membership_id: String,
    height: f64,
    weight: f64,
}

impl Member {
    pub fn new(name: &str, gender: &str, dob: &str, height: f64, weight: f64) -> Self {
        Self {
            name: name.to_string(),
            gender: gender.to_string(),
            dob: dob.to_string(),
            membership_id: String::new(), // This will be set when adding to the database
            height,
            weight,
        }
    }
}

struct MembershipDatabase {
    members: HashMap<String, Member>, // Membership ID to Member mapping
}

impl MembershipDatabase {
    fn new() -> Self {
        Self {
            members: HashMap::new(),
        }
    }

    fn add_member(&mut self, member: Member) {
        // Generate a random 6-digit membership ID
        let membership_id: String = rand::thread_rng()
            .gen_range(100_000..1_000_000)
            .to_string();

        let mut member = member;
        member.membership_id = membership_id.clone();

        // Add the member to the database
        self.members.insert(membership_id, member);
    }

    fn get_member(&self, membership_id: &str) -> Option<&Member> {
        self.members.get(membership_id)
    }
}
