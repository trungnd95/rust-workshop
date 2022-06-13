fn is_sub_array(arr1: &[i32], arr2: &[i32]) -> bool {
    let mut i = 0;
    let mut j = 0;
    // Traverse both arrays simultaneously
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] == arr2[j] {
            i += 1;
            j += 1;

            if j == arr2.len() {
                return true;
            }
        }
        else
        {
            i = i - j + 1;
            j = 0;
        }
    }
    return false;
}

fn main() {
    let arr1 = [4i32, 5, 0, 1, 2, 3];
    let arr4 = [1i32, 2, 9];
    println!("{}", is_sub_array(&arr1, &arr4));
}
