/** Check actual Chrome ink against a CPU box ray after mouse orbit and wheel zoom. */
const assert = require('node:assert/strict');
const fs = require('node:fs');
const {createHash} = require('node:crypto');
const {chromium} = require('playwright');
const {PNG} = require('pngjs');
const state = page => page.locator('canvas').getAttribute('data-viewer-inspection').then(JSON.parse);
const ui = page => page.locator('canvas').getAttribute('data-viewer-ui').then(JSON.parse);
async function settle(page) {
    await page.waitForTimeout(250);
    await page.waitForFunction(() => !JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-inspection')).pick_busy);
}
async function command(page, text) {
    const {rect} = (await ui(page)).controls.find(c => c.key === 'command/input');
    await page.mouse.click((rect[0] + rect[2]) / 2, (rect[1] + rect[3]) / 2);
    await page.keyboard.press('Control+a');
    await page.keyboard.type(text);
    await page.keyboard.press('Enter');
    try {
        await page.waitForFunction(text => JSON.parse(document.querySelector('canvas').getAttribute('data-viewer-ui')).history.at(-1)?.startsWith(`> ${text}\n`), text);
    } catch (error) {
        console.error('Command timeout', text, await ui(page));
        fs.mkdirSync('target/course-checks', {recursive:true});
        await page.screenshot({path:'target/course-checks/close-up-command-failure.png'});
        throw error;
    }
    await settle(page);
}
const dot = (a,b) => a.reduce((n,x,i) => n + x*b[i],0);
const cross = (a,b) => [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
function eye(s) {
    const m=s.mvp, a=[m[0],m[4],m[8]],b=[m[1],m[5],m[9]],c=[m[3],m[7],m[11]];
    const bc=cross(b,c),ca=cross(c,a),ab=cross(a,b),det=dot(a,bc);
    assert(Math.abs(det)>1e-20);
    return bc.map((_,i) => (-m[12]*bc[i]-m[13]*ca[i]-m[15]*ab[i])/det+s.origin[i]);
}
function judge(s, pixels, bounds) {
    const camera=eye(s),m=s.mvp,[lo,hi]=bounds;
    let visible=0,inked=0;
    const ink = (x,y) => {
        for(let dy=-1;dy<=1;dy++) for(let dx=-1;dx<=1;dx++) {
            const k=((Math.floor(y)+dy)*pixels.width+Math.floor(x)+dx)*4;
            if(pixels.data[k]<190 && pixels.data[k+1]<190 && pixels.data[k+2]<190) return true;
        }
        return false;
    };
    for(let axis=0;axis<3;axis++) for(let side0=0;side0<2;side0++) for(let side1=0;side1<2;side1++) {
        const other=[0,1,2].filter(i=>i!==axis);
        for(let sample=1;sample<4000;sample++) {
            const p=[0,0,0];p[axis]=lo[axis]+(hi[axis]-lo[axis])*sample/4000;
            p[other[0]]=bounds[side0][other[0]];p[other[1]]=bounds[side1][other[1]];
            const v=p.map((x,i)=>x-s.origin[i]);
            const clip=[0,1,2,3].map(r=>m[r]*v[0]+m[r+4]*v[1]+m[r+8]*v[2]+m[r+12]);
            if(clip[3]<=0 || clip[2]>clip[3] || clip[2]<0) continue;
            const x=(clip[0]/clip[3]*.5+.5)*pixels.width,y=(.5-clip[1]/clip[3]*.5)*pixels.height;
            if(x<5 || x>=pixels.width-5 || y<5 || y>=pixels.height-45) continue;
            const d=p.map((x,i)=>x-camera[i]);let enter=-Infinity,exit=Infinity;
            for(let i=0;i<3;i++) {
                const t0=(lo[i]-camera[i])/d[i],t1=(hi[i]-camera[i])/d[i];
                enter=Math.max(enter,Math.min(t0,t1));exit=Math.min(exit,Math.max(t0,t1));
            }
            if(exit>=enter && enter>0 && (1-enter)*Math.hypot(...d)>.05) continue;
            visible++;if(ink(x,y))inked++;
        }
    }
    return {visible,inked,share:inked/visible,eye:camera,samples:s.samples};
}
(async()=>{
    const browser=await chromium.launch({executablePath:'/usr/bin/google-chrome',headless:process.env.HEADLESS==='1',args:['--enable-unsafe-webgpu','--enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE','--ozone-platform=x11']});
    const page=await browser.newPage({viewport:{width:1200,height:800}}),errors=[];
    page.on('pageerror',e=>errors.push(e.message));
    page.on('console',m=>{if(m.type()==='error')errors.push(m.text());});
    try {
        await page.route('**/view_local.yaml',r=>r.fulfill({contentType:'application/yaml',body:'name: Close-up test\nitems: []\n'}));
        await page.route('**/favicon.ico',r=>r.fulfill({status:204}));
        await page.goto(new URL('?data=off&inspect=1&nogrid=1&nomarkers=1',process.env.VIEWER_URL||'http://127.0.0.1:8770/').href);
        await page.waitForFunction(()=>document.querySelector('canvas')?.hasAttribute('data-viewer-inspection'),{},{timeout:90000});
        await command(page,'Box Brep 0,0,0 3000 400 120');
        const bounds=(await state(page)).selected_bounds;assert(bounds);
        async function aim() {
            await command(page,'View Isometric');await command(page,'View Perspective');await command(page,'Fit');
            await page.mouse.click(10,10);await settle(page);
            await page.mouse.move(500,350);await page.mouse.down({button:'right'});
            await page.mouse.move(500+60*Math.PI/180/.005,350-27*Math.PI/180/.005,{steps:12});
            await page.mouse.up({button:'right'});await settle(page);
            const camera=eye(await state(page)),centre=bounds[0].map((x,i)=>(x+bounds[1][i])/2);
            const distance=Math.hypot(...camera.map((x,i)=>x-centre[i]));
            const dy=(bounds[1][2]-centre[2]+20)/(distance*.0015*Math.cos(3*Math.PI/180));
            await page.keyboard.down('Control');await page.mouse.move(500,400);
            await page.mouse.down({button:'right'});await page.mouse.move(500,400+dy,{steps:6});
            await page.mouse.up({button:'right'});await page.keyboard.up('Control');await settle(page);
        }
        if(process.env.ARCTIC==='1') await command(page,'Arctic On');
        await aim();
        const proofs=[];
        for(const opacity of [.95,1]) {
            await command(page,`Opacity ${opacity}`);
            if(opacity===1) await aim();
            for(let zoom=0;zoom<12;zoom++) {
                await page.mouse.move(600,380);await page.mouse.wheel(0,-100);await settle(page);
                const s=await state(page),url=await page.locator('canvas').evaluate(c=>c.toDataURL());
                const pixels=PNG.sync.read(Buffer.from(url.split(',')[1],'base64'));
                const proof={opacity,zoom,...judge(s,pixels,bounds)};proofs.push(proof);
                assert(proof.visible>100,`the view must expose edges: ${JSON.stringify(proof)}`);
                assert(proof.share>=.98,`visible edges have gaps: ${JSON.stringify(proof)}`);
            }
        }
        assert.deepEqual(errors,[]);
        fs.mkdirSync('target/course-checks',{recursive:true});
        const hash = file => createHash('sha256').update(fs.readFileSync(file)).digest('hex');
        const sources = Object.fromEntries(['src/shaders/project_triangles.wgsl','src/shaders/ribbon.wgsl'].map(file => [file,hash(file)]));
        fs.writeFileSync('target/course-checks/close-up-chrome.json',JSON.stringify({browser:browser.version(),
            headless:process.env.HEADLESS==='1',captured:new Date().toISOString(),bounds,sources,
            checker:hash(__filename),bundle:hash('dist/index.html'),proofs},null,2));
        await page.screenshot({path:'target/course-checks/close-up-chrome.png'});
        console.log(`Chrome: ${proofs.length} close-up views retain at least 98% of visible box edge samples`);
    } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exit(1);});
