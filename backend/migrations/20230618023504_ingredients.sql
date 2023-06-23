-- Add migration script here
INSERT INTO tbl_ingredient
(ingredient_id,name, type) 
VALUES
/* Fruit */
(1, 'Coconut Cream', 'Fruit'),
(2, 'Grapefruit Juice', 'Fruit'),
(3, 'Lime Juice', 'Fruit'),
(4, 'Lemon Juice', 'Fruit'),
(5, 'Orange Juice', 'Fruit'),
(6, 'Pineapple Juice', 'Fruit'),

/* Spirits */
(7, 'Bourbon', 'Spirit'),
(8, 'Gin', 'Spirit'),
(9, 'Mezcal', 'Spirit'),
(10, 'Pisco', 'Spirit'),
(11, 'Rum', 'Spirit'),
(12, 'Tequila', 'Spirit'),
(13, 'Whiskey', 'Spirit'),

/* Liqueur */
(14, 'Aperol', 'Spirit'),
(15, 'Allspice Dram', 'Spirit'),
(16, 'Amaro Nonino Quintessentia', 'Spirit'),
(17, 'Benedictine', 'Spirit'),
(18, 'Blackberry Liqueur', 'Spirit'),
(19, 'Campari', 'Spirit'),
(20, 'Cherry Liqueur', 'Spirit'),
(21, 'Orange Liqueur', 'Spirit'),
(22, 'Peach Liqueur', 'Spirit'),
(23, 'Velvet Falernum', 'Spirit'),

/* Syrups */
(24, 'Demerara Syrup', 'Syrup'),
(25, 'Grenadine', 'Syrup'),
(26, 'Orgeat', 'Syrup'),
(27, 'Simple Syrup', 'Syrup'),
(28, 'Passion Fruit Syrup', 'Syrup'),

/* Bitters */
(29, 'Agnostura Bitters', 'Bitters'),
(30, 'Orange Bitters', 'Bitters'),
(31, 'Peychauds Bitters', 'Bitters');