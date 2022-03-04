#[derive(Serialize, Deserialize)]
pub struct Employee {
    eid: Option<i32>,
    name: String,
    department: Option<String>
  
}