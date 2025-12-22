use quote::quote;
use syn::{Block, Expr, ExprMacro, ExprTuple, Stmt, parse};

/// Find and replace macro assertions inside the given block with `return Err(..)`.
///
/// The following assertion macros are replaced:
/// - [`assert`]
/// - [`assert_eq`]
/// - [`assert_ne`]
pub(crate) fn box_block(mut block: Box<Block>) -> parse::Result<Box<Block>> {
    block.stmts = block_stmts(block.stmts)?;
    Ok(block)
}

/// Searches recursively through block statements to find and replace macro assertions
/// with `return Err(..)`.
///
/// The following assertion macros are replaced:
/// - [`assert`]
/// - [`assert_eq`]
/// - [`assert_ne`]
fn block_stmts(stmts: Vec<Stmt>) -> parse::Result<Vec<Stmt>> {
    stmts
        .into_iter()
        .map(|stmt| match stmt {
            Stmt::Expr(Expr::Block(mut e)) => {
                e.block.stmts = block_stmts(e.block.stmts)?;
                Ok(Stmt::Expr(Expr::Block(e)))
            }
            Stmt::Expr(Expr::Macro(m)) => expr_macro(m).map(Stmt::Expr),
            Stmt::Semi(Expr::Macro(m), trailing) => expr_macro(m).map(|m| Stmt::Semi(m, trailing)),
            _ => Ok(stmt),
        })
        .collect::<Result<_, _>>()
}

/// Replaces macro assertions with `return Err(..)`.
///
/// The following assertion macros are replaced:
/// - [`assert`]
/// - [`assert_eq`]
/// - [`assert_ne`]
fn expr_macro(m: ExprMacro) -> parse::Result<Expr> {
    if m.mac.path.is_ident("assert") {
        let tokens = m.mac.tokens;
        let tokens_str = tokens.to_string();
        syn::parse(
            quote!(
                if !(#tokens) {
                    return ::core::result::Result::Err(
                        ::core::concat!("assertion failed: ", #tokens_str).into(),
                    );
                }
            )
            .into(),
        )
    } else if m.mac.path.is_ident("assert_eq") {
        let (left, right, msg) = binary_macro(m.mac.tokens)?;
        let left_str = quote!(#left).to_string();
        let right_str = quote!(#right).to_string();
        let msg_str = if let Some(msg) = msg {
            quote!(Some(#msg))
        } else {
            quote!(None)
        };
        syn::parse(
            quote!(
                if #left != #right {
                    return ::core::result::Result::Err(
                        ::flipperzero_test::TestFailure::AssertEq {
                            left: #left_str,
                            right: #right_str,
                            msg: #msg_str,
                        }
                    );
                }
            )
            .into(),
        )
    } else if m.mac.path.is_ident("assert_ne") {
        let (left, right, msg) = binary_macro(m.mac.tokens)?;
        let left_str = quote!(#left).to_string();
        let right_str = quote!(#right).to_string();
        let msg_str = if let Some(msg) = msg {
            quote!(Some(#msg))
        } else {
            quote!(None)
        };
        syn::parse(
            quote!(
                if #left == #right {
                    return ::core::result::Result::Err(
                        ::flipperzero_test::TestFailure::AssertNe {
                            left: #left_str,
                            right: #right_str,
                            msg: #msg_str,
                        }
                    );
                }
            )
            .into(),
        )
    } else {
        Ok(Expr::Macro(m))
    }
}

fn binary_macro(tokens: proc_macro2::TokenStream) -> parse::Result<(Expr, Expr, Option<Expr>)> {
    let parts: ExprTuple = syn::parse(quote!((#tokens)).into())?;
    assert!(parts.elems.len() >= 2);
    let mut elems = parts.elems.into_iter();
    Ok((elems.next().unwrap(), elems.next().unwrap(), elems.next()))
}
