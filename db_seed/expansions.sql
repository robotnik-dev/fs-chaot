-- Pokemon TCG Expansions Seed Data
-- This file contains all English Pokemon TCG expansion sets from 1999-2025
-- Format: (name, abbreviation, cards, secret_cards)

INSERT OR IGNORE INTO expansions (name, abbreviation, cards, secret_cards) VALUES

-- Base Series (1999-2000)
('Base Set', 'BS', 102, 0),
('Jungle', 'JUN', 64, 0),
('Fossil', 'FOS', 62, 0),
('Base Set 2', 'BS2', 130, 0),
('Team Rocket', 'TR', 82, 1),
('Gym Heroes', 'GYH', 132, 0),
('Gym Challenge', 'GYC', 132, 0),

-- Neo Series (2000-2002)
('Neo Genesis', 'NEO1', 111, 0),
('Neo Discovery', 'NEO2', 75, 2),
('Neo Revelation', 'NEO3', 64, 2),
('Neo Destiny', 'NEO4', 105, 8),

-- Legendary Collection & e-Card Series (2002-2003)
('Legendary Collection', 'LC', 110, 0),
('Expedition Base Set', 'EX', 165, 0),
('Aquapolis', 'AQ', 147, 3),
('Skyridge', 'SK', 144, 6),

-- EX Series (2003-2007)
('EX Ruby & Sapphire', 'RS', 109, 0),
('EX Sandstorm', 'SS', 100, 0),
('EX Dragon', 'DR', 97, 3),
('EX Team Magma vs Team Aqua', 'MA', 95, 2),
('EX Hidden Legends', 'HL', 101, 1),
('EX FireRed & LeafGreen', 'FRLG', 112, 4),
('EX Team Rocket Returns', 'TRR', 109, 2),
('EX Deoxys', 'DX', 107, 1),
('EX Emerald', 'EM', 106, 1),
('EX Unseen Forces', 'UF', 115, 2),
('EX Delta Species', 'DS', 113, 1),
('EX Legend Maker', 'LM', 92, 1),
('EX Holon Phantoms', 'HP', 110, 1),
('EX Crystal Guardians', 'CG', 100, 0),
('EX Dragon Frontiers', 'DF', 101, 0),
('EX Power Keepers', 'PK', 108, 0),

-- Diamond & Pearl Series (2007-2009)
('Diamond & Pearl', 'DP', 130, 0),
('Mysterious Treasures', 'MT', 123, 1),
('Secret Wonders', 'SW', 132, 0),
('Great Encounters', 'GE', 106, 0),
('Majestic Dawn', 'MD', 100, 0),
('Legends Awakened', 'LA', 146, 0),
('Stormfront', 'SF', 100, 3),

-- Platinum Series (2009)
('Platinum', 'PL', 127, 3),
('Rising Rivals', 'RR', 111, 3),
('Supreme Victors', 'SV', 147, 3),
('Platinum—Arceus', 'AR', 99, 0),

-- HeartGold & SoulSilver Series (2010-2011)
('HeartGold & SoulSilver', 'HGSS', 123, 0),
('HS—Unleashed', 'UL', 95, 0),
('HS—Undaunted', 'UD', 90, 0),
('HS—Triumphant', 'TM', 102, 0),

-- Call of Legends Series (2011)
('Call of Legends', 'CL', 95, 0),

-- Black & White Series (2011-2013)
('Black & White', 'BLW', 114, 1),
('Emerging Powers', 'EPO', 98, 0),
('Noble Victories', 'NVI', 101, 1),
('Next Destinies', 'NXD', 99, 4),
('Dark Explorers', 'DEX', 108, 3),
('Dragons Exalted', 'DRX', 124, 4),
('Dragon Vault', 'DRV', 20, 1),
('Boundaries Crossed', 'BCR', 149, 4),
('Plasma Storm', 'PLS', 135, 3),
('Plasma Freeze', 'PLF', 116, 6),
('Plasma Blast', 'PLB', 101, 4),
('Legendary Treasures', 'LTR', 113, 2),

