CREATE TABLE storage (
  `storage_id` int(11) NOT NULL AUTO_INCREMENT,
  `storage_hash` varchar(256) NOT NULL,
  `storage_path` varchar(4096) NOT NULL,
  PRIMARY KEY (`storage_id`),
  UNIQUE KEY `storage_id_UNIQUE` (`storage_id`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4;
