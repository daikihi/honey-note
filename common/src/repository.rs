// Repository is a gateway for data accessing objects.
// you can swiitch implementations of data accessing objects.
// now (2025-06-08), we assume that there is only access to sqlite database.
pub mod flowers;
pub mod prefectures;

// Repository 用のException をつくりたい.
