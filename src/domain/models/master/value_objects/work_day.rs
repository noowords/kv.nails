use chrono::{ NaiveTime };
use serde::{ Serialize, Deserialize };

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WorkDay {
    slots: Vec<NaiveTime>
}

impl WorkDay {
    pub fn new(mut slots: Vec<NaiveTime>) -> Self {
        slots.sort();

        Self {
            slots
        }
    }

    pub fn slots(
        &self
    ) -> &Vec<NaiveTime> {
        &self.slots
    }

    pub fn has_slot(
        &self,
        time: NaiveTime
    ) -> bool {
        self.slots.contains(&time)
    }

    pub fn add_slot(
        &mut self,
        time: NaiveTime
    ) {
        if !self.has_slot(time) {
            self.slots.push(time);
            self.slots.sort();
        }
    }

    pub fn remove_slot(
        &mut self,
        time: NaiveTime
    ) {
        self.slots.retain(|t| *t != time);
    }
}
