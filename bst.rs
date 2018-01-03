pub struct Tree<T> {
    key: Option<T>,
    L: Option<Box<Tree<T>>>,
    R: Option<Box<Tree<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree {
            key: None,
            L: None,
            R: None,
        }
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        match self.key{
            None => {
                self.key = Some(key);
                return true;
            },
            Some(ref mut x) => {
                if key == *x {
                    return false;
                }
                else if key < *x {
                    match self.L {
                        None => {
                            self.L = Some(Box::new(Tree{
                                key: Some(key),
                                L: None,
                                R: None,
                            }));
                            return true;
                        },
                        Some(ref mut y) => {
                            return y.insert(key); 
                        }
                    }
                }
                else {
                    match self.R{
                        None => {
                            self.R = Some(Box::new(Tree {
                                key: Some(key),
                                L: None,
                                R: None,
                            }));
                            return true;
                        },
                        Some(ref mut y) => {
                            return y.insert(key); 
                        }
                    }
                }
            }
        }
    }
    
    /// Returns `true` if `key` exists in the tree, and `false` otherwise.  
    pub fn find(&self, key: &T) -> bool {
        match self.key{
            None => {return false}
            Some(ref x) => {
                if key == x {
                    return true;
                }
                if key < x {
                    match self.L{
                        None => {return false}
                        Some(ref y) => {return y.find(key);}
                    }
                }
                else {
                    match self.R{
                        None => {return false}
                        Some(ref y) => {return y.find(key);} 
                    }
                }
            }
        }
    }

    /// Returns the preorder traversal of the tree.    
    pub fn preorder(&self) -> Vec<&T> {
        let mut res:Vec<&T> = Vec::new();
        let mut vec1:Vec<&T> = Vec::new();
        let mut vec2:Vec<&T> = Vec::new();
        match self.key {
            None => { return res }
            Some(ref x) => {
                res.push(x);
                match self.L {
                    None => {}
                    Some(ref x) => {
                        vec1 = x.preorder();
                    }
                }
                match self.R{
                    None => {}
                    Some(ref x) => {
                        vec2 = x.preorder();
                    }
                }
            }
        }
        res.append(&mut vec1);
        res.append(&mut vec2);
        res
    }
 
    /// Returns the inorder traversal of the tree.    
    pub fn inorder(&self) -> Vec<&T> {
        let mut res:Vec<&T> = Vec::new();
        let mut vec1:Vec<&T> = Vec::new();
        let mut vec2:Vec<&T> = Vec::new();
        match self.key {
            None => {return res}
            Some(ref x) => {
                res.push(x);
                match self.L {
                    None => {}
                    Some(ref x) => {
                        vec1 = x.inorder();
                    }
                }
                match self.R{
                    None => {}
                    Some(ref x) => {
                        vec2 = x.inorder();
                    }
                }
            }
        }
        vec1.append(&mut res);
        vec1.append(&mut vec2);
        vec1
    }
  
    /// Returns the postorder traversal of the tree.   
    pub fn postorder(&self) -> Vec<&T> {
        let mut res:Vec<&T> = Vec::new();
        let mut vec1:Vec<&T> = Vec::new();
        let mut vec2:Vec<&T> = Vec::new();
        match self.key {
            None => {return res}
            Some(ref x) => {
                vec2.push(x);
                match self.L {
                    None => {}
                    Some(ref x) => {
                        res = x.postorder();
                    }
                }
                match self.R{
                    None => {}
                    Some(ref x) => {
                        vec1 = x.postorder();
                    }
                }
            }
        }
        res.append(&mut vec1);
        res.append(&mut vec2);
        res
    }
}
