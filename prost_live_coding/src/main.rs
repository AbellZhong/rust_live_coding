use prost::Message;

use crate::pb::tutorial::Person;

mod pb;

fn main() {
    let person = pb::tutorial::Person::default();

    let vec1 = person.encode_to_vec();
    let vec2 = person.encode_length_delimited_to_vec();

    let person2 = pb::tutorial::Person::new(
        "zhongshenchao",
        1,
        "zhongshenchao@foxmail.com",
        vec![pb::tutorial::person::PhoneNumber::new(
            "17681524881",
            pb::tutorial::person::PhoneType::Mobile,
        )],
    );

    let vec3 = person2.encode_to_vec();
    let vec4 = person2.encode_length_delimited_to_vec();

    println!("{person:?}");
    println!("{vec1:?}");
    println!("{vec2:?}");


    println!("{person2:?}");
    println!("{vec3:?}");
    println!("{vec4:?}");

    // 比对proto解码后的数据和编码前的数据是否一致
    let person3 = Person::decode(vec3.as_ref()).unwrap();

    assert_eq!(person2, person3);

    // 现在要将decode 出来的protobuf 转换成 json

    let transform_json_data = serde_json::to_string_pretty(&person3).unwrap();
    
    println!("transform data{}", transform_json_data);
}
