#mv ./docs/CNAME ./CNAME.tmp;
wasm-pack build;
cd ./frontend;
npm run-script build;
cd ..;
cp -r ./frontend/dist/* ./docs;
cp docs/index.html docs/404.html;
#mv ./CNAME.tmp ./docs/CNAME;
#git commit -am 'Deploy';
#git push origin master;
