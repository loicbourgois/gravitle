pwd_=$(pwd)
cd $gravitle_root_folder/gravitle-gpu
cargo fmt --verbose
cargo check || { cd $pwd_ ; exit 1; }
configuration=$(echo $(cat "$gravitle_root_folder/gravitle-gpu/configurations/${config}.json") | sed 's#\"#\\\"#g' )
configuration=$configuration cargo run --release
cd $pwd
