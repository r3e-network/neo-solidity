use super::*;

#[test]
fn single_file_compilation_resolves_relative_imports() {
    let temp = tempfile::tempdir().expect("tempdir");
    let base_path = temp.path().join("Base.sol");
    let derived_path = temp.path().join("Derived.sol");

    fs::write(
        &base_path,
        r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.19;

        contract Base {
            function foo() public pure returns (uint256) {
                return 1;
            }
        }
        "#,
    )
    .expect("write Base.sol");

    fs::write(
        &derived_path,
        r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.19;
        import "./Base.sol";

        contract Derived is Base {}
        "#,
    )
    .expect("write Derived.sol");

    let resolved =
        resolve_solidity_sources_with_imports(&derived_path, &[]).expect("resolve imports");
    assert_eq!(resolved.files.len(), 2, "expected Base + Derived sources");

    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("compilation with imports should succeed");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Base"));
    assert!(names.contains(&"Derived"));
}

#[test]
fn single_file_compilation_resolves_imports_via_include_paths() {
    let temp = tempfile::tempdir().expect("tempdir");
    let lib_dir = temp.path().join("lib");
    fs::create_dir_all(&lib_dir).expect("create lib dir");

    let lib_path = lib_dir.join("Lib.sol");
    let entry_path = temp.path().join("Entry.sol");

    fs::write(
        &lib_path,
        r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.19;

        library Lib {
            function add(uint256 a, uint256 b) internal pure returns (uint256) {
                return a + b;
            }
        }
        "#,
    )
    .expect("write Lib.sol");

    fs::write(
        &entry_path,
        r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.19;
        import "Lib.sol";

        contract Entry {
            function sum(uint256 a, uint256 b) public pure returns (uint256) {
                return Lib.add(a, b);
            }
        }
        "#,
    )
    .expect("write Entry.sol");

    let resolved = resolve_solidity_sources_with_imports(&entry_path, &[lib_dir])
        .expect("resolve imports via include path");
    assert_eq!(resolved.files.len(), 2, "expected Lib + Entry sources");

    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("compilation with include path imports should succeed");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Entry"));
}

#[test]
fn single_file_compilation_accepts_import_aliasing_for_dependency_resolution() {
    let temp = tempfile::tempdir().expect("tempdir");
    let lib_path = temp.path().join("Lib.sol");
    let entry_path = temp.path().join("Entry.sol");

    fs::write(
        &lib_path,
        r#"
        pragma solidity ^0.8.19;

        library Lib {
            function add(uint256 a, uint256 b) internal pure returns (uint256) {
                return a + b;
            }
        }
        "#,
    )
    .expect("write Lib.sol");

    fs::write(
        &entry_path,
        r#"
        pragma solidity ^0.8.19;
        import { Lib as MathLib } from "./Lib.sol";

        contract Entry {
            function sum(uint256 a, uint256 b) public pure returns (uint256) {
                return Lib.add(a, b);
            }
        }
        "#,
    )
    .expect("write Entry.sol");

    let resolved = resolve_solidity_sources_with_imports(&entry_path, &[])
        .expect("alias import should resolve");
    assert_eq!(resolved.files.len(), 2, "expected Lib + Entry sources");

    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("compilation with aliased import syntax should succeed");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Entry"));
}

#[test]
fn single_file_compilation_accepts_global_symbol_imports_for_dependency_resolution() {
    let temp = tempfile::tempdir().expect("tempdir");
    let lib_path = temp.path().join("Lib.sol");
    let entry_path = temp.path().join("Entry.sol");

    fs::write(
        &lib_path,
        r#"
        pragma solidity ^0.8.19;

        library Lib {
            function add(uint256 a, uint256 b) internal pure returns (uint256) {
                return a + b;
            }
        }
        "#,
    )
    .expect("write Lib.sol");

    fs::write(
        &entry_path,
        r#"
        pragma solidity ^0.8.19;
        import * as LibNS from "./Lib.sol";

        contract Entry {
            function sum(uint256 a, uint256 b) public pure returns (uint256) {
                return Lib.add(a, b);
            }
        }
        "#,
    )
    .expect("write Entry.sol");

    let resolved = resolve_solidity_sources_with_imports(&entry_path, &[])
        .expect("global-symbol import should resolve");
    assert_eq!(resolved.files.len(), 2, "expected Lib + Entry sources");

    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("compilation with wildcard import syntax should succeed");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Entry"));
}

