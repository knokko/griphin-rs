use crate::*;

use std::sync::Arc;

/// The *Gateway* between the CPU and GPU. This gateway should be used to transfer vertex data, index
/// data, and textures to the GPU. Use the *get_gateway* method of the *Instance* to get a reference
/// to the gateway.
pub trait Gateway {

    /// Transfers the given vertex data (in *vertices*) from the CPU to the GPU. The *usage* parameter
    /// indices what you are planning to do with these vertices and may be used later on to report
    /// errors if they are used for something else (depending on the *DebugLevel* given to later
    /// functions).
    ///
    /// Depending on the Griphin implementation, this method may or may not block. In either case,
    /// the implementation must make sure that the transfer is completed before these vertices are
    /// used (but might stall the first draw call in the worst case).
    fn transfer_vertices(&self, vertices: &VertexStore, usage: VertexBufferUsage) -> Arc<dyn VertexBuffer>;
}