use crate::*;

/// Represents a list/buffer of vertices in GPU memory. To obtain a *VertexBuffer*, create a
/// *VertexStore* and transfer it through the *Gateway*.
pub trait VertexBuffer {

    /// Gets the *VertexBufferUsage* of this *VertexBuffer*. This is the same usage that was passed
    /// as parameter when this vertex buffer was created.
    fn get_usage(&self) -> VertexBufferUsage;

    /// Gets the number of vertices in this *VertexBuffer*. Note that this is really the number of
    /// vertices, and *not* the number of indices or triangles.
    fn get_num_vertices(&self) -> usize;

    /// Checks if this *VertexBuffer* is ready to be used for drawing right away. Note that the
    /// user doesn't have to worry about this value: the Griphin implementation must ensure that
    /// this buffer is ready before it is actually drawn.
    ///
    /// The primary purpose of this method is to check whether or not the user should expect some
    /// extra delay before drawing. This may help for scheduling operations.
    fn is_ready(&self) -> bool;

    /// Blocks the current thread until this *VertexBuffer* is ready to be drawn. Note that the
    /// user doesn't *have to* call this method before drawing: the Griphin implementation must
    /// ensure that it is ready anyway.
    ///
    /// The primary purpose of this method is to prevent unexpected delays right before drawing.
    fn await_ready(&self);
}