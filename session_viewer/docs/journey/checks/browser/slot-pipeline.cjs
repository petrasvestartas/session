module.exports=async(page,helpers)=>{
    await require('./arena-cache.cjs')(page,{...helpers,instanceAllocationBytes:376});
};
