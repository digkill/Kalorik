#[derive(Clone)]
pub struct Messages {
    pub welcome: String,
    pub help: String,
    pub help_detailed: String,
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
                help_detailed: r#"
📋 *Nutrition Bot Help* 📋
Welcome to the Nutrition Bot\! Track your meals and monitor your nutrition with ease\.

*Commands:*
• `/start` Register and choose your language\.
• `/help` Show this help message\.
• `/stats` View today’s nutrition summary \(calories, protein, fats, carbs\)\.
• `/reset` Clear today’s food logs\.
• `/week` See a chart of your weekly calorie intake\.
• `/subscribe` Subscribe for full access \(299 RUB\/month\)\.
• `/status` Check your subscription status\.
• `/cancel` Learn how to cancel your subscription\.

*How to Log Food:*
• *Text*: Send a message like "I ate 100g chicken and rice" to log a meal\.
• *Photo*: Send a food photo to analyze its contents\.
• *Voice*: Send a voice message describing your meal\.

💡 *Tip*: Use `/start` to change your language at any time\.
"#.to_string(),
                unknown: "I didn't understand.".into(),
                reset_done: "🔁 Your logs have been reset.".into(),
                week_empty: "No data for the last 7 days.".into(),
                graph_error: "❌ Error drawing the chart.".into(),
                error: "❌ An error occurred.".into(),
            },
            "ru" => Messages {
                welcome: "Добро пожаловать!".into(),
                help: "Этот бот помогает считать калории.".into(),
                help_detailed: r#"
📋 *Помощь по боту питания* 📋
Добро пожаловать в бот питания\! Отслеживайте свои приемы пищи и питательные вещества с легкостью\.

*Команды:*
• `/start` Зарегистрируйтесь и выберите язык\.
• `/help` Показать это сообщение с помощью\.
• `/stats` Посмотреть сводку питания за сегодня \(калории, белки, жиры, углеводы\)\.
• `/reset` Очистить логи еды за сегодня\.
• `/week` Посмотреть график калорий за неделю\.
• `/subscribe` Оформить подписку для полного доступа \(299 ₽\/мес\)\.
• `/status` Проверить статус подписки\.
• `/cancel` Узнать, как отменить подписку\.

*Как записывать еду:*
• *Текст*: Отправьте сообщение, например, "Я съел 100г курицы и риса"\.
• *Фото*: Отправьте фото еды для анализа\.
• *Голос*: Отправьте голосовое сообщение с описанием еды\.

💡 *Совет*: Используйте `/start`, чтобы сменить язык в любое время\.
"#.to_string(),
                unknown: "Я не понял команду.".into(),
                reset_done: "🔁 Данные за сегодня сброшены.".into(),
                week_empty: "Нет данных за последние 7 дней.".into(),
                graph_error: "❌ Ошибка при построении графика.".into(),
                error: "❌ Произошла ошибка.".into(),
            },
            "th" => Messages {
                welcome: "ยินดีต้อนรับสู่บอทคำนวณแคลอรี่ของคุณ!".into(),
                help: "บอทนี้ช่วยคุณติดตามแคลอรี่และสารอาหารรายวัน.".into(),
                help_detailed: r#"
📋 *ความช่วยเหลือของบอทโภชนาการ* 📋
ยินดีต้อนรับสู่บอทโภชนาการ\! ติดตามมื้ออาหารและสารอาหารของคุณได้อย่างง่ายดาย

*คำสั่ง:*
• `/start` ลงทะเบียนและเลือกภาษา
• `/help` แสดงข้อความช่วยเหลือนี้
• `/stats` ดูสรุปโภชนาการของวันนี้ \(แคลอรี่, โปรตีน, ไขมัน, คาร์โบไฮเดรต\)
• `/reset` ล้างบันทึกอาหารของวันนี้
• `/week` ดูกราฟแคลอรี่รายสัปดาห์
• `/subscribe` สมัครสมาชิกเพื่อใช้งานเต็มรูปแบบ \(299 รูเบิล\/เดือน\)
• `/status` ตรวจสอบสถานะการสมัครสมาชิก
• `/cancel` เรียนรู้วิธียกเลิกการสมัครสมาชิก

*วิธีบันทึกอาหาร:*
• *ข้อความ*: ส่งข้อความ เช่น "ฉันกินไก่ 100 กรัมและข้าว"
• *รูปภาพ*: ส่งรูปภาพอาหารเพื่อวิเคราะห์
• *เสียง*: ส่งข้อความเสียงที่อธิบายมื้ออาหาร

💡 *เคล็ดลับ*: ใช้ `/start` เพื่อเปลี่ยนภาษาได้ตลอดเวลา
"#.to_string(),
                unknown: "ขออภัย ฉันไม่เข้าใจคำสั่งนั้น.".into(),
                reset_done: "🔁 รีเซ็ตข้อมูลของวันนี้เรียบร้อยแล้ว.".into(),
                week_empty: "ไม่มีข้อมูลในช่วง 7 วันที่ผ่านมา.".into(),
                graph_error: "❌ เกิดข้อผิดพลาดในการสร้างกราฟ.".into(),
                error: "❌ เกิดข้อผิดพลาด.".into(),
            },
            "zh" => Messages {
                welcome: "欢迎使用您的卡路里助手！".into(),
                help: "这个机器人可以帮助您追踪每日摄入的卡路里和营养成分。".into(),
                help_detailed: r#"
📋 *营养机器人帮助* 📋
欢迎使用营养机器人\! 轻松跟踪您的饮食和营养\.

*命令:*
• `/start` 注册并选择语言\.
• `/help` 显示此帮助信息\.
• `/stats` 查看今日营养总结\(卡路里、蛋白质、脂肪、碳水化合物\)\.
• `/reset` 清除今日的饮食记录\.
• `/week` 查看每周卡路里摄入图表\.
• `/subscribe` 订阅以获得完整功能\(299卢布\/月\)\.
• `/status` 检查订阅状态\.
• `/cancel` 了解如何取消订阅\.

*如何记录食物:*
• *文本*: 发送消息，如“我吃了100克鸡肉和米饭”\.
• *图片*: 发送食物照片进行分析\.
• *语音*: 发送描述食物的语音消息\.

💡 *提示*: 随时使用 `/start` 更改语言\.
"#.to_string(),
                unknown: "对不起，我不明白这条消息。".into(),
                reset_done: "🔁 今天的数据已被重置。".into(),
                week_empty: "过去 7 天没有记录。".into(),
                graph_error: "❌ 绘图时出错。".into(),
                error: "❌ 发生错误。".into(),
            },
            _ => Messages {
                welcome: "Welcome!".into(),
                help: "This bot helps track calories.".into(),
                help_detailed: r#"
📋 *Nutrition Bot Help* 📋
Welcome to the Nutrition Bot\! Track your meals and monitor your nutrition with ease\.

*Commands:*
• `/start` Register and choose your language\.
• `/help` Show this help message\.
• `/stats` View today’s nutrition summary \(calories, protein, fats, carbs\)\.
• `/reset` Clear today’s food logs\.
• `/week` See a chart of your weekly calorie intake\.
• `/subscribe` Subscribe for full access \(299 RUB\/month\)\.
• `/status` Check your subscription status\.
• `/cancel` Learn how to cancel your subscription\.

*How to Log Food:*
• *Text*: Send a message like "I ate 100g chicken and rice" to log a meal\.
• *Photo*: Send a food photo to analyze its contents\.
• *Voice*: Send a voice message describing your meal\.

💡 *Tip*: Use `/start` to change your language at any time\.
"#.to_string(),
                unknown: "I didn't understand.".into(),
                reset_done: "🔁 Logs reset.".into(),
                week_empty: "No data for last week.".into(),
                graph_error: "❌ Chart error.".into(),
                error: "❌ An error occurred.".into(),
            },
        }
    }
}