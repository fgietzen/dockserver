use crate::docker_api::ImageMetaData;
use crate::notifications::Notifier;

pub struct TelegramNotifier {
	bot_id: String,
	chat_id: String
}

impl TelegramNotifier {
	pub fn new<S: Into<String>>(bot_id: S, chat_id: S) -> TelegramNotifier {
		return TelegramNotifier {
			bot_id: bot_id.into(),
			chat_id: chat_id.into()
		}
	}

	fn image_to_string(image: &ImageMetaData) -> String {
		return format!("Update available for {}:{}", image.name(), image.tag());
	}
}

impl Notifier for TelegramNotifier {
	fn notify(&mut self, images: &Vec<ImageMetaData>) {
		if images.is_empty() {
			return;
		}

		let text = images.iter()
			.map(|im| TelegramNotifier::image_to_string(im))
			.fold(String::new(), |r, l| r + &l + "\n");
		let res = ureq::post(&format!("https://api.telegram.org/{}/sendMessage", self.bot_id))
			.query("chat_id", &self.chat_id)
			.query("text", &text)
			.call();

		if let Err(e) = res {
			eprintln!("Could not send telegram message! ({})", e);
		}
	}
}
