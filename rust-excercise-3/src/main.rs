use std::collections::HashMap;

//------------ Testcase --------------------------

/*-----------------------------*/
//Test case 1: Khởi tạo đầu tiên danh sách phải rỗng
/*-----------------------------*/
#[test]
fn test_case_1() {
    let sh = School::<u32>::new();
    assert_eq!(sh.students.len(), 0);
}

/*-----------------------------*/
// Test case 2:
//Thêm sinh viên có tên "Lee" với điểm số là 2
// thì tất cả các điểm số hiện có của trường là 2
//nếu thêm sinh viên khác "Nancy" với điểm số là 3
//thì các điểm số hiện tại là [2,3]

/*-----------------------------*/
#[test]
fn test_case_2() {
    let mut sh = School::<u32>::new();
    sh.add(2, "Lee");
    assert_eq!(sh.grades().len(), 1);
    assert_eq!(sh.grades()[0], 2);

    sh.add(3, "Nancy");
    assert_eq!(sh.grades().len(), 2);
    assert_eq!(sh.grades(), vec![2, 3]);
}

/*-----------------------------*/
// Test case 3:
// Giả sử danh sách hiện tại : [(Bob, 4), (Alice,4), (Tom,5)]
// với điểm số 4 thì ta có sinh viên nào: -> [Alice, Bob] not [Bob ,Alice]
// vì cần tên theo alphabet
/*-----------------------------*/
#[test]
fn test_case_3() {
    let mut sh = School::<u32>::new();
    sh.students = HashMap::from([
        ("Bob".to_owned(), 4), 
        ("Alice".to_owned(), 4), 
        ("Tom".to_owned(), 5)
    ]);
    assert_eq!(sh.grade(4), vec!["Alice".to_owned(), "Bob".to_owned()]);
}
//------------ #End testcase ---------------------------

//------------ main ------------------------------------
fn main() {
    // Run cargo test, not cargo run.
}
//------------ #End main -------------------------------

//------------ Implementation --------------------------
#[derive(Debug)]
pub struct School<T> {
    students: HashMap<String, T>
}

impl<T: std::cmp::PartialEq + Copy + std::cmp::Ord> School<T> {
    // Init struct instance
    pub fn new() -> Self {
        School {
            students: HashMap::new()
        }
    }

    // Add new student with grade
    pub fn add(&mut self, grade: T, student: &str) {
        self.students.insert(student.to_owned(), grade);            
    }

    // Get all unique grades
    pub fn grades(&self) -> Vec<T> {
        let mut uniq_grades : Vec<T> = Vec::<T>::new();
        for (_, grade) in &self.students {
            if !uniq_grades.contains(grade) {
                uniq_grades.push(*grade);
            }
        }
        uniq_grades.sort();
        return uniq_grades;
    }

    // Get list students with same grade
    pub fn grade(&self, grade: T) -> Vec<String> {
        let mut students_list : Vec<String> = Vec::new();
        for (student, _grade) in &self.students {
            if *_grade == grade {
                students_list.push(student.clone());
            }
        }
        students_list.sort();
        return students_list;
    }
}
//------------ #End Implementation --------------------------