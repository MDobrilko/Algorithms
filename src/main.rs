use algorithms::sortings::merge_sort;

fn main() {
    let data: Vec<i32> = (1..=10).map(|i| 10 - i).collect();
	
	println!("{:#?}", merge_sort(&data[..]).into_vec());
}
