use super::{
    save::{SavedBoard, SavedSocket},
    Chip, Socket, Trace,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// A Board that contains Traces and Sockets
#[derive(Default, Debug)]
pub struct Board {
    traces: Vec<Rc<RefCell<Trace>>>,
    sockets: Vec<Rc<RefCell<Socket>>>,
}

impl Board {
    /// Create a new empty Board
    pub fn new() -> Self {
        Self {
            traces: vec![],
            sockets: vec![],
        }
    }

    /// Create a new trace and return it
    pub fn new_trace(&mut self) -> Rc<RefCell<Trace>> {
        let trace = Rc::new(RefCell::new(Trace::new()));
        self.traces.push(trace);
        // unwrap because we just pushed a value so there's no reason to get a None here
        self.traces.last_mut().unwrap().clone()
    }

    /// Create a new socket and return it
    /// Note that you'll have to plug a chip on it before linking it with the traces
    pub fn new_socket(&mut self) -> Rc<RefCell<Socket>> {
        let socket = Rc::new(RefCell::new(Socket::new()));
        self.sockets.push(socket);
        // unwrap because we just pushed a value so there's no reason to get a None here
        self.sockets.last_mut().unwrap().clone()
    }

    /// Create a new socket with a chip and return it
    pub fn new_socket_with(&mut self, chip: Box<dyn Chip>) -> Rc<RefCell<Socket>> {
        let socket = Rc::new(RefCell::new(Socket::new()));
        socket.borrow_mut().plug(chip);
        self.sockets.push(socket);
        // unwrap because we just pushed a value so there's no reason to get a None here
        self.sockets.last_mut().unwrap().clone()
    }

    pub fn get_sockets(&self) -> Vec<Rc<RefCell<Socket>>> {
        self.sockets.clone()
    }

    pub fn get_traces(&self) -> Vec<Rc<RefCell<Trace>>> {
        self.traces.clone()
    }

    pub fn get_socket(&mut self, uuid: u128) -> Option<Rc<RefCell<Socket>>> {
        for socket in &self.sockets {
            if socket.borrow().get_uuid() == uuid {
                return Some(socket.clone());
            }
        }
        None
    }

    /// Run the circuit for a certain amount of time
    /// You must use `use_during` since it provides more accurate simulation by stepping
    pub fn run(&mut self, time_elapsed: Duration) {
        // TODO: find a way to update the traces accurately
        // current issue : the order of the traces affects the order of the links
        for trc in &mut self.traces {
            trc.borrow_mut().communicate();
        }
        for skt in &mut self.sockets {
            skt.borrow_mut().run(time_elapsed);
        }
    }

    /// Run the circuit for a certain amount of time segmented by a step
    /// The smaller the step the more accurate the simulation will be.
    pub fn run_during(&mut self, duration: Duration, step: Duration) {
        let mut elapsed = Duration::new(0, 0);
        while elapsed < duration {
            self.run(step);
            elapsed += step;
        }
    }

    pub fn run_realtime(&mut self, duration: Duration) {
        let instant = Instant::now();
        let mut old = Instant::now();
        let mut new = Instant::now();
        while instant.elapsed() <= duration {
            self.run(new.duration_since(old));
            old = new;
            new = Instant::now();
        }
    }

    /// Save the board to a file in RON format
    pub fn save(&self, filepath: &str) -> std::io::Result<()> {
        let mut s_board = SavedBoard::new();
        for socket in &self.sockets {
            let saved_chip = socket.borrow().save();
            let mut saved_socket = SavedSocket::new();
            if saved_chip.chip_type != "NULL" {
                saved_socket.set_chip(saved_chip);
            }
            s_board.add_socket(saved_socket);
        }
        for trace in &self.traces {
            s_board.add_trace(trace.borrow().save());
        }

        let file = std::fs::File::create(std::path::Path::new(filepath))?;
        if let Err(e) = ron::ser::to_writer(file, &s_board) {
            Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                format!("{e:?}"),
            ))
        } else {
            Ok(())
        }
    }

    /// Load a file and create a board according to this file
    /// You'll need to provide a "chip factory" function as second parameter
    /// By default it's `virt_ic::chip::virt_ic_chip_factory`
    /// ```
    /// use virt_ic::chip::virt_ic_chip_factory;
    ///
    /// let mut board = Board::load("my_saved_board.ron", &virt_ic_chip_factory).unwrap();
    /// ```
    pub fn load(
        filepath: &str,
        chip_factory: &dyn Fn(&str) -> Option<Box<dyn Chip>>,
    ) -> std::io::Result<Self> {
        let file = std::fs::File::open(std::path::Path::new(filepath))?;
        let s_board: Result<SavedBoard, ron::Error> = ron::de::from_reader(file);
        s_board.map_or_else(
            |err| {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("{err:?}"),
                ))
            },
            |board| Ok(board.build_board(chip_factory)),
        )
    }
}
