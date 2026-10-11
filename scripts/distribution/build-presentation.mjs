// Editable release deck from bilingual content and synthetic screenshots.
// Use an environment-provided @oai/artifact-tool module. Never reads app data.
import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
const { Presentation, PresentationFile } = await import(process.env.LUMEN_ARTIFACT_MODULE || '@oai/artifact-tool');
const ROOT = process.cwd(), TMP = path.join(ROOT, 'work/phase18-ppt');
const content = JSON.parse(await fs.readFile(path.join(ROOT, 'docs/distribution/PRESENTATION_CONTENT.json'), 'utf8'));
const FONT = 'Microsoft YaHei', CODE = 'Consolas';
const colors = { paper: '#F5F4EE', ink: '#203B30', green: '#476751', muted: '#53665A', white: '#F7F7EE', pale: '#BFD0BA', line: '#D4DCD0' };
for (const language of ['zh-CN', 'en']) {
 const p = Presentation.create({ slideSize: { width: 1280, height: 720 } });
 const tables = [];
 function text(s, value, x, y, w, h, size, color, bold = false, face = FONT) {
  const shape = s.shapes.add({ geometry: 'textbox', position: { left:x, top:y, width:w, height:h }, fill:'none', line:{fill:'none',width:0} });
  shape.text = value;
  shape.text.style = { typeface:face, fontSize:size, color, bold };
  return shape;
 }
 for (const [index, data] of content.slides.entries()) {
  const s=p.slides.add(), dark=data.dark || false;
  const foreground=dark?colors.white:colors.ink, secondary=dark?colors.pale:colors.muted;
  s.background.fill=dark?colors.ink:colors.paper;
  s.speakerNotes.text='Lumen 0.1.0 candidate. '+data.source+'\nDesign reference: https://developer.apple.com/videos/play/wwdc2025/102/ and https://code.visualstudio.com/api/get-started/extension-anatomy . Structure adapted without brand assets or copied wording. Screenshots are synthetic browser fixtures, not native acceptance. '+(language==='en'?'English frontend copy excludes user text; Rust tray/notification copy remains partly Chinese.':'前端英化不改用户原文；Rust 托盘/通知仍有中文。');
  text(s,String(index+1).padStart(2,'0'),1170,674,60,26,17,secondary);
  if(data.kind==='cover') {
   text(s,'Lumen',68,100,1100,150,128,foreground,true);
   text(s,data.body[language],72,340,1110,180,55,foreground,true);
   text(s,'LOCAL-FIRST PERSONAL GROWTH OS',74,625,1050,35,21,secondary);
   continue;
  }
  if(data.kind==='closing') {
   text(s,data.title[language],68,118,1140,175,61,foreground,true);
   text(s,data.body[language],72,355,1110,150,32,secondary);
   text(s,'github.com/wangziwen2794928256/Lumen',72,600,1100,48,27,foreground);
   continue;
  }
  text(s,data.title[language],64,66,1152,124,46,foreground,true);
  const body=data.body[language];
  if(data.kind==='statement') {
   text(s,body,64,275,1152,235,47,foreground,true);
  } else if(data.kind==='rows') {
   for(const [i,row] of body.entries()) {
    const y=217+i*98;
    text(s,row[0],64,y,360,80,30,dark?colors.pale:colors.green,true);
    text(s,row[1],440,y,776,82,29,secondary);
   }
  } else if(data.kind==='image') {
   const imagePath=path.join(ROOT,`docs/distribution/presentation-assets/${data.asset}-${language}.png`);
   s.images.add({blob:new Uint8Array(await fs.readFile(imagePath)),contentType:'image/png',alt:data.title[language]+' (synthetic data)',fit:'contain',position:{left:64,top:183,width:1152,height:414}});
   text(s,body,64,607,1120,58,24,secondary);
  } else if(data.kind==='table') {
   tables.push(index+1);
   const table=s.tables.add({rows:body.length,columns:body[0].length,left:64,top:205,width:1152,height:390,columnWidths:data.widths,values:body});
   table.borders.assign({fill:dark?'#526A5A':colors.line,width:1,style:'solid'});
   for(let r=0;r<body.length;r++)for(let c=0;c<body[0].length;c++){
    const cell=table.getCell(r,c);cell.fill=r===0?colors.green:(dark?colors.ink:colors.paper);
    cell.text.style={typeface:FONT,fontSize:24,color:r===0?colors.white:foreground,bold:r===0};
   }
  } else if(data.kind==='code') {
   text(s,body,64,223,520,235,30,secondary);
   text(s,data.code,642,220,574,355,24,foreground,false,CODE);
  }
  if(data.foot) text(s,data.foot[language],64,622,1120,44,23,secondary);
 }
 const build=path.join(TMP,language+'-v3');await fs.mkdir(build,{recursive:true});
 const candidate=path.join(build,'candidate.pptx');
 await(await PresentationFile.exportPptx(p)).save(candidate);
 await fs.writeFile(path.join(build,'deck-layout.json'),JSON.stringify(p.toProto()));
 if(process.env.LUMEN_PRESENTATION_SKILL) {
  const skill=process.env.LUMEN_PRESENTATION_SKILL;
  const {finalizePresentation}=await import(pathToFileURL(path.join(skill,'container_tools/artifact_tool_utils.mjs')).href);
  const finalPath=path.join(ROOT,`docs/presentations/Lumen-Launch-Developer-${language}-v3.pptx`);
  const result=await finalizePresentation({workspaceDir:ROOT,candidatePath:candidate,finalPath,pythonExecutable:process.env.LUMEN_PRESENTATION_PYTHON,
    integrityValidatorPath:path.join(skill,'container_tools/inspect_presentation_package_integrity.py'),layoutValidatorPath:path.join(skill,'container_tools/inspect_presentation_layout_geometry.py'),
    layoutArgs:['--expected-slide-size-emu','12192000,6858000','--validate-bullet-geometry','--validate-heading-fit',...tables.flatMap(n=>['--require-native-table-slide',String(n)])],
    requiredNativeTableOwnerSlides:tables,explicitTotalSlideCount:24,fontPolicy:{basis:'design',families:[FONT,CODE]},verifyArtifactToolImport:true,receiptPath:path.join(build,'validation.json')});
  console.log(JSON.stringify(result));
 }
 console.log(JSON.stringify({language,slides:p.slides.items.length,candidate}));
}

