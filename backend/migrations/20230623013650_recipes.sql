-- Add migration script here
insert into tbl_prep
(prep_id, how)
VALUES
(1, 
'1. Shake ingredients in shaker tin vigourously for 12-15 seconds.
2. Double Strain into glass.');

insert into tbl_recipe
(recipe_id, name, prep_id)
VALUES
(1, 'Paper Plane', 1);

insert into tbl_recipe_ingredient
(recipe_id,
 ingredient_id,
 quantity )
VALUES
(1, 14, .75),
(1, 7, .75),
(1, 4, .75),
(1, 16, .75);