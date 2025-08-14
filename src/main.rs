use algorithm::container::linked_list::LinkedList;

fn main() {
    let mut l = LinkedList::new();
    l.push(1);
    l.push(2);
    for v in l.iter() {
        println!("num: {v}");
    }
}
