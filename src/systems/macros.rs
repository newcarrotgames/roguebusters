#[macro_export]
macro_rules! check_component {
	($comp:ty) => {
		let storage = world.read_storage::<$comp>();
		if let Some(comp) = storage.get(entity) {
			Some(comp)
		} else {
			None
		}
	};
}