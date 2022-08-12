import{resize_square,stroke_circle,stroke_circle_2,fill_circle_2,fill_circle,clear,line,fill_text}from"../canvas";import{colors}from"../colors";import{collision_response,distance_sqrd,wrap_around,normalize,delta,rotate,add,del,mul,mod}from"../math";import{get_fps,update_fps,get_ups,update_ups,get_ups_avg_delta}from"../perf";import{ship_0,ship_2,default_ship_journey,emerald}from"../ship";const LINK_STRENGH=.2,GRID_SIDE=20,CELL_COUNT=400,DIAM=.0125,LOADING_AWAIT=1;let score_to_win,start_time,scores=[0,0],CONTINUE_RENDER=!0;const msToTime=e=>{function n(e,n){return("00"+e).slice(-(n=n||2))}var r=e%1e3,a=(e=(e-r)/1e3)%60,i=(e=(e-a)/60)%60;return n((e-i)/60)+":"+n(i)+":"+n(a)+"."+n(r,3)},get_best_by_player=e=>{const n=`best_${parseInt(window.location.pathname.split("journey-")[1])}_${e}`;let r=localStorage.getItem(n);return r&&r.length&&(r=JSON.parse(r)),r&&r.id||(r={}),r.id=n,r},get_best=()=>{const e=`best_${parseInt(window.location.pathname.split("journey-")[1])}`;let n=localStorage.getItem(e);return n&&n.length&&(n=JSON.parse(n)),n&&n.id||(n={}),n.id=e,n},update_best=(e,n)=>{const r=get_best();r.duration&&r.duration<=e||(r.duration=e,r.player_name=n,localStorage.setItem(r.id,JSON.stringify(r)));const a=get_best_by_player(n);a.duration&&a.duration<=e||(a.duration=e,a.player_name=n,localStorage.setItem(a.id,JSON.stringify(a)))},player_name=()=>localStorage.getItem("player_journey_name")?localStorage.getItem("player_journey_name"):"Blue",update_player_info=()=>{localStorage.setItem("player_journey_name",document.querySelector("#player_journey_name").value);const e=get_best_by_player(player_name()),n=e.duration?msToTime(e.duration):"--:--:--:---";document.querySelector("#best_duration_current_player").innerHTML=n},html=()=>{const e=get_best(),n=e.duration?msToTime(e.duration):"--:--:--:---",r=e.player_name?e.player_name:"",a=get_best_by_player(player_name()),i=a.duration?msToTime(a.duration):"--:--:--:---";return`\n    <div id="winner" class="hide">\n      <p><span id="duration"></span></p>\n      <div>\n        <button onclick="again()">Play Again<br>[space]</button>\n        <button onclick="next()">Next<br>[enter]</button>\n      </div>\n    </div>\n    <div class="bob">\n      <input class="player_name"  id="player_journey_name"\n        value="${player_name()}"\n        oninput="update_player_info()"></input>\n      <p class="disappearable disappear" id="best_duration_current_player">${i}</p>\n      <p id="move_with_instructions" class="disappearable disappear">Loading...</p>\n      <p class="disappearable disappear"> <a href="/journey">Levels</a> </p>\n      <p class="disappearable disappear"> <a href="/journey-garage">Garage</a> </p>\n    </div>\n    <canvas id="canvas"></canvas>\n    <div class="bob">\n      <p class="disappearable disappear" id="best_name">${r}</p>\n      <p class="disappearable disappear" id="best_duration">${n}</p>\n      <p class="disappearable disappear">FPS: <span id="fps">...</span></p>\n      <p class="disappearable disappear">UPS: <span id="ups">...</span></p>\n      <p class="disappearable disappear"> <a href="/">Home</a> </p>\n    </div>\n  `},style=()=>"\n    * {\n      color: #ffa;\n      background: #113;\n      font-size: 1.05rem;\n    }\n    select {\n      border: none;\n    }\n    option {\n    }\n    #content {\n      display: flex;\n      width: 100%;\n      height: 100%;\n      align-content: center;\n      align-items: center;\n      flex-direction: row;\n    }\n    a {\n      color: #ffa;\n      text-decoration: none;\n      background-color: #fff0;\n      padding: 0.8rem;\n    }\n    .disappear, .disappear * {\n      color: #0000;\n      transition: color 0.2s;\n    }\n    a:hover {\n      background-color: #fff2;\n    }\n    #score_player_1, #score_player_2 {\n      font-size: 2rem;\n    }\n    #content > div.bob {\n      width: 0;\n      flex-grow: 1;\n      display: flex;\n      justify-content: space-around;\n      flex-direction: column;\n      height: 100%;\n    }\n    p {\n      text-align: center;\n      color: #ffa;\n      font-family: monospace;\n    }\n    p span {\n      color: #ffa;\n    }\n    #canvas {\n        background: #113;\n        display:flex;\n        position: unset;\n    }\n    body {\n      background: #113;\n    }\n    #winner {\n      position: absolute;\n      height: 100vh;\n      width: 100vw;\n      background: #0000;\n      display: flex;\n      flex-direction: column;\n      align-content: center;\n      align-items: center;\n      justify-content: center;\n      pointer-events: none;\n    }\n    #winner > p {\n      background: #ffaa;\n      padding: 5rem;\n      border-radius: 10rem;\n      border: solid 5px #ffa;\n    }\n    #winner > p > span, #winner > p  {\n      color: #ffa;\n      font-size: 3rem;\n    }\n    #winner > p > span {\n      background: transparent;\n    }\n    #winner button {\n      border: none;\n      margin: 2rem;\n      cursor: pointer;\n      pointer-events: all;\n      padding: 1rem;\n      border-radius: 10rem;\n      background: #fff0;\n      line-height: 1.5rem;\n    }\n    #winner div {\n      background: #fff0;\n    }\n    #winner button:hover {\n      background: #fff2;\n    }\n    .hide {\n      display: none !important;\n    }\n    a {\n      border-radius: 10rem;\n    }\n    .player_name {\n      text-align: center;\n      background: none;\n      border: solid 2px transparent;\n      margin-left: 1rem;\n      margin-right: 1rem;\n      padding: 0.5rem;\n      font-size: 1.5rem;\n    }\n    .player_name:hover {\n      border: solid 2px #ffdd;\n    }\n    #best_name {\n      margin-left: 1rem;\n      margin-right: 1rem;\n      padding: 0.5rem;\n      font-size: 1.5rem;\n      border: solid 2px #0000;\n    }\n  ",grid_id=e=>parseInt(20*e.y)%20*20+parseInt(20*e.x)%20,grid_id_3=(e,n)=>n%20*20+e%20,grid_ids=[],grid=[];for(var x=0;x<20;x++)for(var y=0;y<20;y++){const e=[(x-1+20)%20,(x+20)%20,(x+1+20)%20],n=[(y-1+20)%20,(y+20)%20,(y+1+20)%20],r=grid_id_3(x,y);grid_ids.push([]),grid_ids[r]=[grid_id_3(e[0],n[0]),grid_id_3(e[0],n[1]),grid_id_3(e[0],n[2]),grid_id_3(e[1],n[0]),grid_id_3(e[1],n[1]),grid_id_3(e[1],n[2]),grid_id_3(e[2],n[0]),grid_id_3(e[2],n[1]),grid_id_3(e[2],n[2])],grid.push(new Set)}let winner,parts=[],parts_deleted=new Set,links=[],links_set=new Set,key_bindings=new Map,emeralds=[],key_allowed=!1;const sleep=e=>new Promise(e?n=>setTimeout(n,e):e=>e()),add_part=(e,n,r,a,i)=>{const t=parts.length;return parts.push({idx:t,kind:i,d:DIAM,dp:{x:r,y:a},pp:{x:e-r,y:n-a},p:{x:e,y:n},np:{x:e,y:n},collision_response:{x:0,y:0,count:0},link_response:{x:0,y:0},links:new Set,direction:{x:0,y:0}}),t},add_link=(e,n,r)=>{const a=e<n?`${e}|${n}`:`${n}|${e}`;links_set.has(a)&&!r||(links.push({a:e,b:n}),links_set.add(a),parts[e].links.add(n),parts[n].links.add(e))},add_player_ship=async(e,n,r)=>{const a=parts.length;for(let a of e.parts){const i=add_part((a.p.x-e.center.x)/e.DIAM*DIAM+n,(a.p.y-e.center.y)/e.DIAM*DIAM+r,0,0,a.kind);parts[i].player_id=a.player_id,a.binding&&(key_bindings.has(a.binding)||key_bindings.set(a.binding,new Set),key_bindings.get(a.binding).add(i)),await sleep(1)}for(let n of e.links)add_link(n.a+a,n.b+a),await sleep(1)},add_ship=async(e,n,r)=>{const a=parts.length,i=parts.length+1;add_part(n-.00625,r,0,0,e.p1),await sleep(1),add_part(n+.00625,r,0,0,e.p2),await sleep(1);for(let n of e.parts){const e=parts[a+n[0]],r=parts[a+n[1]],i=rotate(e.p,r.p,1/6),t=add_part(i.x,i.y,0,0,n[2]);add_link(t,e.idx),await sleep(1),add_link(t,r.idx),await sleep(1)}add_link(a,i),await sleep(1);for(let n of e.links)add_link(n[0]+a,n[1]+a),await sleep(1);for(let n of Object.keys(e.key_bindings)){key_bindings.has(n)||key_bindings.set(n,new Set);for(let r of e.key_bindings[n])key_bindings.get(n).add(r+a)}},average_color=(e,n)=>(e={r:parseInt(e[1],16),g:parseInt(e[2],16),b:parseInt(e[3],16)},n={r:parseInt(n[1],16),g:parseInt(n[2],16),b:parseInt(n[3],16)},`rgb(${.5*(e.r+n.r)*17},${.5*(e.g+n.g)*17},${.5*(e.b+n.b)*17})`),render=e=>{update_fps(),clear(e);for(let n of parts)n.deleted||(n.activated&&"booster"==n.kind?(fill_circle_2(e,add(n.p,mul(n.direction,.007+.003*Math.random())),.7*n.d,colors[n.kind].value_3),fill_circle_2(e,add(n.p,mul(n.direction,.005+.001*Math.random())),.9*n.d,colors[n.kind].value_2),fill_circle_2(e,n.p,n.d,colors[n.kind].value_1)):"booster"==n.kind?fill_circle_2(e,n.p,n.d,colors[n.kind].value):fill_circle_2(e,n.p,n.d,colors[n.kind].value[n.player_id]));for(let n of Object.keys(colors))for(let r of links){const a=parts[r.a],i=parts[r.b];if(a.deleted||i.deleted||r.deleted)continue;const t=wrap_around(a.np,i.np),s=mul(delta(t.a,t.b),.5),d=colors[a.kind].score>colors[i.kind].score?a.kind:i.kind;if(n==d){const n=colors[d].value[a.player_id],r=.75;fill_circle_2(e,add(a.p,s),a.d*r,n),fill_circle_2(e,del(i.p,s),i.d*r,n)}}document.getElementById("fps").innerHTML=get_fps(),document.getElementById("ups").innerHTML=get_ups(),CONTINUE_RENDER?window.requestAnimationFrame((()=>{render(e)})):again_2()},update_grid=()=>{for(var e=0;e<400;e++)grid[e].clear();for(let e of parts){const n=grid_id(e.p);grid[n].add(e.idx),e.grid_id=n}},neighbours=e=>{const n=grid_id(e);return new Set([...grid[grid_ids[n][0]],...grid[grid_ids[n][1]],...grid[grid_ids[n][2]],...grid[grid_ids[n][3]],...grid[grid_ids[n][4]],...grid[grid_ids[n][5]],...grid[grid_ids[n][6]],...grid[grid_ids[n][7]],...grid[grid_ids[n][8]]])},compute=()=>{update_grid();let e=0;for(let n of parts)if(!n.deleted){n.direction={x:0,y:0};for(let e of n.links){const r=parts[e],a=wrap_around(n.p,r.p);n.direction=add(n.direction,delta(a.b,a.a))}n.direction=normalize(n.direction),n.dp.x=n.p.x-n.pp.x,n.dp.y=n.p.y-n.pp.y,"booster"==n.kind&&n.activated&&(n.dp.x-=1e-4*n.direction.x,n.dp.y-=1e-4*n.direction.y),n.np.x=n.p.x+n.dp.x,n.np.y=n.p.y+n.dp.y,n.link_response.x=0,n.link_response.y=0,n.collision_response.x=0,n.collision_response.y=0,n.collision_response.count=0,e+=distance_sqrd(n.dp)}for(let e of parts)if(!e.deleted)for(let n of neighbours(e.p)){const r=parts[n];if(!r.deleted&&e.idx<r.idx){const n=wrap_around(e.np,r.np);n.a.np={x:n.a.x,y:n.a.y},n.b.np={x:n.b.x,y:n.b.y},n.a.dp=e.dp,n.b.dp=r.dp;const a=n.d_sqrd,i=.5*(e.d+r.d);if(a<i*i){let a=null,i=null;void 0!==e.player_id&&"emerald"==r.kind?(a=r.idx,i=e.player_id):void 0!==r.player_id&&"emerald"==e.kind&&(a=e.idx,i=r.player_id),a&&(parts[a].deleted=!0,parts_deleted.add(a),scores[i]+=1);let t=collision_response(n.a,n.b);links_set.has(`${e.idx}|${r.idx}`)&&(t.x*=.5,t.y*=.5),e.collision_response.x-=t.x,e.collision_response.y-=t.y,e.collision_response.count+=1,r.collision_response.x+=t.x,r.collision_response.y+=t.y,r.collision_response.count+=1}}}for(let e of links){const n=parts[e.a],r=parts[e.b];if(n.deleted&&r.deleted&&(e.deleted=!0),n.deleted||r.deleted||e.deleted)continue;const a=wrap_around(n.np,r.np),i=Math.sqrt(a.d_sqrd),t=normalize(delta(a.a,a.b),i),s=.2*(.5*(n.d+r.d)-i);n.link_response.x-=t.x*s*.5,n.link_response.y-=t.y*s*.5,r.link_response.x+=t.x*s*.5,r.link_response.y+=t.y*s*.5}for(let e of parts)e.deleted||(e.collision_response.count&&(e.collision_response.x/=e.collision_response.count,e.collision_response.y/=e.collision_response.count,e.np.x+=e.collision_response.x,e.np.y+=e.collision_response.y,e.np.x+=e.link_response.x,e.np.y+=e.link_response.y),e.p.x=(e.np.x+1)%1,e.p.y=(e.np.y+1)%1,e.pp.x=e.p.x-e.dp.x-e.collision_response.x-e.link_response.x,e.pp.y=e.p.y-e.dp.y-e.collision_response.y-e.link_response.y);update_ups(),winning_condition(),window.setTimeout((()=>{compute()}),10-get_ups_avg_delta())},winning_condition=()=>{if(null!=winner)return;let e;for(var n=0;n<scores.length;n++)if(scores[n]>=score_to_win){winner=n,e=performance.now()-start_time;break}null!=winner&&(localStorage.setItem("progress",parseInt(Math.max(parseInt(window.location.pathname.split("journey-")[1]),parseInt(localStorage.getItem("progress"))))),update_best(e,player_name()),document.querySelector("#duration").innerHTML=`${msToTime(e)}`,document.querySelector("#winner").classList.remove("hide"))},is_in_emerald=e=>{for(var n of emeralds)for(var r of n)if(e==r)return!0},get_free_idx=()=>{if(parts_deleted.size){const e=parts_deleted.keys().next().value;if(!is_in_emerald(e))return parts_deleted.delete(e),e}const e=parts.length;return parts.push({}),e},new_emerald=(e,n)=>{for(var r of(e||(e=.8*Math.random()+.1),n||(n=.8*Math.random()+.1),parts))if(wrap_around(r.p,{x:e,y:n}).d_sqrd<DIAM*DIAM*4*4)return new_emerald();const a=new Set;for(var i=0;i<4;i++)a.add(get_free_idx());return add_ship_2(emerald,e,n,[...a]),a},add_ship_2=(e,n,r,a)=>{const i=a[0],t=a[1];set_part(n-.00625,r,0,0,e.p1,a[0]),set_part(n+.00625,r,0,0,e.p2,a[1]);for(var s=0;s<e.parts.length;s++){const n=e.parts[s],r=parts[a[n[0]]],i=parts[a[n[1]]],t=rotate(r.p,i.p,1/6),d=a[s+2];set_part(t.x,t.y,0,0,n[2],d),add_link(d,r.idx,!0),add_link(d,i.idx,!0)}add_link(i,t,!0);for(let n of e.links)add_link(a[n[0]],a[n[1]],!0);for(let n of Object.keys(e.key_bindings)){key_bindings.has(n)||key_bindings.set(n,new Set);for(let r of e.key_bindings[n])key_bindings.get(n).add(a[r])}},set_part=(e,n,r,a,i,t)=>(parts[t]={idx:t,kind:i,d:DIAM,dp:{x:r,y:a},pp:{x:e-r,y:n-a},p:{x:e,y:n},np:{x:e,y:n},collision_response:{x:0,y:0,count:0},link_response:{x:0,y:0},links:new Set,direction:{x:0,y:0}},t),again=()=>{CONTINUE_RENDER=!1},next=()=>{const e=window.location.pathname.split("journey-")[1];window.location.pathname=`/journey-${parseInt(e)+1}`},again_2=async()=>{CONTINUE_RENDER=!0,parts=[],parts_deleted=new Set,links=[],links_set=new Set,key_bindings=new Map,emeralds=[],key_allowed=!1,winner=void 0,scores=[-1,-1],document.querySelector("#content").innerHTML=html();const e=document.createElement("style");document.head.appendChild(e);for(let n of"\n    * {\n      color: #ffa;\n      background: #113;\n      font-size: 1.05rem;\n    }\n    select {\n      border: none;\n    }\n    option {\n    }\n    #content {\n      display: flex;\n      width: 100%;\n      height: 100%;\n      align-content: center;\n      align-items: center;\n      flex-direction: row;\n    }\n    a {\n      color: #ffa;\n      text-decoration: none;\n      background-color: #fff0;\n      padding: 0.8rem;\n    }\n    .disappear, .disappear * {\n      color: #0000;\n      transition: color 0.2s;\n    }\n    a:hover {\n      background-color: #fff2;\n    }\n    #score_player_1, #score_player_2 {\n      font-size: 2rem;\n    }\n    #content > div.bob {\n      width: 0;\n      flex-grow: 1;\n      display: flex;\n      justify-content: space-around;\n      flex-direction: column;\n      height: 100%;\n    }\n    p {\n      text-align: center;\n      color: #ffa;\n      font-family: monospace;\n    }\n    p span {\n      color: #ffa;\n    }\n    #canvas {\n        background: #113;\n        display:flex;\n        position: unset;\n    }\n    body {\n      background: #113;\n    }\n    #winner {\n      position: absolute;\n      height: 100vh;\n      width: 100vw;\n      background: #0000;\n      display: flex;\n      flex-direction: column;\n      align-content: center;\n      align-items: center;\n      justify-content: center;\n      pointer-events: none;\n    }\n    #winner > p {\n      background: #ffaa;\n      padding: 5rem;\n      border-radius: 10rem;\n      border: solid 5px #ffa;\n    }\n    #winner > p > span, #winner > p  {\n      color: #ffa;\n      font-size: 3rem;\n    }\n    #winner > p > span {\n      background: transparent;\n    }\n    #winner button {\n      border: none;\n      margin: 2rem;\n      cursor: pointer;\n      pointer-events: all;\n      padding: 1rem;\n      border-radius: 10rem;\n      background: #fff0;\n      line-height: 1.5rem;\n    }\n    #winner div {\n      background: #fff0;\n    }\n    #winner button:hover {\n      background: #fff2;\n    }\n    .hide {\n      display: none !important;\n    }\n    a {\n      border-radius: 10rem;\n    }\n    .player_name {\n      text-align: center;\n      background: none;\n      border: solid 2px transparent;\n      margin-left: 1rem;\n      margin-right: 1rem;\n      padding: 0.5rem;\n      font-size: 1.5rem;\n    }\n    .player_name:hover {\n      border: solid 2px #ffdd;\n    }\n    #best_name {\n      margin-left: 1rem;\n      margin-right: 1rem;\n      padding: 0.5rem;\n      font-size: 1.5rem;\n      border: solid 2px #0000;\n    }\n  ".split("}"))try{e.sheet.insertRule(n+"}")}catch(e){}const n=document.querySelector("#canvas");resize_square(n);const r=n.getContext("2d"),a="2022.08.09";localStorage.getItem("ship_journey")&&localStorage.getItem("version")===a||(localStorage.setItem("ship_journey",default_ship_journey),localStorage.setItem("version",a)),render(r),key_allowed=!1,await add_player_ship(JSON.parse(localStorage.getItem("ship_journey")),.5,.5);const i=new Set;for(let e of key_bindings){const n=e[0],r=e[1];for(let e of r)"booster"==parts[e].kind&&i.add(n)}const t=parseInt(window.location.pathname.split("journey-")[1]);0===t?(emeralds.push(new_emerald(.5,.7)),emeralds.push(new_emerald(.5,.3))):1===t?(await add_ship(ship_2,.27,.5),await add_ship(ship_2,.5,.27),await add_ship(ship_2,.73,.5),await add_ship(ship_2,.5,.73),await add_ship(ship_2,.8,.8),await add_ship(ship_2,.2,.8),await add_ship(ship_2,.8,.2),await add_ship(ship_2,.2,.2),emeralds.push(new_emerald(.33,.33)),emeralds.push(new_emerald(.67,.33)),emeralds.push(new_emerald(.33,.67)),emeralds.push(new_emerald(.67,.67))):2===t?(emeralds.push(new_emerald(.27,.5)),emeralds.push(new_emerald(.5,.27)),emeralds.push(new_emerald(.73,.5)),emeralds.push(new_emerald(.5,.73)),emeralds.push(new_emerald(.8,.8)),emeralds.push(new_emerald(.8,.2)),emeralds.push(new_emerald(.2,.8)),emeralds.push(new_emerald(.2,.2)),emeralds.push(new_emerald(.33,.33)),emeralds.push(new_emerald(.67,.33)),emeralds.push(new_emerald(.33,.67)),emeralds.push(new_emerald(.67,.67))):3===t?(emeralds.push(new_emerald(.27,.5)),emeralds.push(new_emerald(.73,.5)),emeralds.push(new_emerald(.8,.5)),emeralds.push(new_emerald(.2,.5)),emeralds.push(new_emerald(.33,.5)),emeralds.push(new_emerald(.67,.5))):4===t?(emeralds.push(new_emerald(.27,.5)),emeralds.push(new_emerald(.5,.27)),emeralds.push(new_emerald(.73,.5)),emeralds.push(new_emerald(.5,.73)),emeralds.push(new_emerald(.8,.8)),emeralds.push(new_emerald(.2,.8)),emeralds.push(new_emerald(.8,.2)),emeralds.push(new_emerald(.2,.2)),await add_ship(ship_2,.33,.33),await add_ship(ship_2,.67,.33),await add_ship(ship_2,.33,.67),await add_ship(ship_2,.67,.67)):(emeralds.push(new_emerald()),emeralds.push(new_emerald()),emeralds.push(new_emerald()),emeralds.push(new_emerald())),parseInt(localStorage.getItem("progress"))>=t-1&&t<=4?(console.log("ok"),scores[0]=0):console.log("not ok"),i.size&&(document.querySelector("#move_with_instructions").innerHTML=`Move with ${Array.from(i).map((e=>e.toUpperCase())).join(", ")}`),key_allowed=!0,score_to_win=parts.filter((e=>"emerald"==e.kind)).length,start_time=performance.now(),console.log(document.querySelector("#best_duration_current_player").offsetTop),console.log(document.querySelector("#best_duration").offsetTop)},journey_level=async e=>{window.again=again,window.next=next,window.update_player_info=update_player_info,document.addEventListener("keydown",(e=>{if(key_bindings.get(e.key)){if(key_allowed){document.querySelectorAll(".disappearable").forEach(((e,n)=>{e.classList.add("disappear")}));for(let n of key_bindings.get(e.key))parts[n].activated=!0}}else" "!=e.key?"Enter"!=e.key?document.querySelectorAll(".disappearable").forEach(((e,n)=>{e.classList.remove("disappear")})):key_allowed&&void 0!==winner&&next():key_allowed&&again()})),document.addEventListener("keyup",(e=>{if(key_bindings.get(e.key))for(let n of key_bindings.get(e.key))parts[n].activated=!1})),document.addEventListener("mousemove",(e=>{document.querySelectorAll(".disappearable").forEach(((e,n)=>{e.classList.remove("disappear")}))})),again_2(),compute(),document.querySelectorAll(".disappearable").forEach(((e,n)=>{e.classList.remove("disappear")}))};export{journey_level};