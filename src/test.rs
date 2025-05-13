#[cfg(test)]
use super::*;

#[test]
fn test_enqueue() {
    let mut queue: Queue<u32> = Queue::new();
    queue.enqueue(5);
    queue.enqueue(6);
    assert_eq!(queue.size, 2);
    assert_eq!(queue.front().unwrap(), Some(6));
    dbg!(queue);
}

#[test]
fn test_dequeue() {
    let mut queue: Queue<u32> = Queue::new();
    queue.enqueue(5);
    queue.enqueue(6);
    assert_eq!(queue.size, 2);
    queue.dequeue().unwrap();
    assert_eq!(queue.front().unwrap(), Some(6));
    assert_eq!(queue.size, 1);
    queue.dequeue().unwrap();
    assert_eq!(queue.front(), Err(format!("Queue is empty")));
    assert_eq!(queue.size, 0);
    dbg!(queue);
}
