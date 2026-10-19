module.exports=async(page,helpers)=>{
    await require('./buffer-growth.cjs')(page,helpers);
    await require('./section-resources.cjs')(page,{...helpers,objectRecordBytes:176});
};
