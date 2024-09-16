// use futures::Stream;
// use telebot::functions::*;
// use telebot::Bot;

// async fn init_bot() {
//     let mut bot = Bot::new("7527659996:AAGVKlDLi13Ml2cd-91oBOPFNJcElh2HI7Y").update_interval(200);

//     let handle = bot
//         .new_cmd("/start")
//         .then(|(bot, msg)| bot.message(msg.chat.id, "hello, im using telebot").send())
//         .for_each(|_| Ok(()));

//     bot.run_with(handle);
// }
