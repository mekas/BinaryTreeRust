use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    /**change node u, with node v via parent swap
     * v must be singular node
     * this function only update parent, copy v value while original link on v is untouched which could be problematic
     */
    fn transplant(u: &BstNodeLink, v: &BstNodeLink) -> BstNodeLink{
        // test if u isn't root
        if BstNode::has_parent(u){
            // if u is the left child of parent
            let parent = &BstNode::get_strong_parent(u);
            if BstNode::is_node_match(&parent.borrow().left.as_ref().unwrap(), u){
                parent.borrow_mut().left = Some(v.clone());
            } else{ //node is coming from the right
                parent.borrow_mut().right = Some(v.clone());
            }
            v.borrow_mut().parent = Some(Rc::<RefCell<BstNode>>::downgrade(parent));
        }
        //whatever the case after processing we return v
        return v.clone()
    }

    /**
     * replace node z with its child according to key arrangement
     */
    pub fn tree_delete(node: &BstNodeLink) -> BstNodeLink{
        let mut node_alter: BstNodeLink = BstNode::new_bst_nodelink(0);
        if !BstNode::is_right_child_exist(node){
            //replace node with its left child
            node_alter = BstNode::transplant(node, &node.borrow().left.clone().unwrap());
        } else if !BstNode::is_left_child_exist(node){
            //replace node with its right child
            node_alter = BstNode::transplant(node, &node.borrow().right.clone().unwrap());
        } else{//in case node have both childs
            //we seek the child of right subtree with minimum key to replace z
            let mut min_node = node.borrow().right.clone().unwrap().borrow().minimum();
            let min_node_parent = BstNode::upgrade_weak_to_strong(min_node.borrow().parent.clone());
            if !BstNode::is_node_match_option_as_ref(&min_node_parent, &Some(node.clone())){
                //only transplant min_node right if exist
                if BstNode::is_right_child_exist(&min_node){
                    //change min_node with its right child
                    min_node = BstNode::transplant(&min_node, &min_node.clone().borrow().right.clone().unwrap());
                } else{//else right child is none, then set current min_node reference from parent as None
                    min_node_parent.as_ref().unwrap().borrow_mut().left = None;
                }
                //attach right child parent to node alter
                node.borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::<RefCell<BstNode>>::downgrade(&min_node));
                //set node_alter right child to current node
                min_node.borrow_mut().right = node.borrow().right.clone();
            }
            node_alter = BstNode::transplant(node, &min_node);
            node.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::<RefCell<BstNode>>::downgrade(&node_alter));
            //set left child of min_node the left child of old node
            node_alter.borrow_mut().left = node.borrow().left.clone();

            node.borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::<RefCell<BstNode>>::downgrade(&node_alter));
            node_alter.borrow_mut().right = node.borrow().right.clone();
        }
        //default return node_alter
        node_alter
    }

    fn has_parent(node: &BstNodeLink) -> bool{
        if BstNode::upgrade_weak_to_strong(node.borrow().parent.clone()).is_none(){
            return false
        }
        true
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    fn get_strong_parent(node: &BstNodeLink) -> BstNodeLink{
        return BstNode::upgrade_weak_to_strong(node.borrow().parent.clone()).unwrap()
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        }
        // empty right child case
        else {
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None
        }
    }

    /**
     * Insert new value in the current tree,
     * return the new tree
     */
    pub fn tree_insert(node: &Option<BstNodeLink>, z: &i32) -> BstNodeLink{
        if let Some(current_node) = node{
            if current_node.borrow().key.unwrap() < *z{
                //recurse to the right child if able
                if BstNode::is_right_child_exist(current_node){
                    let right_node = current_node.borrow().right.clone();
                    return BstNode::tree_insert(&right_node, z);
                } else{
                    //we can't recurse so we insert to the right
                    current_node.borrow_mut().add_right_child(current_node, *z);
                }
            } else{//z is lower than key
                //recurse
                if BstNode::is_left_child_exist(current_node){
                    let left_node = current_node.borrow().left.clone();
                    return BstNode::tree_insert(&left_node, z);
                } else{
                    //we can't recurse so we insert to the left
                    current_node.borrow_mut().add_left_child(current_node, *z);
                }
            }

            //return the root
            return BstNode::get_root(&node.clone().unwrap());
        }

        //all fails mean, the node is None
        let new_node = BstNode::new_bst_nodelink(*z);
        return new_node;
    }

    fn is_right_child_exist(node: &BstNodeLink) -> bool{
        if node.borrow().right.is_some(){
            return true;
        }

        false
    }

    fn is_left_child_exist(node: &BstNodeLink) -> bool{
        if node.borrow().left.is_some(){
            return true;
        }

        false
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink value (only)
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    //helper function to compare both nodelink value (only)
    fn is_node_match_option_as_ref(node1: &Option<BstNodeLink>, node2: &Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        
        if let Some(node1v) = node1 {
            return node2.clone().is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
