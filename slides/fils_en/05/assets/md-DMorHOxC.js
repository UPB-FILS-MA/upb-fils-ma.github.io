import{_ as p}from"./slidev/CodeBlockWrapper.vue_vue_type_script_setup_true_lang-BndULcEq.js";import{o as r,c as u,k as a,e as s,B as n,l as c,m as d,q as m,s as f,C as e}from"./modules/vue-CfOVo5sz.js";import{I as _}from"./slidev/default-C-Uktp1O.js";import{u as k,f as h}from"./slidev/context-CS2xqH0l.js";import"./modules/unplugin-icons-vnfR6w8e.js";import"./index-C4d0Pr1U.js";import"./modules/shiki-BCAiZ4dZ.js";const g={grid:"~ cols-2 gap5"},j={__name:"slides.md__slidev_10",setup(x){const{$slidev:v,$nav:w,$clicksContext:t,$clicks:b,$page:y,$renderContext:$,$frontmatter:i}=k();return t.setup(),(C,l)=>{const o=p;return r(),u(_,m(f(e(h)(e(i),9))),{default:a(()=>[l[2]||(l[2]=s("h1",null,"Tasks can stop the executor",-1)),s("div",g,[l[1]||(l[1]=s("div",null,[s("ul",null,[s("li",null,[n("unless awaited, "),s("code",null,"async"),n(" functions are not executed")]),s("li",null,[n("tasks have to use "),s("code",null,".await"),n(" in loops, otherwise they block the scheduler")])])],-1)),c(o,d({},{ranges:["all","5-8","3-9"]}),{default:a(()=>l[0]||(l[0]=[s("pre",{class:"shiki shiki-themes vitesse-dark vitesse-light slidev-code",style:{"--shiki-dark":"#dbd7caee","--shiki-light":"#393a34","--shiki-dark-bg":"#121212","--shiki-light-bg":"#ffffff"}},[s("code",{class:"language-text"},[s("span",{class:"line"},[s("span",null,"#[embassy_executor::task]")]),n(`
`),s("span",{class:"line"},[s("span",null,"async fn led_blink(mut led:Output<'static, PIN_X>) {")]),n(`
`),s("span",{class:"line"},[s("span",null,"    loop {")]),n(`
`),s("span",{class:"line"},[s("span",null,"        led.toogle();")]),n(`
`),s("span",{class:"line"},[s("span",null,"        // this does not execute anything")]),n(`
`),s("span",{class:"line"},[s("span",null,"        Timer::after_secs(1);")]),n(`
`),s("span",{class:"line"},[s("span",null,"        // infinite loop without `.await`")]),n(`
`),s("span",{class:"line"},[s("span",null,"        // that never gives up the MCU")]),n(`
`),s("span",{class:"line"},[s("span",null,"    }")]),n(`
`),s("span",{class:"line"},[s("span",null,"}")]),n(`
`),s("span",{class:"line"},[s("span")]),n(`
`),s("span",{class:"line"},[s("span",null,"#[embassy_executor::main]")]),n(`
`),s("span",{class:"line"},[s("span",null,"async fn main(spawner: Spawner) {")]),n(`
`),s("span",{class:"line"},[s("span",null,"    // ..")]),n(`
`),s("span",{class:"line"},[s("span",null,"    loop {")]),n(`
`),s("span",{class:"line"},[s("span",null,"        button.wait_for_rising_edge().await;")]),n(`
`),s("span",{class:"line"},[s("span",null,'        info!("button pressed");')]),n(`
`),s("span",{class:"line"},[s("span",null,"    }")]),n(`
`),s("span",{class:"line"},[s("span",null,"}")])])],-1)])),_:1},16)])]),_:1},16)}}};export{j as default};
