use engine;
use engine::{surface, camera};

pub enum TaskState {
    Remove,
    DontDraw,
    OK,
}

pub trait Task<T> {
    fn handle(&mut self, tasklist: &mut TaskList<T>, data: &engine::Data<T>) -> TaskState;
    #[allow(unused_variables)]
    fn draw<'k>(&'k self, surface: &mut surface::Surface<'k>) {}
    #[allow(unused_variables)]
    fn share(&self, data: &mut T, camera: &mut camera::Camera) {}
}

pub struct TaskList<T> {
    pub tasks: Vec<Box<Task<T>>>,
}

impl<T> TaskList<T> {
    pub fn new() -> TaskList<T> {
        TaskList{
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Box<Task<T>>) {
        self.tasks.push(task);
    }

    pub fn flush_share(&self, shared_data: &mut T, camera: &mut camera::Camera) {
        for task in self.tasks.iter() {
            task.share(shared_data, camera);
        }
    }

    pub fn flush_handle_and_draw(&mut self, data: &engine::Data<T>) -> surface::Surface {
        let mut removeixs: Vec<usize> = Vec::new();
        let mut should_draw: Vec<bool> = Vec::new();
        let mut surface = surface::Surface::new();
        let mut ix:usize = 0;
        // TODO: optimize the should_draw vector. Size is known...

        // handle everybody
        let mut newtasks = TaskList::new();
        for task in self.tasks.iter_mut() {
            let mut tmp_newtasks = TaskList::new();
            match task.handle(&mut tmp_newtasks, data) {
                TaskState::Remove => {removeixs.push(ix); should_draw.push(false)},
                TaskState::DontDraw => should_draw.push(false),
                TaskState::OK => should_draw.push(true),
            };
            newtasks.tasks.extend(tmp_newtasks.tasks.into_iter());
            ix += 1;
        }
        // remove tasks
        for ix in removeixs {
            self.tasks.remove(ix);
            should_draw.remove(ix);
        }
        // add new tasks
        self.tasks.extend(newtasks.tasks.into_iter());
        // draw!
        ix = 0;
        for task in self.tasks.iter_mut() {
            if ix==should_draw.len() {
                break;
            }
            if should_draw[ix] {
                let mut tmp_surface = surface::Surface::new();
                task.draw(&mut tmp_surface);
                surface.drawparams.extend(tmp_surface.drawparams.into_iter());
            }
            ix += 1;
        }
        surface
    }
}
