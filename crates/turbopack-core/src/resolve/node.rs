use turbo_tasks::Vc;
use turbo_tasks_fs::FileSystemPath;

use super::options::{
    ConditionValue, ResolutionConditions, ResolveInPackage, ResolveIntoPackage, ResolveModules,
    ResolveOptions,
};

#[turbo_tasks::function]
pub fn node_cjs_resolve_options(root: Vc<FileSystemPath>) -> Vc<ResolveOptions> {
    let conditions: ResolutionConditions = [
        ("node".to_string(), ConditionValue::Set),
        ("require".to_string(), ConditionValue::Set),
    ]
    .into();
    let extensions = vec![
        ".js".to_string().into(),
        ".json".to_string().into(),
        ".node".to_string().into(),
    ];
    ResolveOptions {
        extensions,
        modules: vec![ResolveModules::Nested(
            root,
            vec!["node_modules".to_string().into()],
        )],
        into_package: vec![
            ResolveIntoPackage::ExportsField {
                conditions: conditions.clone(),
                unspecified_conditions: ConditionValue::Unset,
            },
            ResolveIntoPackage::MainField {
                field: "main".to_string().into(),
            },
        ],
        in_package: vec![ResolveInPackage::ImportsField {
            conditions,
            unspecified_conditions: ConditionValue::Unset,
        }],
        default_files: vec!["index".to_string().into()],
        ..Default::default()
    }
    .cell()
}

#[turbo_tasks::function]
pub fn node_esm_resolve_options(root: Vc<FileSystemPath>) -> Vc<ResolveOptions> {
    let conditions: ResolutionConditions = [
        ("node".to_string(), ConditionValue::Set),
        ("import".to_string(), ConditionValue::Set),
    ]
    .into();
    let extensions = vec![
        ".js".to_string().into(),
        ".json".to_string().into(),
        ".node".to_string().into(),
    ];
    ResolveOptions {
        fully_specified: true,
        extensions,
        modules: vec![ResolveModules::Nested(
            root,
            vec!["node_modules".to_string().into()],
        )],
        into_package: vec![
            ResolveIntoPackage::ExportsField {
                conditions: conditions.clone(),
                unspecified_conditions: ConditionValue::Unset,
            },
            ResolveIntoPackage::MainField {
                field: "main".to_string().into(),
            },
        ],
        in_package: vec![ResolveInPackage::ImportsField {
            conditions,
            unspecified_conditions: ConditionValue::Unset,
        }],
        default_files: vec!["index".to_string().into()],
        ..Default::default()
    }
    .cell()
}
