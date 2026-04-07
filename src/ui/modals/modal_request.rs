use super::{
    crosshairs::CrosshairsModalPlayerRequest, inventory::InventoryModalPlayerRequest,
    map::MapModalPlayerRequest,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum ModalPlayerRequest {
    InventoryRequest(InventoryModalPlayerRequest),
    MapRequest(MapModalPlayerRequest),
    CrosshairsRequest(CrosshairsModalPlayerRequest),
}
