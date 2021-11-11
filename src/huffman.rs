#[derive(Debug, Default)]
pub struct HuffmanTree {
    root: HuffmanNode,
}

impl HuffmanTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, length: u8, value: u32) {
        let done = self.root.add_node(length, value);
        assert!(done);
    }
}

#[derive(Debug, Default, PartialEq)]
struct HuffmanNode {
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
    value: Option<u32>,
}

impl HuffmanNode {
    fn new() -> Self {
        Self::default()
    }

    fn with_value(value: u32) -> Self {
        Self {
            value: Some(value),
            ..Self::default()
        }
    }

    fn is_leaf(&self) -> bool {
        self.value.is_none() == false
    }

    fn add_node(&mut self, length: u8, value: u32) -> bool {
        // TODO: make this a stack-based traversal (instead of recursive) operation on HuffmanTree? Depends on performance benefits.
        match length {
            0 => panic!("Invalid Huffman node depth == 0"),
            1 => {
                // We are at the correct depth

                // Add the new node on the left, if there's room
                if self.left.is_none() == true {
                    self.left = Some(Box::new(Self::with_value(value)));
                    return true;
                }

                // Add the new node on the right, if there's room
                if self.right.is_none() == true {
                    self.right = Some(Box::new(Self::with_value(value)));
                    return true;
                }
            }
            _ => {
                // Look deeper

                // Try going left first. If needed, create a new node there first.
                if self.left.is_none() == true {
                    self.left = Some(Box::new(Self::new()));
                }

                // Go left if the left node exists and is not a leaf
                if let Some(ref mut node) = &mut self.left {
                    if node.is_leaf() == false {
                        let done = node.add_node(length - 1, value);
                        if done == true {
                            return true;
                        }
                    }
                }

                // Try going right. If needed, create a new node there first.
                if self.right.is_none() == true {
                    self.right = Some(Box::new(Self::default()));
                }

                // Go right if the right node exists and is not a leaf
                if let Some(ref mut node) = &mut self.right {
                    if node.is_leaf() == false {
                        let done = node.add_node(length - 1, value);
                        if done == true {
                            return true;
                        }
                    }
                }
            }
        }

        // There wasn't room here or at any node lower in the tree. Go back up
        // the tree and look for another node at this depth on another branch.
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut tree = HuffmanTree::default();
        let lengths = vec![2, 4, 4, 4, 4, 2, 3, 3];
        for (i, length) in lengths.into_iter().enumerate() {
            tree.add_node(length, i as u32);
        }

        /*
            Tree should look like this:
            entry 0: length 2 codeword 00
            entry 1: length 4 codeword 0100
            entry 2: length 4 codeword 0101
            entry 3: length 4 codeword 0110
            entry 4: length 4 codeword 0111
            entry 5: length 2 codeword 10
            entry 6: length 3 codeword 110
            entry 7: length 3 codeword 111
        */
        let l = tree.root.left.unwrap();
        assert_eq!(l.value, None);

        let ll = l.left.unwrap();
        assert_eq!(*ll, HuffmanNode::with_value(0));

        let lr = l.right.unwrap();
        assert_eq!(lr.value, None);

        let lrl = lr.left.unwrap();
        assert_eq!(lrl.value, None);

        let lrll = lrl.left.unwrap();
        assert_eq!(*lrll, HuffmanNode::with_value(1));

        let lrlr = lrl.right.unwrap();
        assert_eq!(*lrlr, HuffmanNode::with_value(2));

        let lrr = lr.right.unwrap();
        assert_eq!(lrr.value, None);

        let lrrl = lrr.left.unwrap();
        assert_eq!(*lrrl, HuffmanNode::with_value(3));

        let lrrr = lrr.right.unwrap();
        assert_eq!(*lrrr, HuffmanNode::with_value(4));

        let r = tree.root.right.unwrap();
        assert_eq!(r.value, None);

        let rl = r.left.unwrap();
        assert_eq!(*rl, HuffmanNode::with_value(5));

        let rr = r.right.unwrap();
        assert_eq!(rr.value, None);

        let rrl = rr.left.unwrap();
        assert_eq!(*rrl, HuffmanNode::with_value(6));

        let rrr = rr.right.unwrap();
        assert_eq!(*rrr, HuffmanNode::with_value(7));
    }
}