#[test]
fn single_file_compilation_resolves_aliased_symbol_binding() {
    let temp = tempfile::tempdir().expect("tempdir");
    let lib_path = temp.path().join("Lib.sol");
    let entry_path = temp.path().join("Entry.sol");

    fs::write(
        &lib_path,
        r#"
        pragma solidity ^0.8.19;

        library Lib {
            function add(uint256 a, uint256 b) internal pure returns (uint256) {
                return a + b;
            }
        }
        "#,
    )
    .expect("write Lib.sol");

    fs::write(
        &entry_path,
        r#"
        pragma solidity ^0.8.19;
        import { Lib as MathLib } from "./Lib.sol";

        contract Entry {
            function sum(uint256 a, uint256 b) public pure returns (uint256) {
                return MathLib.add(a, b);
            }
        }
        "#,
    )
    .expect("write Entry.sol");

    let resolved = resolve_solidity_sources_with_imports(&entry_path, &[])
        .expect("alias import should resolve");
    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("aliased symbol binding should compile");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Entry"));
}

#[test]
fn single_file_compilation_resolves_wildcard_namespace_binding() {
    let temp = tempfile::tempdir().expect("tempdir");
    let lib_path = temp.path().join("Lib.sol");
    let entry_path = temp.path().join("Entry.sol");

    fs::write(
        &lib_path,
        r#"
        pragma solidity ^0.8.19;

        library Lib {
            function add(uint256 a, uint256 b) internal pure returns (uint256) {
                return a + b;
            }
        }
        "#,
    )
    .expect("write Lib.sol");

    fs::write(
        &entry_path,
        r#"
        pragma solidity ^0.8.19;
        import * as LibNS from "./Lib.sol";

        contract Entry {
            function sum(uint256 a, uint256 b) public pure returns (uint256) {
                return LibNS.Lib.add(a, b);
            }
        }
        "#,
    )
    .expect("write Entry.sol");

    let resolved = resolve_solidity_sources_with_imports(&entry_path, &[])
        .expect("global-symbol import should resolve");
    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("wildcard namespace binding should compile");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Entry"));
}

#[test]
fn single_file_compilation_resolves_wildcard_namespace_type_cast_binding() {
    let temp = tempfile::tempdir().expect("tempdir");
    let iface_path = temp.path().join("IFoo.sol");
    let entry_path = temp.path().join("Entry.sol");

    fs::write(
        &iface_path,
        r#"
        pragma solidity ^0.8.19;

        interface IFoo {
            function foo() external;
        }
        "#,
    )
    .expect("write IFoo.sol");

    fs::write(
        &entry_path,
        r#"
        pragma solidity ^0.8.19;
        import * as LibNS from "./IFoo.sol";

        contract Entry {
            function ping(address target) public {
                LibNS.IFoo(target).foo();
            }
        }
        "#,
    )
    .expect("write Entry.sol");

    let resolved = resolve_solidity_sources_with_imports(&entry_path, &[])
        .expect("global-symbol import should resolve");
    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("wildcard namespace type-cast binding should compile");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Entry"));
}

#[test]
fn single_file_compilation_resolves_wildcard_namespace_selector_binding() {
    let temp = tempfile::tempdir().expect("tempdir");
    let iface_path = temp.path().join("IFoo.sol");
    let entry_path = temp.path().join("Entry.sol");

    fs::write(
        &iface_path,
        r#"
        pragma solidity ^0.8.19;

        interface IFoo {
            function foo() external;
        }
        "#,
    )
    .expect("write IFoo.sol");

    fs::write(
        &entry_path,
        r#"
        pragma solidity ^0.8.19;
        import * as LibNS from "./IFoo.sol";

        contract Entry {
            function selectorOfFoo() public pure returns (bytes4) {
                return LibNS.IFoo.foo.selector;
            }
        }
        "#,
    )
    .expect("write Entry.sol");

    let resolved = resolve_solidity_sources_with_imports(&entry_path, &[])
        .expect("global-symbol import should resolve");
    let artifacts = compile_contracts(&resolved.combined_source, false, 2)
        .expect("wildcard namespace selector binding should compile");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert!(names.contains(&"Entry"));
}
