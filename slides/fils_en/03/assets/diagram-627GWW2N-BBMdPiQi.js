import{p as w}from"./chunk-K2ZEYYM2-CzPsp8WF.js";import{p as B}from"./gitGraph-YCYPL57B-5MQDGNWY-BUAha0vS.js";import{as as S,s as z,g as F,t as P,u as W,d as T,e as D,a as l,m as x,at as v,au as A,z as E,aU as _,n as N}from"./Mermaid.vue_vue_type_script_setup_true_lang-Dorx8uYM.js";import"./chunk-TGZYFRKZ-H3DmNiEl.js";import"./index-C2wSiMOw.js";import"./modules/vue-DZlg2sXr.js";import"./modules/shiki-Dn4Kph4M.js";import"./modules/file-saver-C8BEGaqa.js";var C={packet:[]},m=structuredClone(C),L=S.packet,Y=l(()=>{const t=v({...L,...A().packet});return t.showBits&&(t.paddingY+=10),t},"getConfig"),I=l(()=>m.packet,"getPacket"),M=l(t=>{t.length>0&&m.packet.push(t)},"pushWord"),O=l(()=>{E(),m=structuredClone(C)},"clear"),h={pushWord:M,getPacket:I,getConfig:Y,clear:O,setAccTitle:z,getAccTitle:F,setDiagramTitle:P,getDiagramTitle:W,getAccDescription:T,setAccDescription:D},U=1e4,G=l(t=>{w(t,h);let e=-1,o=[],s=1;const{bitsPerRow:i}=h.getConfig();for(let{start:a,end:r,label:p}of t.blocks){if(r&&r<a)throw new Error(`Packet block ${a} - ${r} is invalid. End must be greater than start.`);if(a!==e+1)throw new Error(`Packet block ${a} - ${r??a} is not contiguous. It should start from ${e+1}.`);for(e=r??a,x.debug(`Packet block ${a} - ${e} with label ${p}`);o.length<=i+1&&h.getPacket().length<U;){const[u,c]=H({start:a,end:r,label:p},s,i);if(o.push(u),u.end+1===s*i&&(h.pushWord(o),o=[],s++),!c)break;({start:a,end:r,label:p}=c)}}h.pushWord(o)},"populate"),H=l((t,e,o)=>{if(t.end===void 0&&(t.end=t.start),t.start>t.end)throw new Error(`Block start ${t.start} is greater than block end ${t.end}.`);return t.end+1<=e*o?[t,void 0]:[{start:t.start,end:e*o-1,label:t.label},{start:e*o,end:t.end,label:t.label}]},"getNextFittingBlock"),K={parse:l(async t=>{const e=await B("packet",t);x.debug(e),G(e)},"parse")},R=l((t,e,o,s)=>{const i=s.db,a=i.getConfig(),{rowHeight:r,paddingY:p,bitWidth:u,bitsPerRow:c}=a,b=i.getPacket(),n=i.getDiagramTitle(),g=r+p,d=g*(b.length+1)-(n?0:r),k=u*c+2,f=_(e);f.attr("viewbox",`0 0 ${k} ${d}`),N(f,d,k,a.useMaxWidth);for(const[$,y]of b.entries())X(f,y,$,a);f.append("text").text(n).attr("x",k/2).attr("y",d-g/2).attr("dominant-baseline","middle").attr("text-anchor","middle").attr("class","packetTitle")},"draw"),X=l((t,e,o,{rowHeight:s,paddingX:i,paddingY:a,bitWidth:r,bitsPerRow:p,showBits:u})=>{const c=t.append("g"),b=o*(s+a)+a;for(const n of e){const g=n.start%p*r+1,d=(n.end-n.start+1)*r-i;if(c.append("rect").attr("x",g).attr("y",b).attr("width",d).attr("height",s).attr("class","packetBlock"),c.append("text").attr("x",g+d/2).attr("y",b+s/2).attr("class","packetLabel").attr("dominant-baseline","middle").attr("text-anchor","middle").text(n.label),!u)continue;const k=n.end===n.start,f=b-2;c.append("text").attr("x",g+(k?d/2:0)).attr("y",f).attr("class","packetByte start").attr("dominant-baseline","auto").attr("text-anchor",k?"middle":"start").text(n.start),k||c.append("text").attr("x",g+d).attr("y",f).attr("class","packetByte end").attr("dominant-baseline","auto").attr("text-anchor","end").text(n.end)}},"drawWord"),j={draw:R},q={byteFontSize:"10px",startByteColor:"black",endByteColor:"black",labelColor:"black",labelFontSize:"12px",titleColor:"black",titleFontSize:"14px",blockStrokeColor:"black",blockStrokeWidth:"1",blockFillColor:"#efefef"},J=l(({packet:t}={})=>{const e=v(q,t);return`
	.packetByte {
		font-size: ${e.byteFontSize};
	}
	.packetByte.start {
		fill: ${e.startByteColor};
	}
	.packetByte.end {
		fill: ${e.endByteColor};
	}
	.packetLabel {
		fill: ${e.labelColor};
		font-size: ${e.labelFontSize};
	}
	.packetTitle {
		fill: ${e.titleColor};
		font-size: ${e.titleFontSize};
	}
	.packetBlock {
		stroke: ${e.blockStrokeColor};
		stroke-width: ${e.blockStrokeWidth};
		fill: ${e.blockFillColor};
	}
	`},"styles"),nt={parser:K,db:h,renderer:j,styles:J};export{nt as diagram};
