fn main() {
    println!("arry sort start....");
    // 数组大小排序
    let mut list = vec![5, 1, 34, 50, 200, 66, 34, 51, 25, 100, 65, 500];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
    // 字符大小排序
    let mut list = vec!['G', 'z', 'D', 'e', 'A', 'C', 'a', 'W', 'Y'];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
}

/**
 * Ord & PartialOrd
符号：<、⇐、>、>=

区别：PartialOrd 不许满足连通性（a⇐b或a>=b），只需满足反对称性（a ⇐ b 且 a >= b 可推出 a == b）和传递性（a ⇐ b 且 b ⇐ c 可推出 a ⇐ c）。
 */
fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for _i in 0..list.len() {
        for x in 0..list.len() - 1 {
            //实际交换次数等于 n-1
            if list[x] > list[x + 1] {
                list.swap(x, x + 1); //元素交换位置
            }
        }
    }
    list
}
