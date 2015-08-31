use engine::draw;

pub enum TaskState {
    Remove,
    DontDraw,
    OK,
}

//XXX --> TODO: add global state to `handle`

pub trait Task {
    fn handle(&mut self) -> TaskState;
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

    pub fn flush(&mut self) -> draw::DrawList {
        let mut removeixs: Vec<usize> = Vec::new();
        let mut should_draw: Vec<bool> = Vec::new();
        let mut drawlist = draw::DrawList::new();
        let mut ix:usize = 0;
        // TODO: optimize the should_draw vector. Size is known...

        // handle everybody
        for task in self.tasks.iter_mut() {
            match task.handle() {
                TaskState::Remove => {removeixs.push(ix); should_draw.push(false)},
                TaskState::DontDraw => should_draw.push(true),
                TaskState::OK => (),
            };
            ix += 1;
        }
        // remove
        for ix in removeixs {
            self.tasks.remove(ix);
            should_draw.remove(ix);
        }
        // draw!
        for task in self.tasks.iter_mut() {
            task.draw(&mut drawlist)
        }

        drawlist
    }
}
