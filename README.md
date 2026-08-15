# starboard-4
A pretty good starboard bot.

 - [invite](https://discord.com/api/oauth2/authorize?client_id=700796664276844612&permissions=275683339328&scope=applications.commands%20bot)
 - [support](https://discord.gg/3gK8mSA)
 - [docs](https://starboard.best)

## Features
 - Multiple starboards per server
 - Multiple emojis per starboard
 - 25+ configurable options for starboards
 - Per-channel setting overrides
 - Per-role permissions
 - Autostar channels

## Quickstart
 - Use `/starboards create name: starboard-name channel: #starboard` to create a starboard.
 - Use `/starboards view name: starboard-name` to view the settings.
 - Use `/starboards edit [behavior|requirements|style|embed] name: starboard-name [options...]` to
   edit a starboard.

## Self-hosting
You're welcome to self-host this bot if you like, as well as fork it and add your own changes. If
there are features you wish the main bot had, you're also welcome to open PRs or make an issue
requesting features.

Currently, I offer support for self-hosting the bot if you get stuck - just join the support
server, and create a thread with the "Self Hosting" tag.

This guide assumes that you already have a VPS to host the bot on. Some good low-cost providers
are https://www.netcup.eu (what the main bot uses) and https://alphavps.com.

I'll add a proper self-hosting guide later, but essentially you need a PostgreSQL database, a
.env file filled with the required variables from .env.example, and the pre-built container image.

To run just the pre-built container image standalone:

```bash
podman pull circuitsacul/starboard:latest
podman run -d --env-file .env --network=host circuitsacul/starboard:latest
```

You'll likely want to use compose though. You can see how to use the example compose.yml
in the ## Development section below.

## Migration from Starboard-3
If you're already hosting starboard-3 and want to switch to starboard-4, you have to alter the
database structure.

Start by running `psql postgresql`. Then you can run:
```sql
CREATE DATABASE starboard_4 WITH OWNER <starboard username> TEMPLATE <old database name>;
```

Now exit, and then run `psql starboard_4 < migrate.sql` where `migrate.sql` is the file found in
this repo.

After doing this, you should be able to launch starboard-4.

## Development

The easiest way to run the bot for development is to just use the compose.yml file. Officially,
I use podman, though docker likely works fine (and is what I used previously).

```bash
podman compose up --build -d --force-recreate
```

The docker command is likely equivalent, though you may not need `--force-recreate`.
