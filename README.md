# SkyBridge

![SkyBridge](public/banner.png)

[![style: very good analysis][very_good_analysis_badge]][very_good_analysis_link]
[![License: MIT][license_badge]][license_link]
[![Powered by Dart Frog](https://img.shields.io/endpoint?url=https://tinyurl.com/dartfrog-badge)](https://dartfrog.vgv.dev)

A work in progress bridge/proxy that allows you to use [Mastodon](https://joinmastodon.org) apps with 
[Bluesky](https://bsky.app).

SkyBridge attempts to convert Mastodon API calls to Bluesky API calls, and tries to implement enough of the Mastodon API
that apps will be happy connecting to it.

Functionality is very basic at the moment, and there's bound to be a few bugs. Don't expect
this to be your daily driver just yet!

Currently only being tested with [Ivory](https://tapbots.com/ivory) on iOS and macOS, your mileage may vary with
other apps at the moment. PRs and issues are welcome, we want to support as many apps as possible.

If you find this project useful please consider donating, it really helps a lot!

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/videah)

## Getting Started
In order to run SkyBridge you will need the [Dart SDK](https://dart.dev/get-dart) installed on your system.

You will also need the Dart Frog CLI installed to run a development server. You can install it with the following
command:

```shell
# 📦 Install the dart_frog cli from pub.dev
dart pub global activate dart_frog_cli
```

Some environment variables need to be set before you can run SkyBridge. You can set them in a `.env` file in the root
of the project. You can copy the `.env.example` file to get started.

```shell
# 📝 Copy and edit the example .env file
cp .env.example .env
open .env
```

Once that's done you can start a development server with the following command:

```shell
# 📥 Clone the repo
git clone https://github.com/videah/skybridge.git
cd skybridge
dart pub get

# 🏁 Start the dev server
dart_frog dev
```

## Self Hosting
The instance of SkyBridge hosted at [skybridge.fly.dev](https://skybridge.fly.dev) is provided as a free public service
but currently faces heavy rate limiting from Bluesky due to all the requests coming from a single IPv4 address.
If you want a more consistent experience you can host your own instance to avoid this.

### Docker
The easiest way to run SkyBridge yourself is with Docker. The image [videah/skybridge](https://hub.docker.com/r/videah/skybridge)
is kept up to date with the main branch of this repo.

You can use the [docker-compose.yml](docker-compose.yml) file in the root of the project to get started.
Simply change the environment variables in the file, and tweak it to suit your setup. You probably want to stick a reverse proxy
like [nginx](https://nginx.org) or [caddy](https://caddyserver.com) in front of it.

Please note that by default, the web server runs on port 8080, and uses HTTP when setting up your reverse proxy.

### Fly.io
The public instance of SkyBridge is hosted on [Fly.io](https://fly.io). For a single user it is likely possible to
host an instance here for free. A [fly.toml](fly.toml) file is included in the root of the project to help you get started.

## Credits
Huge thanks to [Ninji](https://github.com/Treeki) for their [BirdBridge](https://github.com/Treeki/BirdBridge) project
which was a big help and inspiration for this project!

Thank you to [Shinya Kato](https://github.com/myConsciousness) for 
[atproto.dart](https://github.com/myConsciousness/atproto.dart) which powers this project!

[license_badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license_link]: https://opensource.org/licenses/MIT
[very_good_analysis_badge]: https://img.shields.io/badge/style-very_good_analysis-B22C89.svg
[very_good_analysis_link]: https://pub.dev/packages/very_good_analysis
