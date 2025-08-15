#!/bin/sh
TCNAME="${1}"

cargo clean
OUTPUT=$(cargo ${TCNAME:+"+${TCNAME}"} clippy 2>&1)
_RET=$?
if [[ $_RET -ne 0 ]]; then
	echo $OUTPUT >&2
	exit $_RET
fi

if echo "$OUTPUT" | grep -q "use of a disallowed macro \`macrolib::attrib_macro\`"; then
	exit 1
else
	exit 0
fi
