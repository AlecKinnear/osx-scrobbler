#!/bin/bash

# ListenBrainz API Test Script
# Tests basic connectivity and listens submission to ListenBrainz
# Usage: TOKEN=your_token ./test_listenbrainz.sh

set -e

# Configuration
TOKEN="${TOKEN:-}"
API_URL="${API_URL:-https://api.listenbrainz.org}"
ARTIST="${ARTIST:-Samantha Fox}"
TRACK="${TRACK:-Touch Me}"

# Validate token
if [ -z "$TOKEN" ]; then
    echo "Error: TOKEN environment variable is required"
    echo "Usage: TOKEN=your_listenbrainz_token ./test_listenbrainz.sh"
    echo ""
    echo "Get your token from: https://listenbrainz.org/settings/"
    exit 1
fi

echo "ListenBrainz API Test"
echo "===================="
echo "API URL: $API_URL"
echo "Artist: $ARTIST"
echo "Track: $TRACK"
echo ""

# Create payload with current timestamp
TIMESTAMP=$(date +%s)
PAYLOAD=$(cat <<EOF
{
  "listen_type": "single",
  "payload": [
    {
      "listened_at": $TIMESTAMP,
      "track_metadata": {
        "artist_name": "$ARTIST",
        "track_name": "$TRACK"
      }
    }
  ]
}
EOF
)

echo "Submitting listen..."
echo ""

# Send to ListenBrainz
echo "Request:"
echo "$PAYLOAD"
echo ""

HTTP_CODE=$(curl -s -o /tmp/lb_response.txt -w "%{http_code}" \
  -X POST "$API_URL/1/submit-listens" \
  -H "Authorization: Token $TOKEN" \
  -H "Content-Type: application/json" \
  -d "$PAYLOAD")

BODY=$(cat /tmp/lb_response.txt)
rm -f /tmp/lb_response.txt

echo "HTTP Status: $HTTP_CODE"
echo "Response: $BODY"
echo ""

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ Success! Listen submitted to ListenBrainz"
    echo ""
    echo "Check your feed at: https://listenbrainz.org/feed/"
    exit 0
else
    echo "❌ Error! Failed to submit listen"
    exit 1
fi
