struct Buffer<T> {
    data: Vec<T>,
}
impl<T> Buffer<T> {
    fn new() -> Self {
        Buffer {
            data: Vec::new()
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    // 对 T 类型有如下要求：
    //      1. 实现 Add 关联类型 Output = T
    //      2. 存在空值作为累加的基数
    //      3. 实现 Clone 能够克隆迭代器元素
    fn sum(&self) -> T
    where T: std::ops::Add<Output = T> + Default + Clone
    {
        self.data.iter().cloned().fold(T::default(), |acc, x| acc + x)
    }
}


fn compareString(x: &str, y: &str) -> bool {
    let x: Vec<char> = x.chars().collect();
    let y: Vec<char> = y.chars().collect();
    let len = if x.len() < y.len() {
        x.len()
    } else {
        y.len()
    };
    for i in 0..len {
        if x[i] < y[i] {
            return true;
        } else if x[i] > y[i] {
            return false;
        }
    }
    true
}
fn main() {
    let mut buf: Buffer<i32> = Buffer::new();
    for i in 1..5 {
        buf.push(i);
    }
    println!("Sum of buf is {}", buf.sum());

    let x = "abcd";
    let y = "abd";
    match compareString(x, y) {
        true => println!("{} is less than {}", x ,y),
        _ => println!("{} is less than {}", x, y)
    }

    let vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let vec_new: Vec<char> = vec.iter().cloned().map(|x| (x as u8 + 1) as char).collect();
    println!("vec is {:?}", vec);
    println!("new_vec is {:?}", vec_new);
}
