pub mod binary_tree {
    use std::cmp::{max, Ordering};
    use std::fmt::Debug;

    // binary tree child node link
    type Link<T> = Option<Box<BinaryTree<T>>>;

    // Binary tree definition
    #[derive(Debug, Clone, PartialEq)]
    pub struct BinaryTree<T> {
        key: T,         // data storage key
        left: Link<T>,  // left child node address storage
        right: Link<T>, // right child node address storage
    }

    impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
        pub fn new(key: T) -> Self {
            Self {
                key: key,
                left: None,
                right: None,
            }
        }

        // New child node is added as the
        // left child node of the root node
        pub fn insert_left_tree(&mut self, key: T) {
            if self.left.is_none() {
                let node = BinaryTree::new(key);
                self.left = Some(Box::new(node));
            } else {
                let mut node = BinaryTree::new(key);
                node.left = self.left.take();
                self.left = Some(Box::new(node));
            }
        }

        pub fn insert_right_tree(&mut self, key: T) {
            if self.right.is_none() {
                let node = BinaryTree::new(key);
                self.right = Some(Box::new(node));
            } else {
                let mut node = BinaryTree::new(key);
                node.right = self.right.take();
                self.right = Some(Box::new(node));
            }
        }

        pub fn size(&self) -> usize {
            self.calc_size(0)
        }

        pub fn calc_size(&self, mut size: usize) -> usize {
            size += 1;

            if self.left.is_none() {
                size = self.left.as_ref().unwrap().calc_size(size);
            }
            if self.right.is_some() {
                size = self.right.as_ref().unwrap().calc_size(size);
            }
            size
        }

        // calculates the total leaf nodes num
        pub fn leaf_size(&self) -> usize {
            // both left and right leaf nodes are none
            //which means that current node is a leaf node
            if self.left.is_none() && self.right.is_none() {
                return 1;
            }
            // calculate the total leaf nodes of both left and right leaf subtrees
            let left_leaf = match &self.left {
                Some(left) => left.leaf_size(),
                None => 0,
            };

            let right_leaf = match &self.right {
                Some(right) => right.leaf_size(),
                None => 0,
            };
            // leaf nodes = leaf nodes(left) + leaf nodes(right)
            left_leaf + right_leaf
        }

        //calculate non-leaf nodes
        pub fn none_leaf_size(&self) -> usize {
            self.size() - self.leaf_size()
        }
        // calculate the depth of a tree
        pub fn depth(&self) -> usize {
            let mut left_depth = 1;
            if let Some(left) = &self.left {
                left_depth += left.depth();
            }

            let mut right_depth = 1;
            if let Some(right) = &self.right {
                right_depth += right.depth();
            }

            // return the max depth
            max(left_depth, right_depth)
        }
        // get left subtree
        pub fn get_left(&self) -> Link<T> {
            self.left.clone()
        }

        pub fn get_right(&self) -> Link<T> {
            self.right.clone()
        }

        // get and set key
        pub fn get_key(&self) -> T {
            self.key.clone()
        }

        pub fn set_key(&mut self, key: T) {
            self.key = key;
        }

        // find min/max key in the tree
        pub fn min(&self) -> Option<&T> {
            match self.left {
                None => Some(&self.key),
                Some(ref node) => node.min(),
            }
        }

        pub fn max(&self) -> Option<&T> {
            match self.right {
                None => Some(&self.key),
                Some(ref node) => node.max(),
            }
        }

        // determine whether a key is in the tree
        // return true if the key is in the tree
        pub fn contains(&self, key: &T) -> bool {
            match &self.key.cmp(key) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left {
                    Some(left) => left.contains(key),
                    None => false,
                },
                Ordering::Less => match &self.right {
                    Some(right) => right.contains(key),
                    None => false,
                },
            }
        }
        
        // inorder traversal
        // left -> root -> right_leaf
        // left subtree -> root node -> right subtrees
        //
        pub fn preorder(&self) {
            println!("key: {:?}", &self.key);
            match &self.left {
                Some(node) => node.preorder(),
                None => (),
            }
            match &self.right {
                Some(node) => node.preorder(),
                None => (),
            }
        }
        //
        pub fn postorder(&self) {
            match &self.left {
                Some(node) => node.postorder(),
                None => (),
            }
            match &self.right {
                Some(node) => node.postorder(),
                None => (),
            }
            println!("key: {:?}", &self.key);
        }
    }
}

pub mod bin_search {
    use std::cmp::Ordering;
    pub fn binary_search1(nums: &[i32], num: i32) -> bool {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut found = false;

        while low <= high && !found {
            let mid: usize = low + ((high - low) >> 1);
            println!("mid {}", mid);
            println!(" nums=mid {}", nums[mid]);
            println!(" nums=len{}", nums.len());
            if num == nums[mid] {
                found = true;
                println!("mid {}", mid);
                println!(" nums=mid {}", nums[mid]);
                println!(" nums=len{}", nums.len());
            } else if num < nums[mid] {
                high = mid - 1;
                println!("mid {}", mid);
                println!(" nums=mid {}", nums[mid]);
                println!(" nums=len{}", nums.len());
            } else {
                low = mid + 1;
                println!("mid {}", mid);
                println!(" nums=mid {}", nums[mid]);
                println!(" nums=len{}", nums.len());
            }
            println!("mid {}", mid);
            println!(" nums=mid {}", nums[mid]);
            println!(" nums=len{}", nums.len());
        }

        found
    }
    // search insert position of a number in a sorted array 
    // if the number is not in the array return the index where it should be inserted 
    // if the number is in the array return the index of the number
    // the array is sorted in ascending Order 
    // the array is not empty
    // the array has no duplicate numbers
    // the array length is less than 10^4
    // the number is less than 10^4
     pub fn search_insert(nums: Vec<i32>, num: i32) -> i32 {
           // create a mutable variable left and right and set them to 0 and the length of the
        // array respectively 
        // the left variable will be used to store the index of the first number 
        // the right variable will be used to store the index of the last number
        // 
         let (mut left ,mut right) = (0, nums.len());
    
            // loop through the array until left is less than right ,
            while left < right {
            // calculate the mid point of the array by adding left and right and dividing by 2 
                let mid = left + ((right - left) >> 1);
            // match the mid point with the number 
                match nums[mid].cmp(&num) {
                // if the number is equal to the mid point return the mid point as the index 
                    Ordering::Less => left = mid + 1,
                    _ => right = mid,
                }
            }
            left as _
       }
    pub fn binary_search2(nums: &[i32], num: i32) -> bool {
        if nums.is_empty() {
            return false;
        }

        let mid: usize = nums.len() - 1;
        println!("mid -- {}", mid);
        println!(" nums=mid -- {}", nums[mid]);
        println!(" nums=len -- {}", nums.len());
        if num == nums[mid] {
            println!("mid -- {}", mid);
            println!(" nums=mid -- {}", nums[mid]);
            println!(" nums=len -- {}", nums.len());
            return true;
        } else if num < nums[mid] {
            println!("mid -- {}", mid);
            println!(" nums=mid --{}", nums[mid]);
            println!(" nums=len --{}", nums.len());
            return binary_search2(&nums[..mid], num);
        } else {
            println!("mid -- {}", mid);
            println!(" nums=mid -- {}", nums[mid]);
            println!(" nums=len -- {}", nums.len());
            return binary_search2(&nums[mid + 1..], num);
        }
    }
}
