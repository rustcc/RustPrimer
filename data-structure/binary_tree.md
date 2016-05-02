# 二叉树

## 二叉树简介
在计算机科学中，二叉树是每个节点最多有两个子树的树结构。通常子树被称作“左子树”（left subtree）和“右子树”（right subtree）。二叉树常被用于实现二叉查找树和二叉堆。

1. 二叉树**左节点的值通常少于父亲节点的值**。
2. 二叉树**右节点的值通常大于父亲节点的值**。

>二叉树的每个结点至多只有二棵子树(不存在度大于2的结点)，二叉树的子树有左右之分，次序不能颠倒。二叉树的第i层至多有2^{i-1}个结点；深度为k的二叉树至多有2^k-1个结点；对任何一棵二叉树T，如果其终端结点数为n_0，度为2的结点数为n_2，则n_0=n_2+1。  

>一棵深度为k，且有2^k-1个节点称之为满二叉树；深度为k，有n个节点的二叉树，当且仅当其每一个节点都与深度为k的满二叉树中，序号为1至n的节点对应时，称之为完全二叉树。  

## 二叉树与树的区别
二叉树不是树的一种特殊情形，尽管其与树有许多相似之处，但树和二叉树有两个主要差别：

1. 树中结点的最大度数没有限制，而二叉树结点的最大度数为2。
2. 树的结点无左、右之分，而二叉树的结点有左、右之分。

## 定义二叉树的结构
```rust
type TreeNode = Option<Box<Node>>;
type BinaryTree = Node;

#[derive(Debug)]
struct Node {
   left: TreeNode,
   right: TreeNode,
   value: i32,
}
```
## 实现二叉树的插入和初始化
```rust
impl Node {
    fn new(value: i32) -> Self {
        Node {
            left: None,
            right: None,
            value: value,
        }
    }

    fn insert(&mut self, value: i32) {
        if self.value < value {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        }
    }
}
```
## 二叉树的遍历

- 先序遍历：首先访问根，再先序遍历左（右）子树，最后先序遍历右（左）子树。
- 中序遍历：首先中序遍历左（右）子树，再访问根，最后中序遍历右（左）子树。
- 后序遍历：首先后序遍历左（右）子树，再后序遍历右（左）子树，最后访问根。

下面是代码实现：
```rust
impl Node {
    fn pre_order(&self) {
        println!("{}", self.value);

        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        println!("{}", self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }
    fn pos_order(&self) {
        if let Some(ref left) = self.left {
            left.pos_order();
        }
        if let Some(ref right) = self.right {
            right.pos_order();
        }
        println!("{}", self.value);
    }
}
```
## 测试代码
```rust
fn test_insert() {
    let mut root = BinaryTree::new(3);
    root.insert(2);
    root.insert(4);
    root.insert(5);
    root.insert(6);
    root.insert(1);
    if let Some(ref left) = root.left {
        assert_eq!(left.value, 2);
    }

    if let Some(ref right) = root.right {
        assert_eq!(right.value, 4);
        if let Some(ref right) = right.right {
            assert_eq!(right.value, 5);
        }
    }
    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("\n");
    println!("Pos Order traversal");
    root.pos_order();
}

fn main() {
    test_insert();
}
```

## 练习
基于以上代码，修改成二叉堆的形式。

>二叉堆是一种特殊的堆，二叉堆是完全二元树（二叉树）或者是近似完全二元树（二叉树）。二叉堆有两种：最大堆和最小堆。最大堆：父结点的键值总是大于或等于任何一个子节点的键值；最小堆：父结点的键值总是小于或等于任何一个子节点的键值。