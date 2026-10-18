module.exports=async(page,helpers)=>{
    await require('./display-scale.cjs')(page,helpers);
    await require('./labels.cjs').focused(page,helpers);
};
