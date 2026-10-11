// Review revision: imports previous decks and preserves masters, slides, images and native tables.
import fs from 'node:fs/promises';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
const {PresentationFile,FileBlob}=await import(process.env.LUMEN_ARTIFACT_MODULE||'@oai/artifact-tool');
const ROOT=process.cwd(),BUILD=path.join(ROOT,'work/phase19-ppt');
const SKILL=process.env.LUMEN_PRESENTATION_SKILL;
const content=JSON.parse(await fs.readFile(path.join(ROOT,'docs/distribution/PRESENTATION_REVIEW_CONTENT.json'),'utf8'));
const FONT='Microsoft YaHei',CODE='Consolas';
const C={paper:'#F5F4EE',ink:'#203B30',green:'#476751',muted:'#53665A',white:'#F7F7EE',pale:'#BFD0BA',line:'#D4DCD0'};
const owners=[11,12,14,17,19,21,22];
const captions={
3:{'zh-CN':'估时与容量帮助安排今天，真实用时单独记录。',en:'Estimates guide the daily plan. Actual time stays a separate record.'},
4:{'zh-CN':'暂停、锁屏、休眠和离线时间不计入专注。',en:'Paused, locked, sleeping and offline time stays outside focused time.'},
7:{'zh-CN':'汇总有效区间与记录，不用活动量评价一个人。',en:'Summaries describe recorded activity, without grading the person.'},
8:{'zh-CN':'作品默认私密，公开范围由你主动选择。',en:'Works are private by default. You choose what to share.'}};
for(const language of ['zh-CN','en']){
 const input=path.join(ROOT,`docs/presentations/Lumen-Launch-Developer-${language}-v3.pptx`);
 const p=await PresentationFile.importPptx(await FileBlob.load(input));
 const build=path.join(BUILD,language+'-r6');await fs.mkdir(build,{recursive:true});
 await fs.writeFile(path.join(build,'source-inspection.ndjson'),(await p.inspect({kind:'slide,layout,shape,image,table',maxChars:150000})).ndjson);
 function text(s,value,x,y,w,h,size,color,bold=false,face=FONT){
  const sh=s.shapes.add({geometry:'textbox',name:'review-copy',position:{left:x,top:y,width:w,height:h},fill:'none',line:{fill:'none',width:0}});
  sh.text=value;sh.text.insets=0;sh.text.style={typeface:face,fontSize:size,color,bold};return sh;
 }
 for(const [i,data] of content.slides.entries()){
  const s=p.slides.items[i];s.shapes.deleteAll();
  const dark=[0,2,9,11,16,19,23].includes(i),fg=dark?C.white:C.ink,sub=dark?C.pale:C.muted;
  s.background.fill=dark?C.ink:C.paper;
  const body=data.body[language];
  s.speakerNotes.text=`Lumen 0.1.0. Source: ${data.source}.\n`+
   (i===0?'Cover: AI-generated conceptual still life. It does not depict real product hardware.\n':'')+
   (data.kind==='image'?'Product image: isolated synthetic browser fixture feature excerpt, not a real user record or native platform acceptance.\n':'')+
   (i===2||i>=11?'Current official modules are trusted source integrations. Community WASM execution, marketplace and AI calls remain deferred.\n':'')+
   (i===21?'macOS/Linux are experimental. Today/Focus unavailable. Some system integrations unsupported. This edition did not rebuild their packages.\n':'')+
   (i===22?'905 static English frontend strings plus dynamic labels. Some Rust tray, notification and diagnostic copy remains Chinese.\n':'');
  text(s,String(i+1).padStart(2,'0'),1178,686,45,22,16,sub);
  if(i===0){
   const im=s.images.add({blob:new Uint8Array(await fs.readFile(path.join(ROOT,'docs/distribution/presentation-review-assets/cover-concept.png'))),contentType:'image/png',alt:'Conceptual still life of paper, green glass and a fern',position:{left:0,top:0,width:1280,height:720},fit:'cover'});im.sendToBack();
   text(s,'Lumen',64,92,700,150,112,C.white,true);
   text(s,body,68,316,700,172,language==='en'?49:47,C.white,true);
   text(s,'LOCAL-FIRST PERSONAL GROWTH OS',68,625,750,32,20,C.pale);
   continue;
  }
  if(i===23){
   text(s,data.title[language],64,100,1118,130,65,fg,true);
   text(s,body,68,310,1110,135,32,sub);
   text(s,'github.com/wangziwen2794928256/Lumen',68,590,1110,45,27,fg);
   continue;
  }
  if(i===2){
   text(s,'Everything is a Module',64,78,1152,84,53,fg,true);
   text(s,body,64,246,1152,220,language==='en'?65:72,fg,true);
   text(s,data.foot[language],68,609,1118,60,23,sub);continue;
  }
  text(s,data.title[language],64,58,1152,108,46,fg,true);
  if(data.kind==='image'){
   const image=s.images.items[0];image.position={left:48,top:178,width:1184,height:450};image.fit='contain';
   text(s,captions[i][language],64,648,1110,36,23,sub);
  } else if(data.kind==='table'){
   const table=s.tables.items[0];table.position={left:64,top:201,width:1152,height:386};
   table.borders.assign({color:C.line,fill:C.line,width:0.6,style:'solid'});
   for(let r=0;r<data.body[language].length;r++)for(let c=0;c<data.body[language][0].length;c++){
    const cell=table.getCell(r,c);cell.value=data.body[language][r][c];cell.fill=r===0?C.green:(dark?C.ink:C.paper);
    cell.text.style={typeface:FONT,fontSize:24,bold:r===0,color:r===0?C.white:fg};
   }
   const note=data.foot?.[language];
   if(note)text(s,note,64,620,1118,56,22,sub);
  } else if(data.kind==='code'){
   text(s,body,64,217,490,280,29,sub);
   text(s,data.code,602,215,614,340,24,fg,false,CODE);
   text(s,data.foot[language],64,624,1118,56,22,sub);
  } else if(i===1){
   for(const [j,row] of body.entries()){
    const y=213+j*102;text(s,String(j+1).padStart(2,'0'),64,y,60,62,36,C.green);
    text(s,row[0],153,y,357,78,30,fg,true);text(s,row[1],550,y,657,80,28,sub);
   }
  } else if(i===5||i===12||i===17){
   for(const [j,row] of body.entries()){
    const x=j%2===0?64:672,y=j<2?217:430;
    text(s,row[0],x,y,540,58,31,dark?C.pale:C.green,true);
    text(s,row[1],x,y+71,540,117,28,sub);
   }
  } else if(i===6){
   text(s,body,64,232,705,220,58,fg,true);
   text(s,language==='en'?'Markdown journals\nDraft recovery\nTags, links and search':'Markdown 日记\n草稿恢复\n标签、关联与搜索',825,242,385,180,28,sub);
   text(s,data.foot[language],64,613,1118,60,23,sub);
  } else if(i===9){
   text(s,body,64,235,1152,210,language==='en'?53:64,fg,true);
   text(s,data.foot[language],64,611,1118,60,23,sub);
  } else if(i===19){
   text(s,'MIT',64,231,450,180,136,fg,true);
   text(s,language==='en'?'Fork into your own work\nAdapt the components\nContribute improvements':'可以 Fork 成自己的作品\n可以改造需要的组件\n也欢迎把改进贡献回来',620,246,590,220,36,sub);
   text(s,data.foot[language],64,609,1118,60,23,sub);
  } else if(i===22){
   const columns=language==='en'?[['60','IPC commands'],['12','Feature groups'],['905','English UI strings']]:[['60','IPC 命令'],['12','功能分组'],['905','英文界面资源']];
   for(const [j,row] of columns.entries()){const x=64+j*400;text(s,row[0],x,237,360,130,86,C.green,true);text(s,row[1],x,382,360,65,27,fg,true);}
   text(s,language==='en'?'Module reference, compiled tutorial and configuration guides are available in Chinese and English.':'逐模块参考、可编译教程和配置指南，均提供中英文版本。',64,541,1118,80,29,sub);
   text(s,language==='en'?'Some native tray and notification copy remains Chinese.':'部分原生托盘与通知文案仍为中文。',64,638,1118,34,22,sub);
  }
 }
 const candidate=path.join(build,'candidate.pptx');await(await PresentationFile.exportPptx(p)).save(candidate);
 await fs.writeFile(path.join(build,'deck-layout.json'),JSON.stringify(p.toProto()));
 if(SKILL){
  const {finalizePresentation}=await import(pathToFileURL(path.join(SKILL,'container_tools/artifact_tool_utils.mjs')).href);
  const result=await finalizePresentation({workspaceDir:ROOT,candidatePath:candidate,finalPath:path.join(ROOT,`docs/presentations/Lumen-Review-${language}-r6.pptx`),pythonExecutable:process.env.LUMEN_PRESENTATION_PYTHON,
   integrityValidatorPath:path.join(SKILL,'container_tools/inspect_presentation_package_integrity.py'),layoutValidatorPath:path.join(SKILL,'container_tools/inspect_presentation_layout_geometry.py'),
   layoutArgs:['--expected-slide-size-emu','12192000,6858000','--validate-bullet-geometry','--validate-heading-fit',...owners.flatMap(n=>['--require-native-table-slide',String(n)])],
   requiredNativeTableOwnerSlides:owners,explicitTotalSlideCount:24,fontPolicy:{basis:'design',families:[FONT,CODE]},verifyArtifactToolImport:true,receiptPath:path.join(build,'validation.json')});
  console.log(JSON.stringify({language,finalPath:result.finalPath,sha256:result.finalSha256,slides:24}));
 }
}
