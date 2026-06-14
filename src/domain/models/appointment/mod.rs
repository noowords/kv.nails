mod entity;
mod repository;
mod errors;
pub mod value_objects;

pub use entity::{ Appointment };
pub use repository::{ AppointmentRepository };
pub use errors::{ AppointmentModelDomainError };
