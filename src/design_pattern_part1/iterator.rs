trait IIterator {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Vec<Patient>;
}

trait Aggregate {
    fn get_iterator(&self) -> impl IIterator;
}

#[derive(Debug, Clone)]
struct Patient {
    id: i64,
    name: String,
}

impl Patient {
    fn new(id: i64, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Clone)]
struct WaitingRoom {
    patients: Vec<Patient>,
}

impl WaitingRoom {
    fn new() -> Self {
        Self {
            patients: Vec::new(),
        }
    }

    fn get_patients(&self) -> Vec<Patient> {
        self.patients.clone()
    }

    fn get_count(&self) -> i64 {
        self.patients.len().try_into().unwrap()
    }

    fn check_in(&mut self, patient: Patient) {
        self.patients.push(patient)
    }
}

impl Aggregate for WaitingRoom {
    fn get_iterator(&self) -> impl IIterator {
        WaitingRoomIterator::new(self.clone())
    }
}

struct WaitingRoomIterator {
    position: i64,
    aggregate: WaitingRoom,
}

impl WaitingRoomIterator {
    fn new(aggregate: WaitingRoom) -> Self {
        Self {
            position: 0,
            aggregate: aggregate,
        }
    }
}

impl IIterator for WaitingRoomIterator {
    fn has_next(&self) -> bool {
        self.position < self.aggregate.get_count()
    }

    fn next(&mut self) -> Vec<Patient> {
        if self.has_next() == false {
            println!("患者がいません");
            return Vec::new();
        }

        let patient = self.aggregate.get_patients();
        self.position += 1;

        return patient;
    }
}

pub struct IteratorMain;

impl IteratorMain {
    pub fn index() {
        let mut waiting_room = WaitingRoom::new();

        waiting_room.check_in(Patient::new(1, "Yamada".to_string()));
        waiting_room.check_in(Patient::new(2, "Suzuki".to_string()));
        waiting_room.check_in(Patient::new(3, "Tanaka".to_string()));

        let mut iterator = waiting_room.get_iterator();
        println!("{:?}", iterator.next());
        println!("{:?}", iterator.next());
        println!("{:?}", iterator.next());
        println!("{:?}", iterator.next());
    }
}