/// Represents a data 'kind' (like int or float) for a shader variable. Together 
/// with a *DataShape*, a *DataKind* forms a *DataType*. You can't construct new 
/// *DataKind*s; you can only use the built-in data types *INT*, *FLOAT*, and *BOOL*.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DataKind {
    long_name: &'static str,
    short_name: &'static str,
}

impl DataKind {
    /// Gets the 'long' or full name of this *DataKind*. The result is either
    /// "int", "float", or "bool".
    pub fn get_long_name(&self) -> &'static str {
        self.long_name
    }

    /// Gets the 'short' or glsl name of this *DataKind*. The result is either
    /// "i", "", or "b". This method is made to construct GLSL variable names
    /// and should normally only be used by Griphin implementations.
    pub fn get_short_name(&self) -> &'static str {
        self.short_name
    }
}

const fn data_kind(long_name: &'static str, short_name: &'static str) -> DataKind {
    DataKind {
        long_name,
        short_name,
    }
}

/// The *int* *DataKind*. This will be a 32-bit signed integer in GLSL.
pub const INT: DataKind = data_kind("int", "i");
/// The *float* *DataKind*. This will be a 32-bit (single precision) float in GLSL.
pub const FLOAT: DataKind = data_kind("float", "");
/// The *bool* *DataKind*.
pub const BOOL: DataKind = data_kind("bool", "b");

/// Represents the 'shape' of a variable in GLSL (like vec2 and mat4). In
/// combination with a *DataKind*, this struct can form a *DataType*. You can't
/// create your own instances of this struct; you can only use *SINGLE*, *VEC2*,
/// *VEC3*, *VEC4*, *MAT2*, *MAT3*, AND *MAT4*.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DataShape {
    raw_name: &'static str,
    size: u8,
}

impl DataShape {
    /// Gets the 'raw' name of this *DataShape*. Every type declaration of a GLSL
    /// variable of this type will have this raw name as prefix.
    pub fn get_raw_name(&self) -> &'static str {
        self.raw_name
    }

    /// Gets the size of this *DataShape* in *units* where 1 *unit* is 4 bytes.
    pub fn get_size(&self) -> u8 {
        self.size
    }
}

const fn data_shape(raw_name: &'static str, size: u8) -> DataShape {
    DataShape { raw_name, size }
}

/// The *DataShape* to indicate that a variable has a single value.
pub const SINGLE: DataShape = data_shape("", 1);
/// The *DataShape* to indicate that a variable is a vector of 2 values.
pub const VEC2: DataShape = data_shape("vec2", 2);
/// The *DataShape* to indicate that a variable is a vector of 3 values.
pub const VEC3: DataShape = data_shape("vec3", 3);
/// The *DataShape* to indicate that a variable is a vector of 4 values.
pub const VEC4: DataShape = data_shape("vec4", 4);
/// The *DataShape* to indicate that a variable is a matrix of 2 x 2 values.
pub const MAT2: DataShape = data_shape("mat2", 4);
/// The *DataShape* to indicate that a variable is a matrix of 3 x 3 values.
pub const MAT3: DataShape = data_shape("mat3", 9);
/// The *DataShape* to indicate that a variable is a matrix of 4 x 4 values.
pub const MAT4: DataShape = data_shape("mat4", 16);

/// Represents the data type of a shader variable. This is a tuple of *DataKind*
/// and *DataShape*. Unlike *DataKind* and *DataType*, the *new* function of this
/// struct can be used to create (new) *DataType*s.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DataType {
    kind: DataKind,
    shape: DataShape,
}

impl DataType {
    /// Constructs a new *DataType* with the given *DataKind* and *DataShape*. This
    /// is a very simple constructor that does nothing special.
    pub fn new(kind: DataKind, shape: DataShape) -> DataType {
        DataType { kind, shape }
    }

    /// Gets the *DataKind* of this data type.
    pub fn get_kind(&self) -> DataKind {
        self.kind
    }

    /// Gets the *DataShape* of this data type.
    pub fn get_shape(&self) -> DataShape {
        self.shape
    }

    /// Gets the GLSL name of this *DataType*. That name can be injected into
    /// GLSL code directly and is meant to help Griphin implementations.
    pub fn get_glsl_name(&self) -> String {
        if self.shape == SINGLE {
            self.kind.long_name.to_string()
        } else {
            self.kind.short_name.to_string() + self.shape.raw_name
        }
    }
}
