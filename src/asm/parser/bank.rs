use crate::*;


pub fn parse_directive_bankdef(
    state: &mut asm::parser::State)
    -> Result<(), ()>
{
    let tk_name = state.parser.expect(syntax::TokenKind::Identifier)?;
    let name = tk_name.excerpt.as_ref().unwrap().clone();

    state.parser.expect(syntax::TokenKind::BraceOpen)?;

    let mut bank = asm::Bank
    {
        name: name.clone(),
        wordsize: state.asm_state.cur_wordsize,
        addr_start: util::BigInt::from(0),
        addr_size: None,
        output_offset: None,
        fill: false,
        decl_span: Some(tk_name.span.clone()),
    };

    while !state.parser.next_is(0, syntax::TokenKind::BraceClose)
    {
        parse_bankdef_field(state, &mut bank)?;

        if !state.parser.next_is(0, syntax::TokenKind::BraceClose)
        {
            state.parser.expect_linebreak_or(syntax::TokenKind::Comma)?;
        }
    }

    state.parser.expect(syntax::TokenKind::BraceClose)?;

    state.asm_state.create_bank(bank, state.report.clone())?;

    Ok(())
}


fn parse_bankdef_field(
    state: &mut asm::parser::State,
    bank: &mut asm::Bank)
    -> Result<(), ()>
{
    let _tk_hash = state.parser.expect(syntax::TokenKind::Hash)?;
    let tk_field_name = state.parser.expect(syntax::TokenKind::Identifier)?;
    let field_name = tk_field_name.excerpt.as_ref().unwrap().clone();

    match field_name.as_ref()
    {
        "addr" =>
            bank.addr_start = asm::parser::parse_expr_bigint(state)?,
            
        "addr_end" =>
        {
            let addr_end = asm::parser::parse_expr_bigint(state)?;
            bank.addr_size = match (&addr_end - &bank.addr_start).checked_to_usize()
            {
                Some(size) => Some(size),
                None =>
                {
                    state.report.error_span(
                        "bank size overflows valid range",
                        &tk_field_name.span);
                    return Err(());
                }
            }
        }
        
        "size" => bank.addr_size =
            Some(asm::parser::parse_expr_usize(state)?),

        "outp" => bank.output_offset =
            Some(asm::parser::parse_expr_usize(state)?),

        "bits" => bank.wordsize = asm::parser::parse_expr_usize_fn(state, |u| match u
            {
                0 => None,
                _ => Some(u)
            })?,

        "fill" => bank.fill = true,

        _ =>
        {
            state.report.error_span("unknown bankdef field", &tk_field_name.span);
            return Err(());
        }
    }

    Ok(())
}


pub fn parse_directive_bank(
    state: &mut asm::parser::State)
    -> Result<(), ()>
{
    let tk_name = state.parser.expect(syntax::TokenKind::Identifier)?;
    let name = tk_name.excerpt.as_ref().unwrap().clone();

    state.asm_state.cur_bank = state.asm_state.find_bank(
        name,
        state.report.clone(),
        &tk_name.span)?;

    Ok(())
}