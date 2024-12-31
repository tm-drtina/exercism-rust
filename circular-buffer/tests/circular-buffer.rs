use circular_buffer::*;
use std::rc::Rc;

#[test]
fn reading_empty_buffer_should_fail() {
    let mut buffer = CircularBuffer::<i32>::new(1);
    assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
}

#[test]
fn can_read_an_item_just_written() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write(1).is_ok());
    assert_eq!(buffer.read(), Ok(1));
}

#[test]
fn each_item_may_only_be_read_once() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write(1).is_ok());
    assert_eq!(buffer.read(), Ok(1));
    assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
}

#[test]
fn items_are_read_in_the_order_they_are_written() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    assert_eq!(buffer.read(), Ok(1));
    assert_eq!(buffer.read(), Ok(2));
}

#[test]
fn full_buffer_can_t_be_written_to() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write(1).is_ok());
    assert_eq!(buffer.write(2), Err(Error::FullBuffer));
}

#[test]
fn a_read_frees_up_capacity_for_another_write() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write(1).is_ok());
    assert_eq!(buffer.read(), Ok(1));
    assert!(buffer.write(2).is_ok());
    assert_eq!(buffer.read(), Ok(2));
}

#[test]
fn read_position_is_maintained_even_across_multiple_writes() {
    let mut buffer = CircularBuffer::new(3);
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    assert_eq!(buffer.read(), Ok(1));
    assert!(buffer.write(3).is_ok());
    assert_eq!(buffer.read(), Ok(2));
    assert_eq!(buffer.read(), Ok(3));
}

#[test]
fn items_cleared_out_of_buffer_can_t_be_read() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write(1).is_ok());
    buffer.clear();
    assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
}

#[test]
fn clear_frees_up_capacity_for_another_write() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write(1).is_ok());
    buffer.clear();
    assert!(buffer.write(2).is_ok());
    assert_eq!(buffer.read(), Ok(2));
}

#[test]
fn clear_does_nothing_on_empty_buffer() {
    let mut buffer = CircularBuffer::new(1);
    buffer.clear();
    assert!(buffer.write(1).is_ok());
    assert_eq!(buffer.read(), Ok(1));
}

#[test]
fn overwrite_acts_like_write_on_non_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write(1).is_ok());
    buffer.overwrite(2);
    assert_eq!(buffer.read(), Ok(1));
    assert_eq!(buffer.read(), Ok(2));
}

#[test]
fn overwrite_replaces_the_oldest_item_on_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    buffer.overwrite(3);
    assert_eq!(buffer.read(), Ok(2));
    assert_eq!(buffer.read(), Ok(3));
}

#[test]
fn overwrite_replaces_the_oldest_item_remaining_in_buffer_following_a_read() {
    let mut buffer = CircularBuffer::new(3);
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    assert!(buffer.write(3).is_ok());
    assert_eq!(buffer.read(), Ok(1));
    assert!(buffer.write(4).is_ok());
    buffer.overwrite(5);
    assert_eq!(buffer.read(), Ok(3));
    assert_eq!(buffer.read(), Ok(4));
    assert_eq!(buffer.read(), Ok(5));
}

#[test]
fn initial_clear_does_not_affect_wrapping_around() {
    let mut buffer = CircularBuffer::new(2);
    buffer.clear();
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    buffer.overwrite(3);
    buffer.overwrite(4);
    assert_eq!(buffer.read(), Ok(3));
    assert_eq!(buffer.read(), Ok(4));
    assert_eq!(buffer.read(), Err(Error::EmptyBuffer));
}

#[test]
fn char_buffer() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('A').is_ok());
}

#[test]
fn string_buffer() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write("Testing".to_string()).is_ok());
}

#[test]
fn clear_actually_frees_up_its_elements() {
    let mut buffer = CircularBuffer::new(1);
    let element = Rc::new(());
    assert!(buffer.write(Rc::clone(&element)).is_ok());
    assert_eq!(Rc::strong_count(&element), 2);
    buffer.clear();
    assert_eq!(Rc::strong_count(&element), 1);
}

#[test]
fn dropping_the_buffer_drops_its_elements() {
    let element = Rc::new(());
    {
        let mut buffer = CircularBuffer::new(1);
        assert!(buffer.write(Rc::clone(&element)).is_ok());
        assert_eq!(Rc::strong_count(&element), 2);
    }
    assert_eq!(Rc::strong_count(&element), 1);
}
