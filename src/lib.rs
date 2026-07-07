use swc_core::common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use swc_core::plugin::{metadata::TransformPluginProgramMetadata, plugin_transform};

/// 最简单的 SWC 插件：在每个文件顶部注入 `console.log('swc-plugin-canyon')`
pub struct InjectConsoleLog;

impl VisitMut for InjectConsoleLog {
    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        items.visit_mut_children_with(self);

        let log_stmt = Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident::new("console".into(), DUMMY_SP))),
                    prop: MemberProp::Ident(Ident::new("log".into(), DUMMY_SP)),
                }))),
                args: vec![ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: "swc-plugin-canyon".into(),
                        raw: None,
                    }))),
                }],
                type_args: None,
            })),
        });

        items.insert(0, ModuleItem::Stmt(log_stmt));
    }
}

#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut InjectConsoleLog);
    program
}
