use std::io;

//--- Question 01: Check subarray
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

//--- Question 02: Count substring
fn count_substr(paragraph: &str, sub_str: &str) -> usize {
    return paragraph.to_lowercase().matches(&sub_str.to_lowercase()).collect::<Vec<&str>>().len();
}

fn main() {

    println!("----------\r\n Q01: Check subarray \n");
    let arr1 = [4i32, 5, 0, 1, 2, 3];
    let arr2 = [1i32, 2, 9];
    let arr3 = [5, 0, 1, 2];
    println!("{:?} {} subarray of {:?}", arr2, if is_sub_array(&arr1, &arr2) {"is"} else { "is not"}, arr1);
    println!("{:?} {} subarray of {:?}", arr3, if is_sub_array(&arr1, &arr3) {"is"} else { "is not"}, arr1);

    println!("\r\n ---------- Q02: Count substring");
    let paragraph : &str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    println!("Paragraph: {} \n", paragraph);

    println!("Enter a substr: ");
    let mut sub_str = String::new();
    io::stdin().read_line(&mut sub_str).expect("failed to readline");

    println!("There are {} {} in above paragraph", count_substr(&paragraph, &sub_str.trim()), sub_str.trim());
}
