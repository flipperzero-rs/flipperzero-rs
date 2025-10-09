use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Expr, ExprArray, Ident, Item, ItemMod, ReturnType, Stmt, Token,
    parse::{self, Parse},
    punctuated::Punctuated,
    spanned::Spanned,
    token,
};

mod deassert;

struct TestRunner {
    manifest_args: Punctuated<TestRunnerArg, Token![,]>,
    test_suites: ExprArray,
}

impl Parse for TestRunner {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut manifest_args = Punctuated::new();
        if !input.peek(token::Bracket) {
            loop {
                if input.is_empty() || input.peek(token::Bracket) {
                    break;
                }
                let value = input.parse()?;
                manifest_args.push_value(value);
                if input.is_empty() || input.peek(token::Bracket) {
                    break;
                }
                let punct = input.parse()?;
                manifest_args.push_punct(punct);
            }
        };

        let test_suites = input.parse()?;

        Ok(TestRunner {
            manifest_args,
            test_suites,
        })
    }
}

struct TestRunnerArg {
    ident: Ident,
    eq_token: Token![=],
    value: Box<Expr>,
}

impl Parse for TestRunnerArg {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let eq_token = input.parse()?;
        let value = input.parse()?;
        Ok(TestRunnerArg {
            ident,
            eq_token,
            value,
        })
    }
}

