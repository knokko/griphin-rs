#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DataKind {

    long_name: &'static str,
    short_name: &'static str
}

impl DataKind {

    pub fn get_long_name(&self) -> &'static str {
        self.long_name
    }

    pub fn get_short_name(&self) -> &'static str {
        self.short_name
    }
}

const fn data_kind(long_name: &'static str, short_name: &'static str) -> DataKind {
    DataKind {long_name, short_name}
}

pub const INT: DataKind = data_kind("int", "i");
pub const FLOAT: DataKind = data_kind("float", "");
pub const BOOL: DataKind = data_kind("bool", "b");

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DataShape {

    raw_name: &'static str,
    size: u8
}

impl DataShape {

    pub fn get_raw_name(&self) -> &'static str {
        self.raw_name
    }

    pub fn get_size(&self) -> u8 {
        self.size
    }
}

const fn data_shape(raw_name: &'static str, size: u8) -> DataShape {
    DataShape {raw_name, size}
}

pub const SINGLE: DataShape = data_shape("", 1);
pub const VEC2: DataShape = data_shape("vec2", 2);
pub const VEC3: DataShape = data_shape("vec3", 3);
pub const VEC4: DataShape = data_shape("vec4", 4);
pub const MAT3: DataShape = data_shape("mat3", 9);
pub const MAT4: DataShape = data_shape("mat4", 16);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DataType {

    kind: DataKind,
    shape: DataShape
}

impl DataType {

    pub fn new(kind: DataKind, shape: DataShape) -> DataType {
        DataType { kind, shape }
    }

    pub fn get_kind(&self) -> DataKind {
        self.kind
    }

    pub fn get_shape(&self) -> DataShape {
        self.shape
    }

    pub fn get_glsl_name(&self) -> String {
        if self.shape == SINGLE {
            self.kind.long_name.to_string()
        } else {
            self.kind.short_name.to_string() + self.shape.raw_name
        }
    }
}