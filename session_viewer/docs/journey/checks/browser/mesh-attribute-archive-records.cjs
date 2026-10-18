const assert=require('node:assert/strict'),base=require('./element-archive-records.cjs');
function fields(bytes){
 let at=0;const read=()=>{let v=0n;for(let shift=0n;shift<70n;shift+=7n){assert(at<bytes.length);const b=bytes[at++];v|=BigInt(b&127)<<shift;if(!(b&128))return v;}throw Error('Invalid protobuf varint');};const out=[];
 while(at<bytes.length){const key=Number(read()),tag=key>>>3,wire=key&7;assert(tag>0);let start=at,end;
  if(wire===0){read();end=at;}else if(wire===2){const n=Number(read());assert(Number.isSafeInteger(n)&&n>=0);start=at;end=at+n;at=end;}else{const n=wire===1?8:wire===5?4:0;assert(n);end=at+n;at=end;}
  assert(end<=bytes.length);out.push([tag,wire,bytes.subarray(start,end).toString('hex')]);
 }return out;
}
function sort(rows,tags){for(const tag of tags){const indexes=rows.flatMap((row,i)=>row[0]===tag?[i]:[]),values=indexes.map(i=>rows[i]).sort((a,b)=>JSON.stringify(a[2]).localeCompare(JSON.stringify(b[2])));indexes.forEach((at,i)=>rows[at]=values[i]);}return rows;}
function wire(hex,kind){const rows=fields(Buffer.from(hex,'hex'));
 for(const row of rows){if(row[1]!==2)continue;const tag=row[0];let child;
  if((kind==='vertex-entry'||kind==='face-entry')&&tag===2)child=kind==='vertex-entry'?'vertex':'face';
  if(kind==='vertex'&&tag===4||kind==='face'&&tag===2||kind==='edge'&&tag===3)child='attribute';
  if(child)row[2]=wire(row[2],child);
 }return sort(rows,kind==='vertex'?[4]:kind==='face'?[2]:kind==='edge'?[3]:[]);
}
function walk(rows,kind){
 for(const row of rows){const tag=row[0];if(kind==='mesh'){
   const child=({3:'vertex-entry',4:'face-entry',6:'edge',7:'attribute',8:'attribute',9:'attribute'})[tag];
   if(child&&row[1]===2)row[2]=wire(row[2],child);
  }else if(Array.isArray(row[2])){
   const child=kind==='archive'&&tag===1?'session':kind==='session'&&(tag===3||tag===8)?'objects':kind==='objects'&&tag===9?'mesh':kind==='objects'&&tag===15?'element':kind==='element'&&tag===4?'mesh':null;
   if(child)walk(row[2],child);
  }
 }return kind==='mesh'?sort(rows,[3,4,7,8,9,17]):rows;
}
module.exports=bytes=>walk(base(bytes),'archive');
