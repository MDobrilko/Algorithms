use algorithms;

fn task(n: usize) {
    let mut heap = algorithms::trees::Heap::new();

    [2, 3, 5].iter().for_each(|elem| heap.push(*elem));

    for _ in 0..n {
        let v = *heap.top().unwrap();
        while *heap.top().unwrap() == v {
            heap.pop();
        }

        print!("{} ", v);
        for i in [v * 2, v * 3, v * 5] {
            heap.push(i);
        }
    }
}

fn main() {
    println!("Hello world");
}
