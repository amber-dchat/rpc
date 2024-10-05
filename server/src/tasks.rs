pub(crate) static mut TASKS: usize = 0;

pub fn get_task() -> usize {
  unsafe {
    TASKS += 1;

    TASKS
  }
}
