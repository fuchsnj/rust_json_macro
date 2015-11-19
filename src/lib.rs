extern crate rustc_serialize;

macro_rules! json {
  (null) => (json::Null);
  ([ $($values:tt),* ]) => (json::List(vec![ $(json!($values)),* ]));
  ({ $($keys:expr => $values:tt),* }) => ({
    let kv_pairs = vec![ $(($keys.to_string(), json!($values))),* ];
    ::rustc_serialize::json::Json::Object(kv_pairs.into_iter().collect())
  });
  ($ex:expr) => ({
  	  use ::rustc_serialize::json::ToJson;
	  $ex.to_json()
  });
}

#[test]
fn it_works() {
	json!({
		"test" => "test",
		"test2" => true
	});
}