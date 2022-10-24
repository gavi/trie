pub mod trie;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main(){
    let mut trie = trie::Trie::new();
    if let Ok(lines) = read_lines("data/words.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                trie.insert(word);
            }
        }
    }
    assert_eq!(trie.exists("hello".to_string()),true);
    assert_eq!(trie.exists("Hello".to_string()),false);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
