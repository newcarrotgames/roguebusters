use super::{
    crosshairs::CrosshairsModalPlayerRequest, inventory::InventoryModalPlayerRequest,
    map::MapModalPlayerRequest,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ModalPlayerRequest {
    InventoryRequest(InventoryModalPlayerRequest),
    MapRequest(MapModalPlayerRequest),
    CrosshairsRequest(CrosshairsModalPlayerRequest),
}
