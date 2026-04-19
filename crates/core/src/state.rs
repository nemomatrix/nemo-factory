#[derive(Debug, Clone, PartialEq)]
pub enum SystemState {
    Init,
    SpecLocked,
    ArchLocked,
    DagReady,
    Executing,
    Verifying,
    Repairing,
    Completed,
    Failed,
}
