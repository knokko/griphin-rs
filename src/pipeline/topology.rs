pub enum PrimitiveTopology {
    Points,
    Lines,
    LineStrips { primitive_restart: bool },
    Triangles,
    TriangleStrips { primitive_restart: bool },
}
