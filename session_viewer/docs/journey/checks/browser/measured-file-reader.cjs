module.exports=async(page,helpers)=>{
    await require('./document-phases.cjs')(page,helpers);
    await require('./imported-interactions.cjs')(page,helpers);
    console.log(`${helpers.step.id}: full original Element import, measured file/document phases, atomic failure, child editing, interaction visibility and two archive roundtrips passed`);
};
