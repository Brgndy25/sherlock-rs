pub mod list {
    use std::io::stdin;
    pub fn list(sitelist: &Vec<String>) -> Vec<String> {
        let mut username = String::new();
        println!("Please input the username");
        stdin().read_line(&mut username).ok().expect("Failed!");
        let mut finallist = Vec::new();
        
        let sitelist = [
            "https://vk.com/",
            "https://www.buzzfeed.com/",
            "https://soundcloud.com/",
            "https://www.facebook.com/",
            "https://www.instagram.com/",
            "https://www.github.com/",
        ];
        tokio::runtime::Builder::new_multi_thread()
        .worker_threads(12)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {for x in sitelist {
          finallist.push(format!("{x}{username}"));
    }
});
    drop(sitelist);
    finallist
}
}
