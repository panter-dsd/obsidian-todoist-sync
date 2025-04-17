#[allow(dead_code)]
pub trait Project {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn provider(&self) -> String;
    fn description(&self) -> String;
    fn parent_id(&self) -> Option<String>;
    fn is_inbox(&self) -> bool;
    fn is_favorite(&self) -> bool;
}
