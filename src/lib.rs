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
  let Input { pattern, expr, rest, .. } = syn::parse_macro_input!(input as Input);

  let match_inner = match rest {
    Rest::Simple(SimpleRest { bound, block }) => quote! { #bound => #block, },
    Rest::Complete(CompleteRest { arms, .. }) => quote! { #(#arms)* },
  };

  quote! {
    #[doc(hidden)]
    let __let_else_expr__ = #expr;
    let #pattern = __let_else_expr__ else {
      match __let_else_expr__ {
        #[allow(unused, dead_code)]
        #pattern => unsafe { std::hint::unreachable_unchecked() },
        #match_inner
      }
    };
  }.into()
}


struct Input {
  pattern:  syn::Pat,
  _sym_eq:  syn::Token![=],
  expr:     syn::Expr,
  _kw_else: syn::Token![else],
  rest:     Rest,
}

impl syn::parse::Parse for Input {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self {
      pattern:  input.call(syn::Pat::parse_single)?,
      _sym_eq:  input.parse()?,
      expr:     input.parse()?,
      _kw_else: input.parse()?,
      rest:     input.parse()?,
    })
  }
}


enum Rest {
  Simple(SimpleRest),
  Complete(CompleteRest),
}

impl syn::parse::Parse for Rest {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let fork = input.fork();
    if let Ok(simple) = fork.parse() {
      input.advance_to(&fork);
      return Ok(Self::Simple(simple));
    }

    let fork = input.fork();
    if let Ok(complete) = fork.parse() {
      input.advance_to(&fork);
      return Ok(Self::Complete(complete));
    }

    Err(input.error("Expected `as` or `match` after `else`"))
  }
}


struct SimpleRest {
  bound: RestBound,
  block: syn::Block,
}

impl syn::parse::Parse for SimpleRest {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self {
      bound: input.parse()?,
      block: input.parse()?,
    })
  }
}


enum RestBound {
  Pattern(syn::Pat),
  Ident(syn::Ident),
}

impl syn::parse::Parse for RestBound {
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

impl quote::ToTokens for RestBound {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      RestBound::Pattern(pat) => pat.to_tokens(tokens),
      RestBound::Ident(ident) => ident.to_tokens(tokens),
    }
  }
}


struct CompleteRest {
  _kw_match: syn::Token![match],
  arms:      Vec<syn::Arm>,
}

impl syn::parse::Parse for CompleteRest {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self {
      _kw_match: input.parse()?,
      arms: {
        let content;
        syn::braced!(content in input);

        let mut arms = Vec::new();
        while !content.is_empty() {
          arms.push(content.parse()?);
        }

        arms
      },
    })
  }
}
