import{d as _,u as d,a as u,c as m,b as h,r as p,o as a,e as n,f as t,t as s,g as r,F as f,h as g,n as v,i as b,j as x,k as y,l as k,m as N,_ as w}from"./index-D_zQPKcw.js";import{N as P}from"./NoteDisplay-BXMiSKyP.js";const V={class:"m-4"},L={class:"mb-10"},S={class:"text-4xl font-bold mt-2"},T={class:"opacity-50"},B={class:"text-lg"},D={class:"font-bold flex gap-2"},H={class:"opacity-50"},j=t("div",{class:"flex-auto"},null,-1),z={key:0,class:"border-gray-400/50 mb-8"},C=_({__name:"PresenterPrint",setup(F){d(`
@page {
  size: A4;
  margin-top: 1.5cm;
  margin-bottom: 1cm;
}
* {
  -webkit-print-color-adjust: exact;
}
html,
html body,
html #app,
html #page-root {
  height: auto;
  overflow: auto !important;
}
`),u({title:`Notes - ${m.title}`});const i=h(()=>p.map(o=>{var l;return(l=o.meta)==null?void 0:l.slide}).filter(o=>o!==void 0&&o.noteHTML!==""));return(o,l)=>(a(),n("div",{id:"page-root",style:v(r(b))},[t("div",V,[t("div",L,[t("h1",S,s(r(m).title),1),t("div",T,s(new Date().toLocaleString()),1)]),(a(!0),n(f,null,g(i.value,(e,c)=>(a(),n("div",{key:c,class:"flex flex-col gap-4 break-inside-avoid-page"},[t("div",null,[t("h2",B,[t("div",D,[t("div",H,s(e==null?void 0:e.no)+"/"+s(r(x)),1),y(" "+s(e==null?void 0:e.title)+" ",1),j])]),k(P,{"note-html":e.noteHTML,class:"max-w-full"},null,8,["note-html"])]),c<i.value.length-1?(a(),n("hr",z)):N("v-if",!0)]))),128))])],4))}}),E=w(C,[["__file","/home/runner/work/upb-fils-ma.github.io/upb-fils-ma.github.io/slides/node_modules/@slidev/client/internals/PresenterPrint.vue"]]);export{E as default};
