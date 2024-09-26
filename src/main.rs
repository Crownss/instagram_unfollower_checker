use serde::Deserialize;
use std::{collections::HashSet, env, fs::File, io::BufReader};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let current_dir = env::current_dir().unwrap();
    println!("INFO: this program current directory: {:?} !!", current_dir);
    let mut a = String::new();
    println!("\n path to \"following\" file(.json only!):");
    let _ = std::io::stdin()
        .read_line(&mut a)
        .expect("you didn't input anything!");
    if !a.contains(".json") {
        println!(".json file only");
        return;
    }
    let mut b = String::new();
    println!("\n path to \"followers\" file(.json only!):");
    let _ = std::io::stdin()
        .read_line(&mut b)
        .expect("you didn't input anything!");
    if !b.contains(".json") {
        println!(".json file only");
        return;
    }
    match checker(a.trim(), b.trim()) {
        Ok(res) => {
            print!("\x1B[2J\x1B[1;1H");
            println!("RESULT: this result is sort descending following from your last follow");
            for i in res {
                println!("username: {} -> link: {}", i.username, i.url);
            }
        }
        Err(err) => {
            println!("ERROR: {:?}", err)
        }
    }
}

fn checker(
    following: &str,
    followers: &str,
) -> Result<Vec<List_Of_Evil_People>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let a = File::open(following)?;
    let b = File::open(followers)?;
    let c = BufReader::new(a);
    let d = BufReader::new(b);
    let r: Following = serde_json::from_reader(c)?;
    let l: Vec<Followers> = serde_json::from_reader(d)?;
    let f: HashSet<_> = l.iter().collect();
    let s: Vec<Followers> = r
        .relationships_following
        .into_iter()
        .filter(|x| !f.contains(x))
        .collect();
    for i in s {
        let temp = List_Of_Evil_People {
            username: i.string_list_data[0].value.clone(),
            url: i.string_list_data[0].href.clone(),
        };
        result.push(temp);
    }
    Ok(result)
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
struct Followers {
    title: String,
    media_list_data: Vec<String>,
    string_list_data: Vec<String_List_Data>,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
struct Following {
    relationships_following: Vec<Followers>,
}
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
struct List_Of_Evil_People {
    username: String,
    url: String,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
struct String_List_Data {
    href: String,
    value: String,
}
