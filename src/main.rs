extern crate rmp_serde as serde_msgpack;
extern crate serenity;
extern crate chrono;
extern crate serde_json;

use chrono::DateTime;

use serenity::model::channel::{Message, MessageType};
use serenity::model::user::User;

fn main() {
  let msg = make_message();
  let ser = serde_msgpack::to_vec(&msg).unwrap();
  println!("ser ({} bytes): {:?}", ser.len(), ser);
  let de: Message = serde_msgpack::from_slice(&ser).unwrap();
  println!("de: {:#?}", de);
}

fn make_message() -> Message {
  Message {
    id: 398588712713322500.into(),
    attachments: Default::default(),
    author: User {
      id: 121031174020661250.into(),
      avatar: Some("a_c5edc71aff7e3c59ad16172d17f15f89".into()),
      bot: false,
      discriminator: 1234,
      name: "jkcclemens".into()
    },
    channel_id: 307359970096185340.into(),
    content: "**Welcome to the lala world Discord!**\n\nFollow these steps to be granted full access to the server.\n\n1.  In this channel, use the `!autotag` command to tag yourself. For example, to tag myself as Duvicauroix Priorfaix on Adamantoise, I would send `!autotag Adamantoise Duvicauroix Priorfaix`\n\n2. After tagging yourself, go to <#398353832741961749> and scroll all the way to the very top of the channel. Read the rules there and do the action they tell you to do.\n\n3. Come chat with us in <#307367069777788928>!\n\nIf you're *really* struggling, look here: https://lalafell.world/tutorial/".into(),
    edited_timestamp: DateTime::parse_from_rfc3339("2018-02-04T16:30:13.852+00:00").ok(),
    embeds: Default::default(),
    kind: MessageType::Regular,
    mention_everyone: false,
    mention_roles: Default::default(),
    mentions: Default::default(),
    nonce: serde_json::Value::Null,
    pinned: false,
    reactions: Default::default(),
    timestamp: DateTime::parse_from_rfc3339("2018-01-04T21:29:14.531+00:00").unwrap(),
    tts: false,
    webhook_id: None,
  }
}
