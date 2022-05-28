#!/usr/bin/env bash
set -euo pipefail

version=${1}

sed -i "s/## Unreleased/## Unreleased\n\n## ${version}/" CHANGELOG.md
find -name Cargo.toml | xargs sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/"
sed -i "s/rev .*/rev \"v${version}\")/" hardware/Module.kicad_sch
sed -i "s/gr_text \"board .*\" /gr_text \"board v${version}\" /" hardware/Module.kicad_pcb
sed -i "s/rev .*/rev \"v${version}\")/" hardware/Module.kicad_pcb
