# !/bin/sh

GREEN="\e[1;32m"
RED="\e[0;31m"
PURPLE="\e[0;35m"
NC="\e[0m"

# Remove existing .spv files
# spvs=$(find . -name '*.spv')
# for spv in $spvs; do
#   rm "$spv"
# done

function compile {
  local file_path="$1"
  local shader_type="$2"

  local dir=$(dirname "$file_path")
  local filename=$(basename "$file_path")
  local basename=${filename%.*}

  glslc \
    --target-env=opengl4.5 \
    -fshader-stage=$shader_type \
    $file_path \
    -o "$dir/glsl_${basename}.spv"

  glslc \
    --target-env=opengl4.5 \
    -fshader-stage=$shader_type \
    -S \
    $file_path \
    -o "$dir/glsl_${basename}.spv.s"

  if [ $? -eq 0 ]; then
    echo -e "${GREEN}[SUCCESS]${NC} $file_path -> ${PURPLE}glsl_${basename}.spv${NC}"
  else
    echo -e "${RED}[FAILED]${NC} $file_path${NC}"
  fi
}

# Compile .vert files to .spv
echo -e "${PURPLE}GLSLC: compiling...${NC}"
shaders=$(find . -name '*.vert')
for shader in $shaders; do
  compile $shader vert
done

shaders=$(find . -name '*.frag')
for shader in $shaders; do
  compile $shader frag
done
