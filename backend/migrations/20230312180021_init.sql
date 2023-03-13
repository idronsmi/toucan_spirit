-- Add migration script here
CREATE TABLE tbl_prep (
    prep_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    how TEXT not null
);

CREATE TABLE tbl_recipe (
    recipe_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(100) NOT NULL,
    prep_id BIGINT REFERENCES tbl_prep(prep_id) NOT NULL
);

CREATE TABLE tbl_ingredient (
    ingredient_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(20) NOT NULL,
    type VARCHAR(20) NOT NULL
);

CREATE TABLE tbl_recipe_ingredient (
    recipe_ingredient_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    recipe_id BIGINT REFERENCES tbl_recipe(recipe_id) NOT NULL,
    ingredient_id BIGINT REFERENCES tbl_ingredient(ingredient_id) NOT NULL,
    quantity INTEGER NOT NULL,
    unit_of_easurement VARCHAR(10) NOT NULL
);