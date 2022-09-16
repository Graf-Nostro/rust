# RUST

## Install on Alpine

 - Install gcc and curl
   `apk add gcc curl`
 - Pull rust: 
   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
 - Fix gcc and ld build error **cannot find crti.o: No such file or directory** 
   `apk add --no-cache musl-dev`
