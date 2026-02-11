use quote::quote;
use syn::parse::discouraged::Speculative;


/// A procedural macro that enhance the `let-else` syntax in Rust.
///
/// # Example
///
/// ```rs, no_run
/// fn foo(value: Result<i32, String>) {
///   let_else!(Ok(value) = value else as Err(err) {
///     eprintln!("Error: {}", err);
///     return;
///   });
/// }
/// ```
#[proc_macro]
pub fn let_else(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  enum RestPat {
    Pattern(syn::Pat),
    Ident(syn::Ident),
  }

  impl syn::parse::Parse for RestPat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
      if let Err(_) = input.parse::<syn::Token![as]>() {
        return Ok(Self::Pattern(syn::parse_quote! { _ }));
      }

      let fork = input.fork();
      if let Ok(pattern) = fork.call(syn::Pat::parse_multi_with_leading_vert) {
        input.advance_to(&fork);
        return Ok(Self::Pattern(pattern));
      }

      if let Ok(ident) = input.parse() {
        return Ok(Self::Ident(ident));
      }

      Err(input.error("Expected a pattern or an identifier after `as`"))
    }
  }

  impl quote::ToTokens for RestPat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      match self {
        RestPat::Pattern(pat) => pat.to_tokens(tokens),
        RestPat::Ident(ident) => ident.to_tokens(tokens),
      }
    }
  }

  struct Input {
    pattern:  syn::Pat,
    _sym_eq:  syn::Token![=],
    expr:     syn::Expr,
    _kw_else: syn::Token![else],
    rest:     RestPat,
    block:    syn::Block,
  }

  impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
      Ok(Self {
        pattern:  input.call(syn::Pat::parse_single)?,
        _sym_eq:  input.parse()?,
        expr:     input.parse()?,
        _kw_else: input.parse()?,
        rest:     input.parse()?,
        block:    input.parse()?,
      })
    }
  }

  let Input { pattern, expr, rest, block, .. } = syn::parse_macro_input!(input as Input);

  quote! {
    #[doc(hidden)]
    let __let_else_expr__ = #expr;
    let #pattern = __let_else_expr__ else {
      match __let_else_expr__ {
        #[allow(unused, dead_code)]
        #pattern => unsafe { std::hint::unreachable_unchecked() },
        #rest => #block,
      }
    };
  }.into()
}
