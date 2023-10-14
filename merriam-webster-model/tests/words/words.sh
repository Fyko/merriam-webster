#!/bin/bash

api_key=${API_KEY:-$(echo $API_KEY)}

if [[ -z $api_key ]]; then
    echo "Error: API_KEY environment variable is not set."
    exit 1
fi

urlencode() {
    local string="$1"
    local strlen=${#string}
    local encoded=""
    local pos c o

    for (( pos=0 ; pos<strlen ; pos++ )); do
       c=${string:$pos:1}
       case "$c" in
           [-_.~a-zA-Z0-9] ) o="${c}" ;;
           * )               printf -v o '%%%02x' "'$c"
       esac
       encoded+="${o}"
    done
    echo "${encoded}" 
}

# a function to snakify a string.
snake() {
	echo "$@" | sed -r 's/([A-Z])/_\1/g' | tr '[:upper:]' '[:lower:]'
}

# Fetch words from the homepage endpoint
words_json=$(curl -s "https://www.merriam-webster.com/lapi/v1/mwol-mp/get-lookups-data-homepage")

# Extract words array using jq
words=($(echo $words_json | jq -r '.data.words[]'))

for word in "${words[@]}"; do
	encodedWord=$(urlencode "$word")
    # Fetch word definitions
	definitions_json=$(curl -s "https://www.dictionaryapi.com/api/v3/references/collegiate/json/$encodedWord?key=$API_KEY")

    # Write definitions to a file named after the word
    echo $definitions_json | jq '.' > "$word.json"

    echo "Saved definitions for $word"
done