-- XY Series (2014-2016)
('Kalos Starter Set', 'KSS', 39, 0),
('XY', 'XY', 146, 0),
('Flashfire', 'FLF', 106, 3),
('Furious Fists', 'FFI', 111, 2),
('Phantom Forces', 'PHF', 119, 3),
('Primal Clash', 'PRC', 160, 4),
('Double Crisis', 'DCR', 34, 0),
('Roaring Skies', 'ROS', 108, 2),
('Ancient Origins', 'AOR', 98, 2),
('BREAKthrough', 'BKT', 162, 2),
('BREAKpoint', 'BKP', 122, 1),
('Generations', 'GEN', 83, 0),
('Fates Collide', 'FCO', 124, 1),
('Steam Siege', 'STS', 114, 2),
('Evolutions', 'EVO', 108, 5),

-- Sun & Moon Series (2017-2019)
('Sun & Moon', 'SM', 149, 14),
('Guardians Rising', 'GRI', 145, 24),
('Burning Shadows', 'BUS', 147, 22),
('Shining Legends', 'SLG', 73, 5),
('Crimson Invasion', 'CIN', 111, 13),
('Ultra Prism', 'UPR', 156, 17),
('Forbidden Light', 'FLI', 131, 15),
('Celestial Storm', 'CES', 168, 15),
('Dragon Majesty', 'DRM', 70, 8),
('Lost Thunder', 'LOT', 214, 22),
('Team Up', 'TEU', 181, 15),
('Detective Pikachu', 'DET', 18, 0),
('Unbroken Bonds', 'UNB', 214, 20),
('Unified Minds', 'UNM', 236, 22),
('Hidden Fates', 'HIF', 68, 1),
('Cosmic Eclipse', 'CEC', 236, 35),

-- Sword & Shield Series (2020-2023)
('Sword & Shield', 'SSH', 202, 14),
('Rebel Clash', 'RCL', 192, 17),
('Darkness Ablaze', 'DAA', 189, 12),
('Champion''s Path', 'CPA', 73, 7),
('Vivid Voltage', 'VIV', 185, 18),
('Shining Fates', 'SHF', 72, 1),
('Battle Styles', 'BST', 163, 20),
('Chilling Reign', 'CRE', 198, 35),
('Evolving Skies', 'EVS', 203, 34),
('Celebrations', 'CEL', 25, 0),
('Fusion Strike', 'FST', 264, 20),
('Brilliant Stars', 'BRS', 172, 14),
('Astral Radiance', 'ASR', 189, 27),
('Pokémon GO', 'PGO', 78, 10),
('Lost Origin', 'LOR', 196, 21),
('Silver Tempest', 'SIT', 195, 20),
('Crown Zenith', 'CRZ', 159, 1),

-- Scarlet & Violet Series (2023-2025)
('Scarlet & Violet', 'SVI', 198, 60),
('Paldea Evolved', 'PAL', 193, 86),
('Obsidian Flames', 'OBF', 197, 33),
('151', 'MEW', 165, 42),
('Paradox Rift', 'PAR', 182, 84),
('Paldean Fates', 'PAF', 91, 154),
('Temporal Forces', 'TEF', 162, 56),
('Twilight Masquerade', 'TWM', 167, 59),
('Shrouded Fable', 'SFA', 64, 35),
('Stellar Crown', 'SCR', 142, 33),
('Surging Sparks', 'SSP', 191, 61),
('Prismatic Evolutions', 'PRE', 131, 49),
('Journey Together', 'JTO', 159, 31),
('Destined Rivals', 'DRV2', 182, 62),
('Black Bolt', 'BLB', 86, 86),
('White Flare', 'WHF', 86, 87),

-- Mega Evolution Series (2025-)
('Mega Evolution', 'MEV', 132, 56),
('Phantasmal Flames', 'PHF2', 94, 26);
