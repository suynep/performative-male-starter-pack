use std::collections::HashMap;
use std::path::PathBuf;
use std::str::from_utf8;

/*
 * do NOT download already saved files
 */

pub fn save_already_seen(
    dir: String,
    files: &Vec<PathBuf>,
    preexisting: Option<HashMap<String, Vec<String>>>,
) {
    /*
     * use a simple
     *          {
     *          "dir": ["file_name_1", "..."],
     *          }
     */

    let mut hm = HashMap::<String, Vec<String>>::new();

    if let Some(preexisting) = preexisting {
        if preexisting.len() > 0 {
            hm = preexisting;
        }
    }

    let mut v: Vec<String> = Vec::new();

    for (_, f) in files.iter().enumerate() {
        v.push(f.to_str().unwrap().to_owned());
    }

    hm.insert(dir, v);

    if let Ok(s) = serde_json::ser::to_string(&hm) {
        let _ = std::fs::write("seen.json", s);
    } else {
        eprintln!("Cannot write seen files");
    };
}

pub fn read_already_seen(path: String) -> Option<HashMap<String, Vec<String>>> {
    let curr = HashMap::<String, Vec<String>>::new();

    if let Ok(read_data) = std::fs::read(path) {
        if let Ok(res) = serde_json::de::from_str(&String::from_utf8(read_data).unwrap()) {
            return Some(res);
        } else {
            return None;
        }
    } else {
        return None;
    }
}
