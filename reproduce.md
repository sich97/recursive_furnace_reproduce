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

## 4. Create unit_categories table
cargo loco generate model unit_categories user:references:created_by name:text^

## 5. Create unit_bases table
cargo loco generate model unit_bases user:references:created_by unit_categorie:references:unit_category name_base:text^ name_plural:text^ symbol:text

## 6. Create units table
cargo loco generate model units user:references:created_by unit_base:references:unit_base multiplier_name_prefix:text^ multiplier_symbol_prefix:text

## 7. Create global_materials table
cargo loco generate model global_materials user:references:created_by unit:references:unit name:text hash:text^

## 8. Create global_recipes table
cargo loco generate model global_recipes user:references:created_by machine_fuel_consumption:double hash:text^

## 9. Create migration for input_materials join table between global_recipes table and global_materials table
cargo loco generate migration CreateJoinTableGlobal_recipesAndGlobal_materials quantity:int! chance:float!

## 10. (ERROR HERE) Run migration for the join table creation
cargo loco db migrate