use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    // === Test list of u32s ===
    // let mut list: LinkedList<u32> = LinkedList::new();
    // assert!(list.is_empty());
    // assert_eq!(list.get_size(), 0);
    // for i in 1..12 {
    //     list.push_front(i);
    // }
    // println!("{}", list);
    // println!("list size: {}", list.get_size());
    // println!("top element: {}", list.pop_front().unwrap());
    // println!("{}", list);
    // println!("size: {}", list.get_size());
    // println!("{}", list.to_string()); // ToString impl for anything impl Display

    // === Test list of Strings ===
    let mut list: LinkedList<String> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(format!("str_#{}", i));
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
