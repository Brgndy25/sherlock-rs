pub mod userinput {
    pub fn userinput(mut username: String) -> String {
        use std::io::stdin;
        println!("Please input the username");
        stdin().read_line(&mut username).ok().expect("Failed!");
        return username;
    }
}

pub mod list {
    pub fn list(list: Vec<String>) -> Vec<String> {
        use super::userinput::userinput;
        let usernameorg = String::new();
        let username = userinput(usernameorg);
        let list = [
            format!("https://vk.com/{username}"),
            format!("https://www.buzzfeed.com/{username}"),
            format!("https://soundcloud.com/{username}"),
            format!("https://www.facebook.com/{username}"),
            format!("https://www.instagram.com/{username}"),
        ];
        return list.to_vec();
    }
}
