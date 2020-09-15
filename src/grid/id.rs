/// Represents the id and 'handle' of an abstract grid (can be either an
/// *AbstractColorGrid* or an *AbstractDepthStencilGrid*). This handle has
/// multiple purposes (currently only 2), which are listed below:
/// 
/// (1) The id is needed to refer to abstract grids when populating 
/// *RenderFlowBuilder*s. 
/// 
/// (2) Getting an instance of a concrete *ColorGrid* or *DepthStencilGrid* from
/// a concrete *GridGroup*.
/// 
/// When creating an *AbstractGridGroup* via the *create_abstract_grid_group*
/// method of the *Instance*, the second element of the returned pair are the
/// *GridGroupIDs* of the abstract grid group. You need to get all the
/// *AbstractGridID*s you need from there.
/// 
/// Only Griphin implementations should create instances of this struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbstractGridID {
    group_id: u32,
    local_id: u16,
}

impl AbstractGridID {

    /// Creates a new *AbstractGridID*. This should only be used by Griphin
    /// implementations.
    pub fn new(group_id: u32, local_id: u16) -> Self {
        Self { group_id, local_id }
    }

    /// Gets the *group* id of this *AbstractGridID*. This should be a unique
    /// identifier for the *AbstractGridGroup* that owns this abstract grid.
    /// 
    /// This should only be used by implementations of Griphin.
    pub fn get_group_id(&self) -> u32 {
        self.group_id
    }

    /// Gets the *local* id of this *AbstractGridID*. Every abstract grid in
    /// the same *AbstractGridGroup* should have a distinct local id, but
    /// abstract grids in different groups can share the same local id.
    /// 
    /// This should only be used by implementations of Griphin.
    pub fn get_local_id(&self) -> u16 {
        self.local_id
    }
}
