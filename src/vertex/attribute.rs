use crate::*;

/// Represents an attribute of a *RawVertexDescription*. A *VertexAttribute* has a
/// name, *DataType*, and *AttributeKind*. It also has an offset, but that is
/// only for internal use.
/// 
/// To get an instance of this struct, use the *add_attribute* method of
/// an instance of *RawVertexDescription*. You should store every *VertexAttribute*
/// you create, because the implementation of *Vertex* corresponding to the
/// *VertexDescription* will need it to implement its *store* method.
#[derive(Debug)]
pub struct VertexAttribute {

    name: StringRef,
    data_type: DataType,
    kind: AttributeKind,
    pub(super) offset: usize
}

impl VertexAttribute {

    pub(super) fn new(name: &StringRef, data_type: DataType, kind: AttributeKind, offset: usize) -> Self {
        Self { name: name.clone(), data_type, kind, offset }
    }

    /// Gets the name of this attribute
    pub fn get_name(&self) -> &StringRef {
        &self.name
    }

    /// Gets the *DataType* of this attribute
    pub fn get_data_type(&self) -> DataType {
        self.data_type
    }

    /// Gets the *AttributeKind* of this attribute. This is only used for automatic debugging if a
    /// high debug level is used when creating a *VertexStore*.
    pub fn get_kind(&self) -> AttributeKind {
        self.kind
    }
}

/// This struct is a *handle* to a previously created *VertexAttribute*. Handles are returned by
/// the *add_attribute* method of *RawVertexDescription*. Griphin users should store these handles
/// in a struct that implements *VertexDescription*. Vertex structs will need these handles to
/// store their attribute values in a *VertexStore* in their *store* method.
///
/// See the documentation of *Vertex* for details and examples.
#[derive(Clone, Copy, Debug)]
pub struct VertexAttributeHandle {

    pub(super) offset: usize
}

/// This enum is used to define what the purpose of a vertex attribute is, and thus what the
/// expected values should be.
///
/// On low *DebugLevel*s, instances of this enum are ignored (almost) completely. But on high debug
/// levels, extensive validation checks are done upon creating *VertexStore*s.
///
/// See the documentation of the individual enum options for more information.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttributeKind {

    /// This should be used for attributes that represent a position (in the world or on the
    /// screen).
    ///
    /// This enum option has a single parameter *max* that indicates the maximum value that
    /// components of this position could reasonably have. For instance, 10_000.0 should be a
    /// reasonable value for scenes with a reasonable size.
    ///
    /// If the debug level is at least *High*, the following checks will be done, and warnings will
    /// be printed if any of them fails:
    ///
    /// 1) Checking that the *DataKind* of the attribute is *FLOAT*.
    ///
    /// 2) Checking that none of the component values is NaN.
    ///
    /// 3) Checking that none of the absolute values of the components is greater than *max*.
    ///
    /// 4) Checking that at most half of the vertices have the exact same position.
    ///
    /// If any of these checks fails, it is likely that (parts of) the object to be drawn will not
    /// be visible on the screen, without any errors being raised on the side of the graphics card.
    /// These checks could spare you a lot of debugging if such a situation occurs.
    Position{max: f32},

    /// This enum option should be used for vertex attributes that represent a normal vector. This
    /// enum option has no additional parameters.
    ///
    /// If the debug level is at least *High* and any of the following checks failed, warnings will
    /// be printed:
    ///
    /// 1) The *DataKind* of the attribute is not *FLOAT*
    ///
    /// 2) The *DataShape* of the attribute is not *VEC2* or *VEC3*
    ///
    /// 3) The squared length of the vector is NaN
    ///
    /// 4) The squared length of the vector is not between 0.95 and 1.05 (some rounding errors
    /// are permitted)
    ///
    /// If any of these checks fails, it is likely that your lighting will not behave like you
    /// expect. Debugging this is not as nasty as debugging bad positions, but can still be
    /// annoying. These checks could spare you a bit of debugging.
    Normal,

    /// This enum option should be used for attributes that represent texture coordinates and are
    /// floating point numbers (there is a separate enum option for integer texture coordinates).
    /// This enum option has no parameters.
    ///
    /// If the debug level is at least *High* and any of the following checks failed, warnings will
    /// be printed:
    ///
    /// 1) The *DataKind* of the attribute is not *FLOAT*
    ///
    /// 2) One of the components of the texture coordinates is NaN
    ///
    /// 3) One of the components of the texture coordinates is not between -0.05 and 1.05
    FloatTexCoords,

    /// This enum option should be used for attributes that represent texture coordinates and are
    /// integers (there is a separate enum option for floating point texture coordinates).
    ///
    /// This enum option has 1 parameter: the *texture_size*. That should be the maximum of the
    /// width and height of the biggest texture that you will use.
    ///
    /// If the debug level is at least *High* and any of the following checks failed, warnings will
    /// be printed:
    ///
    /// 1) The *DataKind* of the attribute is not *INT*
    ///
    /// 2) One of the components of the texture coordinates is negative
    ///
    /// 3) One of the components of the texture coordinates is not smaller than *texture_size*
    IntTexCoords{texture_size: u32},

    /// This enum option should be used for vertex attributes that will be used as index into some
    /// array.
    ///
    /// This enum option has a single parameter: the *bound* (length) of that array.
    ///
    /// If the debug level is at least *High* and any of the following checks failed, warnings will
    /// be printed:
    ///
    /// 1) The *DataKind* of the attribute is not *INT*
    ///
    /// 2) One of the indices is negative
    ///
    /// 3) One of the indices is not smaller than *bound*
    Index{bound: u32},

    /// This enum option should be used for vertex attributes that do not fit into the categories
    /// of any of the other options of this enum. No additional validation checks will be done for
    /// these attributes.
    Other
}