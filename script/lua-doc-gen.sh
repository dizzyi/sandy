mv ./lua/.luarc.json ./lua/.luarc.temp.json
mv ./lua/.luarc.docs.json ./lua/.luarc.json

cat ./lua/.luarc.json
lua-language-server --doc=./lua/ --doc_out_path=./manual/ # --configpath=../lua/.luarc.json
rm ./manual/doc.json

mv ./lua/.luarc.json ./lua/.luarc.docs.json
mv ./lua/.luarc.temp.json ./lua/.luarc.json
