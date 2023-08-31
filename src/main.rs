fn main() {
    let mut nums = [10, 5, 2, 8, 7 , 1];

    // bubble_sort(&mut nums);
    // println!("bubble sorted nums: {:?}", nums);

    insert_sort(&mut nums);
}


fn bubble_sort(nums: &mut [i32]) {

    for i in 0..nums.len() {
        for j in 0..nums.len() - i - 1 {
            println!("{:?}", nums);
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1)
            }
        }
    }
}

fn insert_sort(nums: &mut [i32]) {

    for i in 1..nums.len() {

        let mut j = i;
        while j > 0 && nums[j] < nums[j - 1]{
                nums.swap(j, j - 1);
                j -= 1;
        }
    }
    println!("insert sorted array: {:?}", nums);
}