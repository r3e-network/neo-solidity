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
struct CompileOptions {
    optimizer_level: u8,
    use_callt: bool,
    deny_wildcard_permissions: bool,
    deny_wildcard_contracts: bool,
    deny_wildcard_methods: bool,
    manifest_permissions: Option<ManifestPermissionsOverride>,
}
