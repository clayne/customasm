use crate::*;


#[derive(Debug)]
pub struct AlignDirective
{
    pub item_ref: util::ItemRef<Self>,
    pub align_size: usize,
}


pub fn define(
    _report: &mut diagn::Report,
    ast: &mut asm2::AstTopLevel,
    _decls: &mut asm2::ItemDecls,
    defs: &mut asm2::ItemDefs)
    -> Result<(), ()>
{
    for any_node in &mut ast.nodes
    {
        if let asm2::AstAny::DirectiveAlign(ref mut ast_align) = any_node
        {
            let item_ref = defs.align_directives.next_item_ref();

            let res = AlignDirective {
                item_ref,
                align_size: 0,
            };
            
            defs.align_directives.define(item_ref, res);
                
            ast_align.item_ref = Some(item_ref);
        }
    }


    Ok(())
}