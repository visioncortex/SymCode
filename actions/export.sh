# export visioncortex
rsync -av --delete "./visioncortex/src" "../visioncortex"
rsync -av "./visioncortex/Attributions.md" "../visioncortex"
rsync -av "./visioncortex/README.md" "../visioncortex"