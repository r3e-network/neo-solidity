use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ManifestPermissionsMode {
    Merge,
    ReplaceWildcards,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ManifestPermissionMethods {
    All,
    Some(BTreeSet<String>),
}

impl ManifestPermissionMethods {
    fn merge_in(&mut self, other: ManifestPermissionMethods) {
        match (self, other) {
            (ManifestPermissionMethods::All, _) => {}
            (this, ManifestPermissionMethods::All) => {
                *this = ManifestPermissionMethods::All;
            }
            (ManifestPermissionMethods::Some(set), ManifestPermissionMethods::Some(other_set)) => {
                set.extend(other_set);
            }
        }
    }

    fn is_wildcard(&self) -> bool {
        matches!(self, ManifestPermissionMethods::All)
    }
}

type ManifestPermissionMap = BTreeMap<String, ManifestPermissionMethods>;

#[derive(Clone, Debug)]
pub(crate) struct ManifestPermissionsOverride {
    mode: ManifestPermissionsMode,
    permissions: ManifestPermissionMap,
}

#[derive(Clone, Debug)]
pub struct CompileOptions {
    pub optimizer_level: u8,
    pub use_callt: bool,
    pub deny_wildcard_permissions: bool,
    pub deny_wildcard_contracts: bool,
    pub deny_wildcard_methods: bool,
    pub(crate) manifest_permissions: Option<ManifestPermissionsOverride>,
}

impl CompileOptions {
    pub fn new(optimizer_level: u8, use_callt: bool) -> Self {
        Self {
            optimizer_level,
            use_callt,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: false,
            deny_wildcard_methods: false,
            manifest_permissions: None,
        }
    }
}
