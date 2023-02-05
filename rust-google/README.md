# Rust Project using Google APIs (rust-google)

## About

My gathering up of some very basic abstractions to simplify automating against some Google APIs.

- Read, update and append to Google Sheets
- Read and download files from Drive

Probably will add before long:

- Basic Calendar operations
- YouTube subscription and notification operations

## Examples

There are some examples in `pmc-google/examples`. Run them like so:

```bash
export GOOGLE_SA_PATH='/path/to/your/serviceaccount.json'
cd pmc-google
cargo run --example sheets -- <sheet_id>
```

If you don't already have a Service Account, see the [Getting Started](#getting-started) section, below.

## Getting Started

### Preparing a Service Account

Complete the following steps in the [Google Cloud Console](https://console.cloud.google.com) in order to prepare a Service Account that your
app can use to authenticate with the desired APIs non-interactively.

#### Create a Google Cloud Project

Create a Project, if you don't already have one you want to use.

[Google Cloud :: Create a Project](https://developers.google.com/workspace/guides/create-project) 

#### Enable APIs for the Project

Enable any APIs your project requires, in __More products > Google Workspace > Product Library__.

[Google Cloud :: Enable APIs](https://developers.google.com/workspace/guides/enable-apis)

#### Create a Service Account

With your Project selected, in __APIs & Services >> Credentials__, create a new Service Account. Grant it the
minimum required access level to your APIs.

With the Service Account created:
1. Click it to view its details.
2. Select the Keys tab.
3. Click Add key->Create new Key->JSON.
  - __The resulting JSON download is sensitive. Store it somewhere safe.__
  - We'll refer to this file as `serviceaccount.json`, but you may name it what you want.
4. Set environment variable `GOOGLE_SA_FILE` to the path of the JSON.

```bash
export GOOGLE_SA_PATH="/path/to/your/serviceaccount.json"
```

#### Grant application-specific access

Specifics will vary from service to service, but in Google Sheets for example, you'll need to share
the desired spreadsheets with the new service account (using its full email ID) before it will be able
to access them.

1. __IAM & Admin -> Service Accounts -> Select Service Account -> Get the 'Email' value__
2. In Google Sheets, use the sharing function to share resources with the Service Account, setting the desired permission level as you do so.

### Request additional permission Scopes

Each service defines its own set of (oauth2) permission scopes. Available scopes are documented, [here](https://developers.google.com/identity/protocols/oauth2/scopes).

If after performing all the above steps, you still get a 403 error when attempting some operations, it's quite 
possible that you need to include an additional scope in that particular API call. See the your library's docs for the specifics,
but as an example, to add a scope while making a Google Drive API request using the `google-drive4` crate, you need to use the `add_scope("<scope_url>")` builder method, like so:

```rust
  let result = self
      .hub
      .files()
      .get(file_id)
      .param("alt", "media")
      .supports_all_drives(true)
      .add_scope("https://www.googleapis.com/auth/drive")
      .doit()
      .await;
```

In this case, we're downloading a file, which would fail with a 403 if not for the additional scope specified.

