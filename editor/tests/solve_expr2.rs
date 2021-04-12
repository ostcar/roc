#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate indoc;

use bumpalo::Bump;
use roc_can::expected::Expected;
use roc_collections::all::MutMap;
use roc_editor::lang::solve;
use roc_editor::lang::{
    constrain::constrain_expr,
    constrain::Constraint,
    expr::{str_to_expr2, Env},
    pool::Pool,
    scope::Scope,
    types::Type2,
};
use roc_module::ident::{Lowercase, TagName};
use roc_module::symbol::Symbol;
use roc_module::symbol::{IdentIds, ModuleIds};
use roc_region::all::Region;
use roc_types::solved_types::Solved;
use roc_types::subs::{Subs, Variable};
use roc_types::{pretty_print::content_to_string, subs::VarStore, types::Type};

fn ed_constraint_to_can_constraint(
    constraint: roc_editor::lang::constrain::Constraint,
) -> roc_can::constraint::Constraint {
    match constraint {
        roc_editor::lang::constrain::Constraint::Eq(typ, expected, category, region) => {
            let new_typ = type2_to_type(&typ);
            let expected_typ = expected.get_type_ref();

            let expected_typ = type2_to_type(expected_typ);

            roc_can::constraint::Constraint::Eq(
                new_typ,
                expected.replace(expected_typ),
                category,
                region,
            )
        }
        _ => todo!("{:?}", constraint),
    }
}

fn type2_to_type(typ: &Type2) -> Type {
    match typ {
        Type2::Apply(symbol, _) => Type::Apply(*symbol, Vec::new()),
        Type2::Variable(var) => Type::Variable(*var),
        _ => todo!("{:?}", typ),
    }
}

fn run_solve(
    aliases: MutMap<Symbol, roc_types::types::Alias>,
    rigid_variables: MutMap<Variable, Lowercase>,
    constraint: Constraint,
    var_store: VarStore,
) -> (Solved<Subs>, solve::Env, Vec<solve::TypeError>) {
    let env = solve::Env {
        vars_by_symbol: MutMap::default(),
        aliases,
    };

    let mut subs = Subs::new(var_store.into());

    for (var, name) in rigid_variables {
        subs.rigid_var(var, name);
    }

    // Now that the module is parsed, canonicalized, and constrained,
    // we need to type check it.
    let mut problems = Vec::new();

    // Run the solver to populate Subs.
    let (solved_subs, solved_env) = solve::run(&env, &mut problems, subs, &constraint);

    (solved_subs, solved_env, problems)
}

fn infer_eq(actual: &str, expected_str: &str) {
    let mut env_pool = Pool::with_capacity(1024);
    let env_arena = Bump::new();
    let code_arena = Bump::new();

    let mut var_store = VarStore::default();
    let var = var_store.fresh();
    let dep_idents = IdentIds::exposed_builtins(8);

    let exposed_ident_ids = IdentIds::default();
    let mut module_ids = ModuleIds::default();
    let mod_id = module_ids.get_or_insert(&"ModId123".into());

    let mut env = Env::new(
        mod_id,
        &env_arena,
        &mut env_pool,
        &mut var_store,
        dep_idents,
        &module_ids,
        exposed_ident_ids,
    );

    let mut scope = Scope::new(env.home, env.pool, env.var_store);

    let region = Region::zero();

    let expr2_result = str_to_expr2(&code_arena, actual, &mut env, &mut scope, region);

    match expr2_result {
        Ok((expr, _)) => {
            let constraint = constrain_expr(
                &mut env,
                &expr,
                Expected::NoExpectation(Type2::Variable(var)),
            );

            let (mut solved, _, _) = run_solve(
                Default::default(),
                Default::default(),
                constraint,
                var_store,
            );

            let mut subs = solved.inner_mut();

            let content = subs.get(var).content;

            let actual_str = content_to_string(content, &mut subs, mod_id, &Default::default());

            assert_eq!(actual_str, expected_str);
        }
        Err(e) => panic!("syntax error {:?}", e),
    }
}

#[test]
fn constrain_str() {
    infer_eq(
        indoc!(
            r#"
            "type inference!"
            "#
        ),
        "Str",
    )
}
