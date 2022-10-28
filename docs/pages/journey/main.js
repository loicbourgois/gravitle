import{resize_square,stroke_circle,stroke_circle_2,fill_circle_2,fill_circle,clear,line,fill_text}from"../canvas";const level_count=2;localStorage.getItem("progress")||localStorage.setItem("progress",0);const progress=localStorage.getItem("progress"),levels=()=>{const n=[];for(var e=0;e<2;e++){const l=[];for(var o=0;o<2;o++){const n=2*e+o;l.push({locked:n>progress?"locked":"",idx:n,label:(2*e+o+1).toLocaleString("en-US",{minimumIntegerDigits:2,useGrouping:!1})})}n.push(l.map((n=>n.locked?`<p class="level_link locked">${n.label}</p>`:`<a class="level_link" href="journey-${n.idx+1}">${n.label}</a>`)).join(""))}return n.map((n=>`<div class="line">${n}</div>`)).join("")},html=()=>`\n    <div class="bob">\n      <p> <a href="/">Home</a> </p>\n\n    </div>\n\n    <div id="levels">\n      ${levels()}\n    </div>\n\n    <div class="bob">\n    \x3c!--  <p> <a href="/leaderboard">Leaderboard</a> </p>--\x3e\n      <p> <a href="/journey-garage">Garage</a> </p>\n    </div>\n  `,style=()=>"\n    #levels > * {\n      flex-grow: 1;\n    }\n    * {\n      color: #ffa;\n      background: #113;\n      font-size: 1.05rem;\n    }\n    select {\n      border: none;\n    }\n    option {\n    }\n    #content {\n      display: flex;\n      width: 100%;\n      height: 100%;\n      align-content: center;\n      align-items: center;\n      flex-direction: row;\n    }\n    a, .level_link {\n      color: #ffa;\n      text-decoration: none;\n      background-color: #fff0;\n      padding: 0.8rem;\n    }\n    .level_link.locked {\n      color: #aaa;\n    }\n    a:hover {\n      background-color: #fff2;\n    }\n    #score_player_1, #score_player_2 {\n      font-size: 2rem;\n    }\n    #content > div.bob {\n      width: 0;\n      flex-grow: 1;\n      display: flex;\n      justify-content: space-around;\n      flex-direction: column;\n      height: 100%;\n    }\n    p {\n      text-align: center;\n      color: #ffa;\n      font-family: monospace;\n    }\n    p span {\n      color: #ffa;\n    }\n    #canvas {\n        background: #113;\n        display:flex;\n        position: unset;\n    }\n    body {\n      background: #113;\n    }\n    a {\n      padding: 2rem;\n      border-radius: 10rem;\n    }\n    #levels {\n      display: flex;\n      flex-wrap: wrap;\n      align-content: center;\n      align-items: center;\n      flex-direction: column;\n      justify-content: space-around;\n      justify-items: stretch;\n      flex-flow: column wrap;\n      place-content: center space-around;\n      place-items: center;\n      padding: 1rem;\n      flex-direction: column;\n      justify-content: space-around;\n      padding: 1rem;\n    }\n    #levels > * {\n      flex-grow: 1;\n      display: flex;\n      flex-direction: row;\n      align-content: center;\n      align-items: center;\n      display: flex;\n      flex-direction: row;\n      align-content: center;\n      align-items: center;\n    }\n  ",journey_main=()=>{document.querySelector("#content").innerHTML=`\n    <div class="bob">\n      <p> <a href="/">Home</a> </p>\n\n    </div>\n\n    <div id="levels">\n      ${levels()}\n    </div>\n\n    <div class="bob">\n    \x3c!--  <p> <a href="/leaderboard">Leaderboard</a> </p>--\x3e\n      <p> <a href="/journey-garage">Garage</a> </p>\n    </div>\n  `;const n=document.createElement("style");document.head.appendChild(n);for(let e of"\n    #levels > * {\n      flex-grow: 1;\n    }\n    * {\n      color: #ffa;\n      background: #113;\n      font-size: 1.05rem;\n    }\n    select {\n      border: none;\n    }\n    option {\n    }\n    #content {\n      display: flex;\n      width: 100%;\n      height: 100%;\n      align-content: center;\n      align-items: center;\n      flex-direction: row;\n    }\n    a, .level_link {\n      color: #ffa;\n      text-decoration: none;\n      background-color: #fff0;\n      padding: 0.8rem;\n    }\n    .level_link.locked {\n      color: #aaa;\n    }\n    a:hover {\n      background-color: #fff2;\n    }\n    #score_player_1, #score_player_2 {\n      font-size: 2rem;\n    }\n    #content > div.bob {\n      width: 0;\n      flex-grow: 1;\n      display: flex;\n      justify-content: space-around;\n      flex-direction: column;\n      height: 100%;\n    }\n    p {\n      text-align: center;\n      color: #ffa;\n      font-family: monospace;\n    }\n    p span {\n      color: #ffa;\n    }\n    #canvas {\n        background: #113;\n        display:flex;\n        position: unset;\n    }\n    body {\n      background: #113;\n    }\n    a {\n      padding: 2rem;\n      border-radius: 10rem;\n    }\n    #levels {\n      display: flex;\n      flex-wrap: wrap;\n      align-content: center;\n      align-items: center;\n      flex-direction: column;\n      justify-content: space-around;\n      justify-items: stretch;\n      flex-flow: column wrap;\n      place-content: center space-around;\n      place-items: center;\n      padding: 1rem;\n      flex-direction: column;\n      justify-content: space-around;\n      padding: 1rem;\n    }\n    #levels > * {\n      flex-grow: 1;\n      display: flex;\n      flex-direction: row;\n      align-content: center;\n      align-items: center;\n      display: flex;\n      flex-direction: row;\n      align-content: center;\n      align-items: center;\n    }\n  ".split("}"))try{n.sheet.insertRule(e+"}")}catch(n){}};export{journey_main};