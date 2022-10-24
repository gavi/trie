use std::cmp::Ordering;
#[derive(Debug)]
#[derive(Eq)]
pub struct Node{
    val:char,
    is_terminal:bool,
    children:Vec<Node>
}

impl Node{
    pub fn new(ch:char)->Self{
        Node{val:ch,is_terminal:false,children:vec![]}
    }
}

impl Ord for Node{
    fn cmp(&self,other:&Self)->Ordering{
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

#[derive(Debug)]
pub struct Trie{
    head:Node
}

impl Trie{
    pub fn new()->Self{
        Trie{head:Node::new('*')}
    }

    pub fn is_empty(&self)->bool{
        self.head.children.len()==0
    }

    pub fn insert(&mut self,word:String){
        let mut root = &mut self.head;
        let mut count = 0;
        for ch in word.chars(){
            count = count + 1;
            let x = Node::new(ch);
            if let Ok(i) = root.children.binary_search(&x){
                root = &mut root.children[i];
            }else{
                root.children.push(x);
                root.children.sort();
                if let Ok(i) = root.children.binary_search(&Node::new(ch)){
                    root = &mut root.children[i];
                }   
            }
        }
        if count == word.len(){
            root.is_terminal = true;
        }
    }

    pub fn exists(&self,word:String)->bool{
        let mut root = &self.head;
        for ch in word.chars(){
            if let Ok(i) = root.children.binary_search(&Node::new(ch)){
                root = &root.children[i]
            }
            else{
                return false;
            }
        }
        root.is_terminal
    }

}