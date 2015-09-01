use engine;
use engine::draw;

pub enum TaskState {
    Remove,
    DontDraw,
    OK,
}

pub trait Task<T> {
    fn handle(&mut self, tasklist: &mut TaskList<T>, data: &engine::Data<T>) -> TaskState;
    fn draw<'k>(&'k self, drawlist: &mut draw::DrawList<'k>);
    fn share(&self, data: &mut T);
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

    pub fn flush_share(&self, shared_data: &mut T) {
        for task in self.tasks.iter() {
            task.share(shared_data);
        }
    }

    pub fn flush_handle_and_draw(&mut self, data: &engine::Data<T>) -> draw::DrawList {
        let mut removeixs: Vec<usize> = Vec::new();
        let mut should_draw: Vec<bool> = Vec::new();
        let mut drawlist = draw::DrawList::new();
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
                let mut tmp_drawlist = draw::DrawList::new();
                task.draw(&mut tmp_drawlist);
                drawlist.params.extend(tmp_drawlist.params.into_iter());
            }
            ix += 1;
        }
        drawlist
    }
}
