use crate::*;

#[derive(Clone)]
pub struct NodeOutput {

    destination: NodeOutputDestination,
    shader_variable_name: String
}

#[derive(Clone)]
pub enum NodeOutputDestination {

    Internal(GridState),
    External(Grid)
}