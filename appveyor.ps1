if ($env:FUNCTIONALTESTS -eq 'true') {
  echo "---> Functionaltests"
  cargo build
  cargo test
  $env:TEST_FILE = "..\test\nvimpam_spec.lua"
  #Install-Product node 8
  cd neovim
  ci\build.ps1
} else {
  echo "---> No Functionaltests"
  cargo build
  cargo test
}
