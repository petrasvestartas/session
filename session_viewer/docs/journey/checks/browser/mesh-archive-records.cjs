const assert=require('node:assert/strict');
function fields(bytes){
    let offset=0;const read=()=>{let value=0n,shift=0n;for(let n=0;n<10;n++){assert(offset<bytes.length);const b=bytes[offset++];value|=BigInt(b&127)<<shift;if(!(b&128))return value;shift+=7n;}throw Error('Invalid protobuf varint');},result=[];
    while(offset<bytes.length){const key=Number(read()),tag=key>>>3,wire=key&7;assert(tag>0);let payload;
        if(wire===0){const start=offset;read();payload=bytes.subarray(start,offset);}
        else if(wire===2){const size=Number(read());assert(Number.isSafeInteger(size)&&size>=0&&offset+size<=bytes.length);payload=bytes.subarray(offset,offset+size);offset+=size;}
        else{const size=wire===1?8:wire===5?4:0;assert(size&&offset+size<=bytes.length);payload=bytes.subarray(offset,offset+size);offset+=size;}
        result.push({tag,wire,payload});
    }return result;
}
function message(bytes,type){
    const items=fields(bytes).map(({tag,wire,payload})=>{let child;if(type==='archive'&&tag===1)child='session';if(type==='session'&&(tag===3||tag===8))child='objects';if(type==='objects'&&tag===9)child='mesh';
        return [tag,wire,child?message(payload,child):type==='archive'&&tag===2?JSON.parse(payload):payload.toString('hex')];});
    if(type==='mesh')for(const tag of [3,4,17]){const positions=items.flatMap((item,index)=>item[0]===tag?[index]:[]),sorted=positions.map(index=>items[index]).sort((a,b)=>a[2].localeCompare(b[2]));positions.forEach((index,n)=>items[index]=sorted[n]);}
    return items;
}
module.exports=bytes=>{const magic=Buffer.from('SESSION-VIEWER\x01\n');assert(bytes.subarray(0,magic.length).equals(magic));return message(bytes.subarray(magic.length),'archive');};
