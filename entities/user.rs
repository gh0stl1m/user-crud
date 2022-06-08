use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

type Latitude = f32;
type Longitude = f32;
pub struct Location(Latitude, Longitude);

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub username: String,
	pub email : String,
	pub location: Location
}
