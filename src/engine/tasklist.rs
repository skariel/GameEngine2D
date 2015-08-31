use engine;
use engine::draw;

pub enum TaskState {
    Remove,
    DontDraw,
    OK,
}

//XXX --> TODO: add global state to `handle`

pub trait Task {
    fn handle(&mut self, tasklist: &mut TaskList, data: &engine::Data) -> TaskState;
    fn draw<'k>(&'k self, drawlist: &mut draw::DrawList<'k>);
}

pub struct TaskList {
    pub tasks: Vec<Box<Task>>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList{
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Box<Task>) {
        self.tasks.push(task);
    }

    pub fn flush(&mut self, data: &engine::Data) -> draw::DrawList {
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
                task.draw(&mut drawlist);
            }
            ix += 1;
        }
        drawlist
    }
}
