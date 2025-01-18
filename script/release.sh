clear

if [ -z "$1" ]; then
  echo "Please Provide release version e.g. 'v0.0.0'"
  exit 1
fi

echo "Releasing Sandy $1"

RELEASE_DIR=${PWD}/target/release
echo "    RELEASE_DIR : $RELEASE_DIR"

cargo build --release
cp "$RELEASE_DIR/sandy" "$RELEASE_DIR/sandy_linux_$1"

cargo build --release --target=x86_64-pc-windows-gnu
cp ${PWD}/target/x86_64-pc-windows-gnu/release/sandy.exe  "$RELEASE_DIR/sandy_windows_$1.exe"


rm -fr ${RELEASE_DIR}/sandy_workspace
cp -r ${PWD}/lua ${RELEASE_DIR}/sandy_workspace
rm ${RELEASE_DIR}/sandy_workspace/.luarc.docs.json
cd ${RELEASE_DIR}
tar czf ./sandy_workspace_$1.tar.gz ./sandy_workspace



