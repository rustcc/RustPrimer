# 图

## 图的存储结构

图的存储结构除了要存储图中各个顶点的本身信息外，同时还要存储顶点与顶点之间的所有关系(边的信息)，因此，图的结构比较复杂，很难以数据元素在存储区中的物理位置来表示元素之间的关系，但也正是由于其任意的特性，故物理表示方法很多。常用的图的存储结构有邻接矩阵、邻接表等。

## 邻接矩阵表示法

对于一个具有n个结点的图，可以使用n*n的矩阵(二维数组)来表示它们间的邻接关系。矩阵 A(i,j) = 1 表示图中存在一条边 (Vi,Vj),而A(i,j)=0表示图中不存在边 (Vi,Vj)。
实际编程时，当图为不带权图时，可以在二维数组中存放 bool 值。

* A(i,j) = true 表示存在边 (Vi,Vj),
* A(i,j) = false 表示不存在边 (Vi,Vj);


当图带权值时，则可以直接在二维数值中存放权值，A(i,j) = null 表示不存在边 (Vi,Vj)。

下面看看我们使用邻接矩阵实现的图结构：

```rust
#[derive(Debug)]
struct Node {
    nodeid: usize,
    nodename: String,
}

#[derive(Debug,Clone)]
struct Edge {
    edge: bool,
}

#[derive(Debug)]
struct Graphadj {
    nodenums: usize,
    graphadj: Vec<Vec<Edge>>,
}

impl Node {
    fn new(nodeid: usize, nodename: String) -> Node {
        Node{
            nodeid: nodeid,
            nodename: nodename,
        }
    }
}
impl Edge {
    fn new() -> Edge {
        Edge{
            edge: false,
        }
    }
    fn have_edge() -> Edge {
        Edge{
            edge: true,
        }
    }
}

impl Graphadj {
    fn new(nums:usize) -> Graphadj {
        Graphadj {
            nodenums: nums,
            graphadj: vec![vec![Edge::new();nums]; nums],
        }
    }

    fn insert_edge(&mut self, v1: Node, v2:Node) {
        match v1.nodeid < self.nodenums && v2.nodeid<self.nodenums {
            true => {
                self.graphadj[v1.nodeid][v2.nodeid] = Edge::have_edge();
                //下面这句注释去掉相当于把图当成无向图
                //self.graphadj[v2.nodeid][v1.nodeid] = Edge::have_edge();
            }
            false => {
                panic!("your nodeid is bigger than nodenums!");
            }
        }
    }
}

fn main() {
    let mut g = Graphadj::new(2);
    let v1 = Node::new(0, "v1".to_string());
    let v2 = Node::new(1, "v2".to_string());
    g.insert_edge(v1,v2);
    println!("{:?}", g);
}
```

## 邻接表表示法

邻接表是图的一种最主要存储结构，用来描述图上的每一个点。

>**实现方式：**对图的每个顶点建立一个容器（n个顶点建立n个容器），第i个容器中的结点包含顶点Vi的所有邻接顶点。实际上我们常用的邻接矩阵就是一种未离散化每个点的边集的邻接表。

* 在有向图中，描述每个点向别的节点连的边（点 a->点 b 这种情况）。
* 在无向图中，描述每个点所有的边(点 a->点 b这种情况)

与邻接表相对应的存图方式叫做边集表，这种方法用一个容器存储所有的边。

## **练习：**
实现链接表表示法的图结构。
