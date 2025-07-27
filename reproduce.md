# Steps to reproduce loco.rs issue #1555

### These steps are not necesarily all required in order to reproduce, but they should be sufficient to replicate the issue.

## 1. Set up a new saas app with server side rendering
(base) simon@POSEIDON:~/git$ loco new

✔ ❯ App name? · recursive_furnace_reproduce

✔ ❯ What would you like to build? · Saas App with server side rendering

✔ ❯ Select a DB Provider · Postgres

✔ ❯ Select your background worker type · Async (in-process tokio async tasks)

🚂 Loco app generated successfully in:
/home/simon/git/recursive_furnace_reproduce

- database: You've selected `postgres` as your DB provider (you should have a postgres instance to connect to)

## 2. Set database URI in config/development.yaml and config/test.yaml (not sure if postgresql vs sqllite is relevant - but I used postgresql in my original project)

## 3. Run initial project once