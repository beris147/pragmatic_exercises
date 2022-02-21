SRC=''
GEN='false'

print_usage() {
    cat <<EOF
    Script than signals all the files with camelCase in them
        script -s SRC [-g] [-h]

    -s  source directory of the project
    -g  generate alt project with snake_case
    -h show this
EOF
}

while getopts 's:gh' flag; do
  case "${flag}" in
    g) GEN='true' ;;
    s) SRC="${OPTARG}" ;;
    h) print_usage
        exit 0 ;;
    *) print_usage
       exit 1 ;;
  esac
done

if [ ! -d "$SRC" ]; then
    echo "Provided directory $SRC does not exists"
    exit 1;
fi

OUTPUT=$SRC/../snake_case_out

if [ $GEN = true ]; then 
    rm -rf $OUTPUT
    mkdir -p $OUTPUT
fi

for FILE in $(find "$SRC")
do
    if [ -d "$FILE" ]; then continue; fi
    DIR="$(dirname "$FILE")"
    BASENAME="$(basename -- $FILE)"
    sed -r 's/([A-Z])/_\L\1/g' "$FILE" && printf '%s\n' "$FILE"
    if [ $GEN = true ]; then
        mkdir -p $OUTPUT/$DIR
        sed -r 's/([A-Z])/_\L\1/g' $FILE > $OUTPUT/$DIR/$BASENAME;
    fi
done