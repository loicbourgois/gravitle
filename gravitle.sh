enable_strict_mode() {
  set -e -o pipefail
  set -o nounset
}
disable_strict_mode() {
  set +o nounset
}
assert_not_none_nor_empty() {
  local callback=$1
  disable_strict_mode
  local variable=$2
  enable_strict_mode
  if [[ -z "$variable" ]]; then
    $callback 1>&2
    exit 1
  fi
}
get_script_dir() {
  # adapted from https://stackoverflow.com/a/246128
  local source=${BASH_SOURCE[0]}
  local dir=""
  # resolve $SOURCE until the file is no longer a symlink
  while [ -h "$source" ]; do
    dir="$( cd -P "$( dirname "$source" )" >/dev/null 2>&1 && pwd )"
    source="$(readlink "$source")"
    # if $SOURCE was a relative symlink,
    # we need to resolve it relative to the path where the symlink file was located
    [[ $source != /* ]] && source="$dir/$source"
  done
  dir="$( cd -P "$( dirname "$source" )" >/dev/null 2>&1 && pwd )"
  echo $dir
}
about() {
  jq -n \
    --arg gravitle "${BASH_SOURCE[0]}" \
    --arg cargo_version "$(cargo --version)" \
    --arg cargo_which "$(which cargo)" \
    '{
      "gravitle":$gravitle,
      "dependencies":{
        "cargo": {
          "expected": {
            "version": "*",
            "from": "TODO"
          },
          "using": {
            "version": $cargo_version,
            "at": $cargo_which
          }
        }
      }
    }'
}
help_ok() {
  echo " $alias about"
  echo " $alias help"
  echo " $alias website start"
  echo " $alias gpu <config>"
}
help_error() {
  help_ok 1>&2
  exit 1
}
website_start() {
  local pwd_=$(pwd)
  cd $gravitle_dir/gravitle-website
  npm start
  cd $pwd_
}
gpu() {
  local config=$1
  local pwd_=$(pwd)
  cd $gravitle_dir/gravitle-gpu
  cargo fmt --verbose
  cargo check || { cd $pwd_ ; exit 1; }
  local configuration=$(echo $(cat "$gravitle_dir/gravitle-gpu/configurations/${config}.json") | sed 's#\"#\\\"#g' )
  open $gravitle_dir/gravitle-gpu/client/index.html
  configuration=$configuration cargo run --release
  cd $pwd
}
main() {
  assert_not_none_nor_empty help_error $1
  enable_strict_mode
  local command_1=$1
  local command_2=${2:-""}
  case "$command_1" in
    about)
      about
      ;;
    help)
      help_ok
      ;;
    gpu)
      gpu $command_2
      ;;
    website)
      case "$command_2" in
        start)
          website_start
          ;;
        *)
          help_error
          ;;
        esac
      ;;
    *)
      help_error
      ;;
  esac
}
alias="\$gravitle"
gravitle_dir=$(get_script_dir)
main $*
