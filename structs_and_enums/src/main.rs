use bcrypt::hash;
use rand::distributions::Alphanumeric;
use rand::{prelude::SliceRandom, thread_rng, Rng};
use std::fmt;

impl fmt::Debug for User {
    // Proper sorting of Debug-Print.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.user_id)
            .field("username", &self.username)
            .field("password", &self.password)
            .field("email", &self.email)
            .field("status", &self.status)
            .field("active", &self.active)
            .field("avatar", &format!("{:?}", &self.avatar))
            .field("stats", &format!("{:?}", &self.stats))
            .finish()
    }
}

// #[allow(dead_code)]
// #[derive(Debug)]
struct User {
    user_id: u64,
    username: String,
    password: String,
    email: String,
    status: String,
    active: bool,
    avatar: (String, u8, u8), //Character-Type, XP, Health.
    stats: [u8; 3],           //Speed, Accelaration, Turning.
}

impl Default for User {
    fn default() -> Self {
        Self {
            active: true,
            username: "TEST".to_string(),
            password: pwgen(),
            status: "Normal".to_string(),
            email: "".to_string(),
            user_id: 1,
            avatar: (choose_type(), 0, 50), //Alchemists, Wizards, Knights, Snipers.
            stats: [100, 100, 100],
        }
    }
}

fn main() {
    let mut users = vec![];
    let mut usercount: u64 = 0;
    // let user0 = User {
    //     email: String::from("EXAMPLE-MAIL."),
    //     username: String::from("EXAMPLE-USER."),
    //     password: String::from("EXAMPLE-PASSWORD."),
    //     status: "Normal".to_string(),
    //     active: false,
    //     user_id: 0,
    //     avatar: ("KNIGHT, WIZARD, SNIPER, ALCHEMIST".to_string(), 0, 255),
    //     stats: [100, 200, 255],
    // };
    // users.push(user0);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        password: pwgen(),
        status: "Admin".to_string(),
        active: true,
        user_id: 0,
        avatar: ("WIZARD".to_string(), 255, 255),
        stats: [255, 255, 255],
    };
    users.push(user1);
    // usercount += 1;
    for number in 2..=3 {
        usercount += 1;
        let user = build_user(
            format!("user{}@mail.com", number),
            format!("user{}", number + 40),
            usercount,
        );
        // println!("{:?}\n\n", user);
        users.push(user);
    }
    dbg!(&users);

    // println!("{:?}\n\n", users[1]);
    // users[1].email = String::from("lolihackedu");
    // println!("{}", users[1].email);

    // println!("{:?}", users[1]);
    // users[1].email = String::from("lolihackeduevenharder");
    // println!("{:?}", users[1]);
}

fn build_user(email: String, username: String, user_id: u64) -> User {
    User {
        email,
        username,
        // password: pwgen(),
        // status: "Normal".to_string(),
        // active: true,
        user_id,
        // avatar: (choose_type(), 0, 50),
        // stats: [100, 100, 100],
        ..Default::default()
    }
}

fn pwgen() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let bytes = rand_string.into_bytes();
    // println!("Debug: {:?}", &bytes);
    /*let hashed = */
    hash(bytes, 4).unwrap() //;
                            // println!("Hash: {}", hashed);
                            //hashed
}

fn choose_type() -> String {
    let chartype = vec!["Knight", "Wizard", "Sniper", "Alchemist"];
    chartype
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
    // let range = rand::thread_rng().gen_range(0..=4);
    // match range {
    //     1 => chartype[1].to_string(),
    //     2 => chartype[2].to_string(),
    //     3 => chartype[3].to_string(),
    //     _ => chartype[0].to_string(),
    // }
    // chartypes[1].to_string()
}
