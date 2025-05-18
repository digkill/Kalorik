#[derive(Clone)]
pub struct Messages {
    pub welcome: String,
    pub help: String,
    pub unknown: String,
    pub reset_done: String,
    pub week_empty: String,
    pub graph_error: String,
    pub error: String,
}

impl Messages {
    pub fn get(lang: &str) -> Self {
        match lang {
            "en" => Messages {
                welcome: "Welcome!".into(),
                help: "This bot helps track calories.".into(),
                unknown: "I didn't understand.".into(),
                reset_done: "🔁 Your logs have been reset.".into(),
                week_empty: "No data for the last 7 days.".into(),
                graph_error: "❌ Error drawing the chart.".into(),
                error: "❌ An error occurred.".into(),
            },
            "ru" => Messages {
                welcome: "Добро пожаловать!".into(),
                help: "Этот бот помогает считать калории.".into(),
                unknown: "Я не понял команду.".into(),
                reset_done: "🔁 Данные за сегодня сброшены.".into(),
                week_empty: "Нет данных за последние 7 дней.".into(),
                graph_error: "❌ Ошибка при построении графика.".into(),
                error: "❌ Произошла ошибка.".into(),
            },
            "th" => Messages {
                welcome: "ยินดีต้อนรับสู่บอทคำนวณแคลอรี่ของคุณ!".into(),
                help: "บอทนี้ช่วยคุณติดตามแคลอรี่และสารอาหารรายวัน.".into(),
                unknown: "ขออภัย ฉันไม่เข้าใจคำสั่งนั้น.".into(),
                reset_done: "🔁 รีเซ็ตข้อมูลของวันนี้เรียบร้อยแล้ว.".into(),
                week_empty: "ไม่มีข้อมูลในช่วง 7 วันที่ผ่านมา.".into(),
                graph_error: "❌ เกิดข้อผิดพลาดในการสร้างกราฟ.".into(),
                error: "❌ เกิดข้อผิดพลาด.".into(),
            },
            "zh" => Messages {
                welcome: "欢迎使用您的卡路里助手！".into(),
                help: "这个机器人可以帮助您追踪每日摄入的卡路里和营养成分。".into(),
                unknown: "对不起，我不明白这条消息。".into(),
                reset_done: "🔁 今天的数据已被重置。".into(),
                week_empty: "过去 7 天没有记录。".into(),
                graph_error: "❌ 绘图时出错。".into(),
                error: "❌ 发生错误。".into(),
            },
            _ => Messages {
                welcome: "Welcome!".into(),
                help: "This bot helps track calories.".into(),
                unknown: "I didn't understand.".into(),
                reset_done: "🔁 Logs reset.".into(),
                week_empty: "No data for last week.".into(),
                graph_error: "❌ Chart error.".into(),
                error: "❌ An error occurred.".into(),
            },
        }
    }
}