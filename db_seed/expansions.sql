-- Pokemon TCG Expansions Seed Data
-- This file contains all English Pokemon TCG expansion sets from 1999-2025
-- Format: (name, cards, secret_cards)

INSERT OR IGNORE INTO expansions (name, cards, secret_cards) VALUES

-- Base Series (1999-2000)
('Base Set', 102, 0),
('Jungle', 64, 0),
('Fossil', 62, 0),
('Base Set 2', 130, 0),
('Team Rocket', 82, 1),
('Gym Heroes', 132, 0),
('Gym Challenge', 132, 0),

-- Neo Series (2000-2002)
('Neo Genesis', 111, 0),
('Neo Discovery', 75, 2),
('Neo Revelation', 64, 2),
('Neo Destiny', 105, 8),

-- Legendary Collection & e-Card Series (2002-2003)
('Legendary Collection', 110, 0),
('Expedition Base Set', 165, 0),
('Aquapolis', 147, 3),
('Skyridge', 144, 6),

-- EX Series (2003-2007)
('EX Ruby & Sapphire', 109, 0),
('EX Sandstorm', 100, 0),
('EX Dragon', 97, 3),
('EX Team Magma vs Team Aqua', 95, 2),
('EX Hidden Legends', 101, 1),
('EX FireRed & LeafGreen', 112, 4),
('EX Team Rocket Returns', 109, 2),
('EX Deoxys', 107, 1),
('EX Emerald', 106, 1),
('EX Unseen Forces', 115, 2),
('EX Delta Species', 113, 1),
('EX Legend Maker', 92, 1),
('EX Holon Phantoms', 110, 1),
('EX Crystal Guardians', 100, 0),
('EX Dragon Frontiers', 101, 0),
('EX Power Keepers', 108, 0),

-- Diamond & Pearl Series (2007-2009)
('Diamond & Pearl', 130, 0),
('Mysterious Treasures', 123, 1),
('Secret Wonders', 132, 0),
('Great Encounters', 106, 0),
('Majestic Dawn', 100, 0),
('Legends Awakened', 146, 0),
('Stormfront', 100, 3),

-- Platinum Series (2009)
('Platinum', 127, 3),
('Rising Rivals', 111, 3),
('Supreme Victors', 147, 3),
('Platinum—Arceus', 99, 0),

-- HeartGold & SoulSilver Series (2010-2011)
('HeartGold & SoulSilver', 123, 0),
('HS—Unleashed', 95, 0),
('HS—Undaunted', 90, 0),
('HS—Triumphant', 102, 0),

-- Call of Legends Series (2011)
('Call of Legends', 95, 0),

-- Black & White Series (2011-2013)
('Black & White', 114, 1),
('Emerging Powers', 98, 0),
('Noble Victories', 101, 1),
('Next Destinies', 99, 4),
('Dark Explorers', 108, 3),
('Dragons Exalted', 124, 4),
('Dragon Vault', 20, 1),
('Boundaries Crossed', 149, 4),
('Plasma Storm', 135, 3),
('Plasma Freeze', 116, 6),
('Plasma Blast', 101, 4),
('Legendary Treasures', 113, 2),

-- XY Series (2014-2016)
('Kalos Starter Set', 39, 0),
('XY', 146, 0),
('Flashfire', 106, 3),
('Furious Fists', 111, 2),
('Phantom Forces', 119, 3),
('Primal Clash', 160, 4),
('Double Crisis', 34, 0),
('Roaring Skies', 108, 2),
('Ancient Origins', 98, 2),
('BREAKthrough', 162, 2),
('BREAKpoint', 122, 1),
('Generations', 83, 0),
('Fates Collide', 124, 1),
('Steam Siege', 114, 2),
('Evolutions', 108, 5),

-- Sun & Moon Series (2017-2019)
('Sun & Moon', 149, 14),
('Guardians Rising', 145, 24),
('Burning Shadows', 147, 22),
('Shining Legends', 73, 5),
('Crimson Invasion', 111, 13),
('Ultra Prism', 156, 17),
('Forbidden Light', 131, 15),
('Celestial Storm', 168, 15),
('Dragon Majesty', 70, 8),
('Lost Thunder', 214, 22),
('Team Up', 181, 15),
('Detective Pikachu', 18, 0),
('Unbroken Bonds', 214, 20),
('Unified Minds', 236, 22),
('Hidden Fates', 68, 1),
('Cosmic Eclipse', 236, 35),

-- Sword & Shield Series (2020-2023)
('Sword & Shield', 202, 14),
('Rebel Clash', 192, 17),
('Darkness Ablaze', 189, 12),
('Champion''s Path', 73, 7),
('Vivid Voltage', 185, 18),
('Shining Fates', 72, 1),
('Battle Styles', 163, 20),
('Chilling Reign', 198, 35),
('Evolving Skies', 203, 34),
('Celebrations', 25, 0),
('Fusion Strike', 264, 20),
('Brilliant Stars', 172, 14),
('Astral Radiance', 189, 27),
('Pokémon GO', 78, 10),
('Lost Origin', 196, 21),
('Silver Tempest', 195, 20),
('Crown Zenith', 159, 1),

-- Scarlet & Violet Series (2023-2025)
('Scarlet & Violet', 198, 60),
('Paldea Evolved', 193, 86),
('Obsidian Flames', 197, 33),
('151', 165, 42),
('Paradox Rift', 182, 84),
('Paldean Fates', 91, 154),
('Temporal Forces', 162, 56),
('Twilight Masquerade', 167, 59),
('Shrouded Fable', 64, 35),
('Stellar Crown', 142, 33),
('Surging Sparks', 191, 61),
('Prismatic Evolutions', 131, 49),
('Journey Together', 159, 31),
('Destined Rivals', 182, 62),
('Black Bolt', 86, 86),
('White Flare', 86, 87),

-- Mega Evolution Series (2025-)
('Mega Evolution', 132, 56),
('Phantasmal Flames', 94, 26);
