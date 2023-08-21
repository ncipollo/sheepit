use crate::version::update::VersionUpdate;

pub enum Operation {
    BumpVersion(VersionUpdate),
    SetVersion(VersionUpdate)
}

pub enum BumpMode {
    Major,
    Minor,
    Patch
}