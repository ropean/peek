# Cloudflare R2 Setup Guide

This guide explains how to obtain the necessary credentials to enable automatic uploads to Cloudflare R2 for peek releases.

## Prerequisites

- A Cloudflare account with R2 enabled
- Access to your Cloudflare dashboard

## Step 1: Create an R2 Bucket

1. Log in to your [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. Navigate to **R2** from the left sidebar
3. Click **Create bucket**
4. Enter bucket name: `r2-aceapp-dev`
5. Choose your preferred location
6. Click **Create bucket**

## Step 2: Configure Public Access (Optional)

To make your bucket publicly accessible via `https://r2.aceapp.dev/`:

1. Go to your bucket settings
2. Navigate to **Settings** → **Public Access**
3. Click **Connect Domain**
4. Enter your custom domain: `r2.aceapp.dev`
5. Follow the DNS configuration instructions
6. Wait for DNS propagation (usually 5-15 minutes)

Alternatively, you can use the default R2.dev subdomain provided by Cloudflare.

## Step 3: Get Your Account ID

1. In the Cloudflare Dashboard, click on **R2** in the sidebar
2. Your **Account ID** is displayed at the top of the R2 overview page
3. It looks like: `a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6`
4. Copy this value - you'll need it for `R2_ACCOUNT_ID`

## Step 4: Create API Token

1. Navigate to **R2** → **Manage R2 API Tokens**
2. Click **Create API token**
3. Configure the token:
   - **Token name**: `peek-github-upload` (or any descriptive name)
   - **Permissions**:
     - ✅ Object Read & Write
     - ✅ (Optional) Bucket List if you want to list buckets
   - **TTL**: Set to "Forever" or choose an expiration date
   - **Specific Buckets**: Select `r2-aceapp-dev` (recommended for security)
4. Click **Create API Token**

## Step 5: Save Credentials

After creating the token, you'll see:

```text
Access Key ID: <your-access-key-id>
Secret Access Key: <your-secret-access-key>
```

⚠️ **Important**: The Secret Access Key is only shown once. Copy it immediately!

## Step 6: Configure GitHub Secrets

1. Go to your GitHub repository: `https://github.com/ropean/peek`
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret** and add the following three secrets:

### Secret 1: R2_ACCESS_KEY_ID

- **Name**: `R2_ACCESS_KEY_ID`
- **Value**: The Access Key ID from Step 5

### Secret 2: R2_SECRET_ACCESS_KEY

- **Name**: `R2_SECRET_ACCESS_KEY`
- **Value**: The Secret Access Key from Step 5

### Secret 3: R2_ACCOUNT_ID

- **Name**: `R2_ACCOUNT_ID`
- **Value**: Your Account ID from Step 3

## Verification

To verify your setup:

1. Manually trigger the workflow:

   - Go to **Actions** → **Upload to Cloudflare R2**
   - Click **Run workflow**
   - Enter a tag (e.g., `v1.1.3`)
   - Click **Run workflow**

2. Check the workflow logs for successful uploads

3. Verify files are accessible:

   ```text
   https://r2.aceapp.dev/peek/v1.1.3/peek-windows-x64.exe
   https://r2.aceapp.dev/peek/v1.1.3/index.html
   ```

## Security Best Practices

✅ **Do:**

- Limit API token permissions to specific buckets only
- Use descriptive names for API tokens
- Rotate tokens periodically
- Keep Secret Access Key secure and never commit to Git

❌ **Don't:**

- Share API credentials publicly
- Grant more permissions than necessary
- Use the same token for multiple projects

## Troubleshooting

### Error: "Access Denied"

- Verify that the API token has write permissions
- Ensure the bucket name matches exactly: `r2-aceapp-dev`
- Check that the token hasn't expired

### Error: "Endpoint not found"

- Verify your Account ID is correct
- Check the R2 endpoint format: `https://<account-id>.r2.cloudflarestorage.com`

### Files not publicly accessible

- Ensure public access is configured for the bucket
- Verify custom domain DNS settings
- Check CORS settings if accessing from browsers

## Additional Resources

- [Cloudflare R2 Documentation](https://developers.cloudflare.com/r2/)
- [R2 API Reference](https://developers.cloudflare.com/r2/api/s3/api/)
- [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)

## Support

If you encounter any issues:

1. Check the GitHub Actions workflow logs
2. Review Cloudflare R2 analytics for error details
3. Open an issue in the [peek repository](https://github.com/ropean/peek/issues)
