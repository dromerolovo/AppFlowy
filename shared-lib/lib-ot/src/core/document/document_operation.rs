use crate::core::document::position::Position;
use crate::core::{NodeAttributes, NodeData, TextDelta};

#[derive(Clone)]
pub enum DocumentOperation {
    Insert(InsertOperation),
    Update(UpdateOperation),
    Delete(DeleteOperation),
    TextEdit(TextEditOperation),
}

impl DocumentOperation {
    pub fn path(&self) -> &Position {
        match self {
            DocumentOperation::Insert(insert) => &insert.path,
            DocumentOperation::Update(update) => &update.path,
            DocumentOperation::Delete(delete) => &delete.path,
            DocumentOperation::TextEdit(text_edit) => &text_edit.path,
        }
    }
    pub fn invert(&self) -> DocumentOperation {
        match self {
            DocumentOperation::Insert(insert_operation) => DocumentOperation::Delete(DeleteOperation {
                path: insert_operation.path.clone(),
                nodes: insert_operation.nodes.clone(),
            }),
            DocumentOperation::Update(update_operation) => DocumentOperation::Update(UpdateOperation {
                path: update_operation.path.clone(),
                attributes: update_operation.old_attributes.clone(),
                old_attributes: update_operation.attributes.clone(),
            }),
            DocumentOperation::Delete(delete_operation) => DocumentOperation::Insert(InsertOperation {
                path: delete_operation.path.clone(),
                nodes: delete_operation.nodes.clone(),
            }),
            DocumentOperation::TextEdit(text_edit_operation) => DocumentOperation::TextEdit(TextEditOperation {
                path: text_edit_operation.path.clone(),
                delta: text_edit_operation.inverted.clone(),
                inverted: text_edit_operation.delta.clone(),
            }),
        }
    }
    pub fn clone_with_new_path(&self, path: Position) -> DocumentOperation {
        match self {
            DocumentOperation::Insert(insert) => DocumentOperation::Insert(InsertOperation {
                path,
                nodes: insert.nodes.clone(),
            }),
            DocumentOperation::Update(update) => DocumentOperation::Update(UpdateOperation {
                path,
                attributes: update.attributes.clone(),
                old_attributes: update.old_attributes.clone(),
            }),
            DocumentOperation::Delete(delete) => DocumentOperation::Delete(DeleteOperation {
                path,
                nodes: delete.nodes.clone(),
            }),
            DocumentOperation::TextEdit(text_edit) => DocumentOperation::TextEdit(TextEditOperation {
                path,
                delta: text_edit.delta.clone(),
                inverted: text_edit.inverted.clone(),
            }),
        }
    }
    pub fn transform(a: &DocumentOperation, b: &DocumentOperation) -> DocumentOperation {
        match a {
            DocumentOperation::Insert(insert_op) => {
                let new_path = Position::transform(a.path(), b.path(), insert_op.nodes.len() as i64);
                b.clone_with_new_path(new_path)
            }
            DocumentOperation::Delete(delete_op) => {
                let new_path = Position::transform(a.path(), b.path(), delete_op.nodes.len() as i64);
                b.clone_with_new_path(new_path)
            }
            _ => b.clone(),
        }
    }
}

#[derive(Clone)]
pub struct InsertOperation {
    pub path: Position,
    pub nodes: Vec<NodeData>,
}

#[derive(Clone)]
pub struct UpdateOperation {
    pub path: Position,
    pub attributes: NodeAttributes,
    pub old_attributes: NodeAttributes,
}

#[derive(Clone)]
pub struct DeleteOperation {
    pub path: Position,
    pub nodes: Vec<NodeData>,
}

#[derive(Clone)]
pub struct TextEditOperation {
    pub path: Position,
    pub delta: TextDelta,
    pub inverted: TextDelta,
}
