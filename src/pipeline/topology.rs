/// This enum lists the possible topologies of input vertices or indices.
///
/// If there are no indices, the list of vertices that is submitted to the GPU
/// is just that: a list of vertices. The topology is needed to give meaning
/// to these vertices. For instance, the *Lines* topology tells the GPU that
/// it should draw a line between the first and second vertex, between the
/// third and fourth vertex, between the fifth and sixth vertex...
/// See the documentation of the individual enum options for more information.
///
/// If there *are* indices, the meaning of the list of vertices is defined by
/// the list of indices. Every index in the list of indices points to a vertex
/// in the list of vertices. So if the indices are [0,3,2,5], it basically
/// means [first vertex, fourth vertex, third vertex, sixth vertex]. But as
/// you can see, this is still meaningless, so the GPU also needs a topology
/// to find out what this means. If the topology is *Lines*, this means that
/// it should draw a line between the first vertex and fourth vertex, and a
/// line between the third vertex and sixth vertex.
pub enum PrimitiveTopology {
    /// A point should be drawn at the location of each vertex
    Points,

    /// Lines should be drawn between the first and second vertex, third and
    /// fourth vertex, fifth and sixth vertex...
    Lines,

    /// Lines should be drawn between the first and second vertex, the second
    /// and third vertex, the third and fourth vertex...
    ///
    /// If *primitive_restart* is set to true, it will be possible to finish
    /// a line strip and begin a new strip. (So you don't need to necessarily
    /// have 1 big line strip, but you can also use multiple smaller line strips
    /// instead.) TODO Define how to break them up
    LineStrips { primitive_restart: bool },

    /// Triangles should be drawn between the first, second, and third vertex,
    /// between the fourth, fifth, and sixth vertex, between the seventh, eigth,
    /// and ninth vertex...
    Triangles,

    /// Triangles should be drawn between the first, second, and third vertex,
    /// between the second, third, and fourth vertex, between the third, fourth,
    /// and fifth vertex...
    ///
    /// If *primitive_restart* is set to true, it will be possible to finish
    /// a triangle strip and begin a new strip. (So you don't need to necessarily
    /// have 1 big triangle strip, but you can also have multiple smaller ones.)
    /// TODO Define how to break them up
    TriangleStrips { primitive_restart: bool },
}
