#!/usr/bin/env bash
set -euo pipefail

# Deploy the ./website directory to Cloudflare Pages using wrangler.
# Usage: ./scripts/deploy_pages.sh [project-name] [directory]
PROJECT_NAME=${1:-peek}
DIR=${2:-website}

echo "Deploying '$DIR' to Cloudflare Pages project '$PROJECT_NAME'"

if ! command -v wrangler >/dev/null 2>&1; then
  echo "wrangler CLI not found. Install with: npm i -g wrangler or refer to https://developers.cloudflare.com/workers/cli-wrangler/" >&2
  exit 1
fi

if [ -z "${CLOUDFLARE_API_TOKEN:-}" ]; then
  echo "Warning: CLOUDFLARE_API_TOKEN is not set. wrangler may use existing auth or fail." >&2
fi

echo "Running: wrangler pages deploy $DIR --project-name $PROJECT_NAME --branch main"
wrangler pages deploy "$DIR" --project-name "$PROJECT_NAME" --branch main

echo "Deployment command finished. Visit https://pages.cloudflare.com/ to see details or map your custom domain to the Pages site." 
