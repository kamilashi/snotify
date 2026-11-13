#!/usr/bin/env bash
set -euo pipefail

# Checks if a file exists -
# creates it if no, prompts to overwrite otherwise.
# $1 is the path to the file.
create_or_overwrite() {
    local file_path="$1"

    if ! [ -e "${file_path}" ]; then
        touch "${file_path}"    
    else
        echo "${file_path} will be overwritten!"
        echo "press Enter to continue or Ctrl + C to exit"
        read -r -p ""
    fi
}

declare dir_prefix="."

if [[ "$(basename "$PWD")" == "setup_dev_env" ]]; then
    dir_prefix=".." 
fi

declare -r vscode_ext_dir="${dir_prefix}/.vscode"
declare -r vscode_ext_path="${vscode_ext_dir}/extensions.json"
declare -r vscode_settings_path="${vscode_ext_dir}/settings.json"
declare -r vscode_config_path="${dir_prefix}/.editorconfig"
declare -r envrc_path="${dir_prefix}/.envrc"

echo "Setting up VSCode dev environment..."

if ! [ -d ${vscode_ext_dir} ]; then
    mkdir -p ${vscode_ext_dir}   
fi

create_or_overwrite ${vscode_ext_path}

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

create_or_overwrite ${vscode_settings_path}

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

create_or_overwrite ${vscode_config_path}

cat > "$vscode_config_path" <<'EOF'
root = true

[*.{js,jsx,mjs,cjs,ts,tsx,mts,cts,vue,css,scss,sass,less,styl}]
charset = utf-8
indent_size = 2
indent_style = space
insert_final_newline = true
trim_trailing_whitespace = true
end_of_line = lf
max_line_length = 100

[*.toml]
indent_size = 2
indent_style = space

[*.md]
trim_trailing_whitespace = false
EOF

echo "Setting up automatic direnv shell mounting"
if [ -e ${envrc_path} ]; then
    echo "${envrc_path} will be overwritten!"
    echo "press Enter to continue or Ctrl + C to exit"
    read -r -p ""
fi

echo "use flake" > ${envrc_path}

direnv allow