#[proc_macro]
pub fn tests_runner(args: TokenStream) -> TokenStream {
    match tests_runner_impl(args) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn tests_runner_impl(args: TokenStream) -> parse::Result<TokenStream> {
    let TestRunner {
        manifest_args,
        test_suites,
    } = syn::parse(args)?;

    let test_suites = test_suites
        .elems
        .into_iter()
        .map(|attr| {
            let mut module = String::new();
            for token in attr.to_token_stream() {
                module.push_str(&token.to_string());
            }
            let module = module.trim_start_matches("crate::");

            (
                quote!(#attr::__test_list().len()),
                quote!(#attr::__test_list().iter().copied().map(|(name, test_fn)| (#module, name, test_fn))),
            )
        })
        .collect::<Vec<_>>();

    let test_counts = test_suites.iter().map(|(count, _)| count);
    let test_lists = test_suites.iter().map(|(_, list)| list);

    let manifest_args = manifest_args.into_iter().map(
        |TestRunnerArg {
             ident,
             eq_token,
             value,
         }| { quote!(#ident #eq_token #value) },
    );

    Ok(quote!(
        #[cfg(test)]
        mod __test_runner {
            // Required for panic handler
            extern crate flipperzero_rt;

            // Required for allocator
            #[cfg(feature = "alloc")]
            extern crate flipperzero_alloc;

            use flipperzero_rt::{entry, manifest};

            manifest!(#(#manifest_args),*);
            entry!(main);

            const fn test_count() -> usize {
                let ret = 0;
                #( let ret = ret + #test_counts; )*
                ret
            }

            fn test_list() -> impl Iterator<Item = (&'static str, &'static str, ::flipperzero_test::TestFn)> + Clone {
                let ret = ::core::iter::empty();
                #( let ret = ret.chain(#test_lists); )*
                ret
            }

            // Test runner entry point
            pub(super) fn main(args: Option<&::core::ffi::CStr>) -> i32 {
                let args = ::flipperzero_test::__macro_support::Args::parse(args);
                match ::flipperzero_test::__macro_support::run_tests(test_count(), test_list(), args) {
                    Ok(()) => 0,
                    Err(e) => e,
                }
            }
        }

        #[cfg(all(test, miri))]
        #[start]
        fn main(argc: isize, argv: *const *const u8) -> isize {
            // TODO: Is there any benefit to Miri in hooking up the binary arguments to
            // the test runner?
            let ret = __test_runner::main(None);

            // Clean up app state.
            ::flipperzero_rt::__macro_support::__wait_for_thread_completion();

            ret.try_into().unwrap_or(isize::MAX)
        }
    )
    .into())
}

#[proc_macro_attribute]
pub fn tests(args: TokenStream, input: TokenStream) -> TokenStream {
    match tests_impl(args, input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn tests_impl(args: TokenStream, input: TokenStream) -> parse::Result<TokenStream> {
    if !args.is_empty() {
        return Err(parse::Error::new(
            Span::call_site(),
            "`#[tests]` attribute takes no arguments",
        ));
    }

    let module: ItemMod = syn::parse(input)?;

    let items = if let Some(content) = module.content {
        content.1
    } else {
        return Err(parse::Error::new(
            module.span(),
            "module must be inline (e.g. `mod foo {}`)",
        ));
    };

    let mut tests = vec![];
    let mut test_cfgs = vec![];
    let mut untouched_tokens = vec![];
    for item in items {
        match item {
            Item::Fn(mut f) => {
                let mut is_test = false;
                let mut cfg = vec![];

                // Find and extract the `#[test]` and `#[cfg(..)] attributes, if present.
                f.attrs.retain(|attr| {
                    if attr.path.is_ident("test") {
                        is_test = true;
                        false
                    } else {
                        if attr.path.is_ident("cfg") {
                            cfg.push(attr.clone());
                        }
                        true
                    }
                });

                if is_test {
                    // Enforce expected function signature.
                    if !f.sig.inputs.is_empty() {
                        return Err(parse::Error::new(
                            f.sig.inputs.span(),
                            "`#[test]` function must have signature `fn()`",
                        ));
                    }
                    if !matches!(f.sig.output, ReturnType::Default) {
                        return Err(parse::Error::new(
                            f.sig.output.span(),
                            "`#[test]` function must have signature `fn()`",
                        ));
                    }

                    // Add a `TestResult` return type.
                    f.sig.output = syn::parse(quote!(-> ::flipperzero_test::TestResult).into())?;

                    // Replace `assert` macros in the test with `TestResult` functions.
                    f.block = deassert::box_block(f.block)?;

                    // Enforce that the test doesn't return anything. This is somewhat
                    // redundant with the function signature check above, as the compiler
                    // will enforce that no value is returned by the unmodified function.
                    // However, in certain cases this results in better errors, due to us
                    // appending an `Ok(())` that can interfere with the previous expression.
                    check_ret_block(&mut f.block.stmts)?;

                    // Append an `Ok(())` to the test.
                    f.block.stmts.push(Stmt::Expr(syn::parse(
                        quote!(::core::result::Result::Ok(())).into(),
                    )?));

                    tests.push(f);
                    test_cfgs.push(cfg);
                } else {
                    untouched_tokens.push(Item::Fn(f));
                }
            }
            _ => {
                untouched_tokens.push(item);
            }
        }
    }

    let ident = module.ident;
    let test_names = tests.iter().zip(test_cfgs).map(|(test, cfg)| {
        let ident = &test.sig.ident;
        let name = ident.to_string();
        quote! {
            #(#cfg)*
            (#name, #ident)
        }
    });

    Ok(quote!(
        #[cfg(test)]
        pub(crate) mod #ident {
            #(#untouched_tokens)*

            #(#tests)*

            pub(crate) const fn __test_list() -> &'static [(&'static str, ::flipperzero_test::TestFn)] {
                &[#(#test_names), *]
            }
        }
    )
    .into())
}

fn check_ret_block(stmts: &mut [Stmt]) -> parse::Result<()> {
    if let Some(stmt) = stmts.last_mut() {
        if let Stmt::Expr(expr) = stmt {
            if let Some(new_stmt) = check_ret_expr(expr)? {
                *stmt = new_stmt;
            }
        }
    }
    Ok(())
}

fn check_ret_expr(expr: &mut Expr) -> parse::Result<Option<Stmt>> {
    match expr {
        // If `expr` is a block that implicitly returns `()`, do nothing.
        Expr::ForLoop(_) | Expr::While(_) => Ok(None),
        // If `expr` is a block, recurse into it.
        Expr::Async(e) => check_ret_block(&mut e.block.stmts).map(|()| None),
        Expr::Block(e) => check_ret_block(&mut e.block.stmts).map(|()| None),
        Expr::If(e) => {
            // Checking the first branch is sufficient; the compiler will enforce that the
            // other branches match.
            check_ret_block(&mut e.then_branch.stmts).map(|()| None)
        }
        Expr::Loop(e) => check_ret_block(&mut e.body.stmts).map(|()| None),
        Expr::Match(e) => {
            if let Some(arm) = e.arms.first_mut() {
                if let Some(stmt) = check_ret_expr(&mut arm.body)? {
                    *arm.body = Expr::Block(syn::parse(quote!({#stmt}).into())?);
                }
            }
            Ok(None)
        }
        Expr::TryBlock(e) => check_ret_block(&mut e.block.stmts).map(|()| None),
        Expr::Unsafe(e) => check_ret_block(&mut e.block.stmts).map(|()| None),
        // If `expr` implicitly returns `()`, append a semicolon.
        Expr::Assign(_) | Expr::AssignOp(_) => {
            Ok(Some(Stmt::Semi(expr.clone(), Token!(;)(expr.span()))))
        }
        Expr::Break(brk) if brk.expr.is_none() => {
            Ok(Some(Stmt::Semi(expr.clone(), Token!(;)(expr.span()))))
        }
        // For all other expressions, raise an error.
        _ => Err(parse::Error::new(
            expr.span(),
            "`#[test]` function must not return anything",
        )),
    }
}
