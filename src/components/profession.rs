use specs::{storage::HashMapStorage, Component};
use specs_derive::Component;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum JobType {
    Unemployed,
    Police,
    Bartender,
    Shopkeeper,
    Clerk,
    Doctor,
    Criminal,
    Cook,
    Journalist,
    Lawyer,
    Pharmacist,
    Barber,
    Boxer,
    Priest,
    Brewer,
    Dealer,
    Tailor,
    LaundryWorker,
    DockWorker,
    HotelStaff,
    Banker,
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Profession {
    pub job_type: JobType,
    pub employer_building_id: Option<i32>,
    #[allow(dead_code)]
    pub home_building_id: Option<i32>,
    pub at_work: bool,
}

impl Profession {
    pub fn with_employer(job_type: JobType, building_id: i32) -> Self {
        Profession {
            job_type,
            employer_building_id: Some(building_id),
            home_building_id: None,
            at_work: false,
        }
    }

    /// Maps an interior_type to its staffing configuration: (JobType, count).
    pub fn staff_for_interior(interior_type: &str) -> Option<(JobType, usize)> {
        match interior_type {
            "police_precinct" => Some((JobType::Police, 6)),
            "speakeasy"       => Some((JobType::Bartender, 2)),
            "shop"            => Some((JobType::Shopkeeper, 1)),
            "pawn_shop"       => Some((JobType::Shopkeeper, 1)),
            "office"          => Some((JobType::Clerk, 2)),
            "clinic"          => Some((JobType::Doctor, 2)),
            "hideout"         => Some((JobType::Criminal, 3)),
            "restaurant"      => Some((JobType::Cook, 2)),
            "newspaper"       => Some((JobType::Journalist, 2)),
            "courthouse"      => Some((JobType::Lawyer, 2)),
            "pharmacy"        => Some((JobType::Pharmacist, 1)),
            "barbershop"      => Some((JobType::Barber, 1)),
            "boxing_gym"      => Some((JobType::Boxer, 2)),
            "church"          => Some((JobType::Priest, 1)),
            "brewery"         => Some((JobType::Brewer, 2)),
            "casino"          => Some((JobType::Dealer, 3)),
            "tailor"          => Some((JobType::Tailor, 1)),
            "laundry"         => Some((JobType::LaundryWorker, 1)),
            "dock_office"     => Some((JobType::DockWorker, 2)),
            "hotel"           => Some((JobType::HotelStaff, 2)),
            "bank"            => Some((JobType::Banker, 2)),
            _ => None,
        }
    }

    pub fn should_be_at_work(hour: u8, job_type: &JobType) -> bool {
        match job_type {
            JobType::Unemployed => false,
            JobType::Police     => true,
            JobType::Bartender | JobType::Dealer => hour >= 18 || hour < 2,
            _ => hour >= 8 && hour < 17,
        }
    }
}
