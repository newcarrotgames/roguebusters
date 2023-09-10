use super::{inventory::InventoryModalPlayerRequest, map::MapModalPlayerRequest, crosshairs::CrosshairsModalPlayerRequest};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ModalPlayerRequest {
	InventoryRequest(InventoryModalPlayerRequest),
	MapRequest(MapModalPlayerRequest),
	CrosshairsRequest(CrosshairsModalPlayerRequest),
}