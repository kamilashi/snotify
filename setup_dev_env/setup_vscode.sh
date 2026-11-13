#!/usr/bin/env bash
set -euo pipefail

declare dir_prefix="."

if [[ "$(basename "$PWD")" == "setup_dev_env" ]]; then
    dir_prefix=".." 
fi

declare -r vscode_ext_dir="${dir_prefix}/.vscode"
declare -r vscode_ext_path="${vscode_ext_dir}/extensions.json"
declare -r vscode_settings_path="${vscode_ext_dir}/settings.json"
echo "Setting up VSCode dev environment..."

if ! [ -d ${vscode_ext_dir} ]; then
    mkdir -p ${vscode_ext_dir}   
fi

if ! [ -e ${vscode_ext_path} ]; then
    touch ${vscode_ext_path}    
else
    echo "${vscode_ext_path} will be overwritten!"
    echo "press Enter to continue or Ctrl + C to exit"
    read -r -p ""
fi

cat > "$vscode_ext_path" <<'EOF'
{
  "recommendations": [
    "Vue.volar",
    "dbaeumer.vscode-eslint",
    "EditorConfig.EditorConfig",
    "oxc.oxc-vscode",
    "esbenp.prettier-vscode",
    "jnoortheen.nix-ide"
  ]
}
EOF

if ! [ -e ${vscode_settings_path} ]; then
    touch ${vscode_settings_path}    
else
    echo "${vscode_settings_path} will be overwritten!"
    echo "press Enter to continue or Ctrl + C to exit"
    read -r -p ""
fi

cat > "$vscode_settings_path" <<'EOF'
{
    "explorer.fileNesting.patterns": {
        "tsconfig.json": "tsconfig.*.json, env.d.ts, typed-router.d.ts",
        "vite.config.*": "jsconfig*, vitest.config.*, cypress.config.*, playwright.config.*",
        "package.json": "package-lock.json, pnpm*, .yarnrc*, yarn*, .eslint*, eslint*, .oxlint*, oxlint*, .oxfmt*, .prettier*, prettier*, .editorconfig, bun.lock, nub.lock"
    },
    "editor.codeActionsOnSave": {
        "source.fixAll": "explicit"
    },
    "editor.formatOnSave": true,
    "[javascript][typescript][vue][json][jsonc][css][scss][html]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
EOF
