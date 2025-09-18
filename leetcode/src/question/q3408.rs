// There is a task management system that allows users to manage their tasks, each associated with a priority. The system should efficiently handle adding, modifying, executing, and removing tasks.
//
// Implement the TaskManager class:
//
// TaskManager(vector<vector<int>>& tasks) initializes the task manager with a list of user-task-priority triples. Each element in the input list is of the form [userId, taskId, priority], which adds a task to the specified user with the given priority.
//
// void add(int userId, int taskId, int priority) adds a task with the specified taskId and priority to the user with userId. It is guaranteed that taskId does not exist in the system.
//
// void edit(int taskId, int newPriority) updates the priority of the existing taskId to newPriority. It is guaranteed that taskId exists in the system.
//
// void rmv(int taskId) removes the task identified by taskId from the system. It is guaranteed that taskId exists in the system.
//
// int execTop() executes the task with the highest priority across all users. If there are multiple tasks with the same highest priority, execute the one with the highest taskId. After executing, the taskId is removed from the system. Return the userId associated with the executed task. If no tasks are available, return -1.
//
// Note that a user may be assigned multiple tasks.

use std::collections::{BinaryHeap, HashMap};

struct TaskManager {
    by_id: HashMap<i32, (i32, i32)>, // task_id -> (user_id, priority)
    heap: BinaryHeap<(i32, i32, i32)>, // (priority, task_id, user_id)
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {

    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut by_id = HashMap::new();
        let mut heap = BinaryHeap::new();

        for task in tasks {
            let (user_id, task_id, priority) = (task[0], task[1], task[2]);
            by_id.insert(task_id, (user_id, priority));
            heap.push((priority, task_id, user_id));
        }

        Self { by_id, heap }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.by_id.insert(task_id, (user_id, priority));
        self.heap.push((priority, task_id, user_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some((user_id, priority)) = self.by_id.get_mut(&task_id) {
            *priority = new_priority;
            self.heap.push((new_priority, task_id, *user_id));
        }
    }

    fn rmv(&mut self, task_id: i32) {
        self.by_id.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id, user_id)) = self.heap.pop() {
            match self.by_id.get(&task_id) {
                Some(&(uu, pp)) if uu==user_id && pp==priority => {
                    self.by_id.remove(&task_id);
                    return user_id;
                }
                _ => continue
            }
        }
        -1
    }
}
