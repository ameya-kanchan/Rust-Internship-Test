struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn max_depth(&self) -> i32 {
        if self.left.is_none() && self.right.is_none() {
            1
        } else {
            let left_depth = self.left.as_ref().map_or(0, |n| n.max_depth());
            let right_depth = self.right.as_ref().map_or(0, |n| n.max_depth());
            std::cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
    let root = TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })),
    };

    println!("The maximum depth of the tree is: {}", root.max_depth());
}