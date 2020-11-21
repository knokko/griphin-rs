use crate::PrimitiveTopology;

/// This enum is used to indicate what a *VertexBuffer* is going to be used for. (What will be its
/// *PrimitiveTopology*? Will it be used in combination with an *IndexBuffer*?)
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VertexBufferUsage {

    /// The vertex buffer will be drawn without an associated index buffer. Thus, the *topology*
    /// and order of the vertices in the buffer determine the shape.
    NoIndices{ topology: PrimitiveTopology },

    /// The vertex buffer will be drawn in combination with an index buffer. Thus, the *topology*
    /// and the indices will determine the shape.
    Indices{ topology: PrimitiveTopology },

    /// It is either not known how the vertices will be used, or the vertex buffer is 'flexible' in
    /// the sense that it can be drawn both with and without an index buffer (or is made to be
    /// meaningful with more than 1 topology).
    WildCard
}